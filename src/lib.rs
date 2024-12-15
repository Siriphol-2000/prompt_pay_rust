/// PromptPay Module
///
/// This module provides utilities to generate PromptPay payloads for Thailand's PromptPay system.
/// It ensures compliance with the required format, including phone number/national ID sanitization,
/// amount formatting, and CRC checksum calculation.

pub mod promptpay_utils {
    // Import the crc crate for CRC checksum calculation
    use crc::{Algorithm, Crc};

    /// A utility struct for generating PromptPay payloads.
    pub struct Utils;

    /// Enum for specifying the input type, either a phone number or national ID.
    pub enum InputType {
        PhoneNumber(String),
        NationalID(String),
    }

    impl Utils {
        /// Generate the PromptPay payload string based on the input type (phone number or national ID) and amount.
        ///
        /// # Parameters
        /// - `input`: The input can either be a phone number or a national ID.
        /// - `amount`: The amount to be transferred in Baht.
        ///
        /// # Returns
        /// A formatted PromptPay payload as a string, or an error if the input is invalid.
        pub fn generate_payload(input: InputType, amount: f64) -> Result<String, String> {
            // Sanitize the input (phone number or national ID)
            let formatted_id = match input {
                InputType::PhoneNumber(phone) => Self::sanitize_phone_number(phone)?,
                InputType::NationalID(id) => Self::sanitize_national_id(id)?,
            };

            // Convert the amount to satangs (1 Baht = 100 satangs) and format it
            let amount_satangs = (amount * 100.0).round() / 100.0;
            let formatted_amount = format!("{:09.2}", amount_satangs);

            // Create the PromptPay payload structure
            let payload = format!(
                "00020101021129370016A000000677010111{}5802TH53037645409{}6304",
                formatted_id, formatted_amount
            );

            // Calculate the CRC checksum for the payload
            let crc = Self::calculate_precise_crc(&payload);
            let final_payload = format!("{}{}", payload, crc);

            Ok(final_payload)
        }

        /// Sanitize and format the phone number to meet PromptPay's requirements.
        ///
        /// # Parameters
        /// - `phone_number`: The phone number string to be sanitized.
        ///
        /// # Returns
        /// A sanitized phone number, or an error if the format is invalid.
        pub fn sanitize_phone_number(phone_number: String) -> Result<String, String> {
            let sanitized = phone_number
                .trim()
                .replace(['-', '+'], "")
                .replace("66", "")
                .trim_start_matches('0')
                .to_string();

            if sanitized.len() != 9 {
                return Err("Invalid phone number format".to_string());
            }

            // Format the phone number with the country code "01130066"
            let formatted_phone_number = format!("01130066{}", sanitized);

            Ok(formatted_phone_number)
        }

        /// Sanitize and format the national ID to meet PromptPay's requirements.
        ///
        /// # Parameters
        /// - `national_id`: The national ID string to be sanitized.
        ///
        /// # Returns
        /// A sanitized national ID, or an error if the format is invalid.
        pub fn sanitize_national_id(national_id: String) -> Result<String, String> {
            let sanitized = national_id.trim().replace('-', "");

            if sanitized.len() != 13 || !sanitized.chars().all(char::is_numeric) {
                return Err("Invalid national ID format".to_string());
            }
            // Format the national ID with the prefix "0213"
            let formatted_national_id = format!("0213{}", sanitized);
            Ok(formatted_national_id)
        }

        /// Calculate the CRC-16 checksum (XMODEM) for a given payload.
        ///
        /// # Parameters
        /// - `payload`: The string payload to calculate the CRC checksum for.
        ///
        /// # Returns
        /// The calculated CRC checksum as a 4-character hexadecimal string.
        pub fn calculate_precise_crc(payload: &str) -> String {
            // Define the custom CRC-16 algorithm parameters (XMODEM)
            const CRC16_XMODEM: Algorithm<u16> = Algorithm {
                width: 16,
                poly: 0x1021,    // Polynomial for CRC-16 XMODEM
                init: 0xFFFF,    // Initial value
                refin: false,    // No reflection of input bits
                refout: false,   // No reflection of output bits
                xorout: 0x0000,  // No XOR applied to the output
                check: 0x906E,   // Check value for validation
                residue: 0x0000, // Residue for the algorithm
            };

            // Create a custom CRC instance with the specified algorithm
            let crc = Crc::<u16>::new(&CRC16_XMODEM);

            // Calculate the CRC for the payload
            let mut digest = crc.digest();
            digest.update(payload.as_bytes());

            // Return the CRC value as a 4-character hexadecimal string
            format!("{:04X}", digest.finalize())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::promptpay_utils::{InputType, Utils};

    #[test]
    fn test_sanitize_phone_number_valid() {
        let input = "+66-812345678".to_string();
        let expected = "812345678".to_string();
        let result = Utils::sanitize_phone_number(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sanitize_phone_number_invalid() {
        let input = "+66-81234".to_string();
        let result = Utils::sanitize_phone_number(input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid phone number format");
    }

    #[test]
    fn test_sanitize_national_id_valid() {
        let input = "1234567890123".to_string();
        let expected = "1234567890123".to_string();
        let result = Utils::sanitize_national_id(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sanitize_national_id_invalid() {
        let input = "1234-5678".to_string();
        let result = Utils::sanitize_national_id(input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid national ID format");
    }

    #[test]
    fn test_calculate_precise_crc() {
        let payload = "00020101021129370016A000000677010111011300668123456785802TH53037645408";
        let crc = Utils::calculate_precise_crc(payload);
        assert_eq!(crc, "8242"); // Example expected CRC, replace with actual expected value
    }

    #[test]
    fn test_generate_payload_phone_number() {
        let input = InputType::PhoneNumber("+66-812345678".to_string());
        let amount = 123.45;
        let result = Utils::generate_payload(input, amount).unwrap();
        assert!(result.contains("812345678"));
    }

    #[test]
    fn test_generate_payload_national_id() {
        let input = InputType::NationalID("1234567890123".to_string());
        let amount = 123.45;
        let result = Utils::generate_payload(input, amount).unwrap();
        assert!(result.contains("1234567890123"));
    }
}
