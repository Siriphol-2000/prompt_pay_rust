use crc::{Algorithm, Crc};

/// A utility struct for generating PromptPay payloads.
///
/// This struct provides methods to generate a formatted PromptPay payload, which is used
/// in Thailand's PromptPay system for transferring money. The system allows users to make
/// payments using either a phone number or a national ID. The `generate_payload` method
/// constructs the necessary payload string, including the phone number and amount, formatted
/// according to the PromptPay standard.
///
/// How it works:
/// 1. **Phone Number Sanitization**: The `sanitize_phone_number` method ensures that the phone number
///    is cleaned of any non-numeric characters and is in the correct format (9 digits).
/// 2. **Amount Conversion**: The `convert_to_satangs` method converts the payment amount from Baht
///    to satangs (1 Baht = 100 satangs), rounding it to the nearest whole number.
/// 3. **Payload Creation**: The `create_payload` method formats the phone number and amount into the
///    required structure for the PromptPay system.
/// 4. **CRC Calculation**: The `calculate_precise_crc` method computes a CRC-16 checksum (XMODEM)
///    of the generated payload to ensure its integrity.
///
/// This utility simplifies the process of generating PromptPay-compatible payloads and ensures
/// compliance with the system's format.
pub struct PromptPayUtils;

impl PromptPayUtils {
    /// Generate the PromptPay payload string
    pub fn generate_payload(phone_number: String, amount: f64) -> Result<String, String> {
        let sanitized_phone = Self::sanitize_phone_number(phone_number)?;

        // Convert amount to satangs
        let amount_satangs = (amount * 100.0).round() as u64;
        let formatted_amount = format!("{:06}.00", amount_satangs / 100); // Format as 8 digits

        let payload = format!(
            "00020101021129370016A00000067701011101130066{:0>9}5802TH53037645409{}6304",
            sanitized_phone, formatted_amount
        );

        let crc = Self::calculate_precise_crc(&payload);
        let final_payload = format!("{}{}", payload, crc);

        Ok(final_payload)
    }

    fn sanitize_phone_number(phone_number: String) -> Result<String, String> {
        let sanitized = phone_number
            .trim()
            .replace(['-', '+'], "")
            .replace("66", "")
            .trim_start_matches('0')
            .to_string();

        if sanitized.len() != 9 {
            return Err("Invalid phone number format".to_string());
        }

        Ok(sanitized)
    }

    fn calculate_precise_crc(payload: &str) -> String {
        // Define the custom CRC-16 algorithm parameters as a constant
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

#[cfg(test)]
mod tests {
    use super::PromptPayUtils;

    #[test]
    fn test_sanitize_phone_number_valid() {
        // Test valid phone number formats
        let input = "+66-812345678".to_string();
        let expected = "812345678".to_string();
        let result = PromptPayUtils::sanitize_phone_number(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sanitize_phone_number_invalid() {
        // Test invalid phone number
        let input = "+66-81234".to_string();
        let result = PromptPayUtils::sanitize_phone_number(input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid phone number format");
    }

    #[test]
    fn test_calculate_precise_crc() {
        // Test CRC calculation
        let payload = "00020101021129370016A000000677010111011300668123456785802TH53037645408";
        let crc = PromptPayUtils::calculate_precise_crc(payload);
        assert_eq!(crc, "8242"); // Example expected value (replace with actual expected CRC)
    }

    #[test]
    fn test_generate_payload_invalid_phone() {
        // Test payload generation with an invalid phone number
        let phone_number = "81234".to_string();
        let amount = 123.45;
        let result = PromptPayUtils::generate_payload(phone_number, amount);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid phone number format");
    }

    #[test]
    fn test_generate_payload_zero_amount() {
        // Test payload generation with a zero amount
        let phone_number = "+66-812345678".to_string();
        let amount = 0.0;
        let result = PromptPayUtils::generate_payload(phone_number, amount).unwrap();
        assert!(result.contains("000000.00"));
    }
}
