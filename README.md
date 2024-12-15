# PromptPayUtils

`PromptPayUtils` is a utility module for generating **PromptPay Payloads** compliant with Thailand's **PromptPay** system. It can be used to transfer money or make payments via registered phone numbers or national IDs in the PromptPay system.

## How it Works

`PromptPayUtils` operates as follows:

1. **Phone Number and National ID Sanitization**:
   - The `sanitize_phone_number` function cleans the phone number by removing unnecessary characters (e.g., `-`, `+`) and ensures the phone number is exactly 9 digits long.
   - The `sanitize_national_id` function processes national IDs, removes hyphens, and ensures the ID is in the correct 13-digit format.

2. **Amount Conversion to Satangs**:
   - The amount (in Baht) is converted to satangs (1 Baht = 100 satangs), rounding the value to two decimal places.

3. **Payload Creation**:
   - The `generate_payload` function combines the sanitized phone number or national ID with the formatted amount to create a **PromptPay Payload**. The payload also includes a CRC checksum to ensure data integrity.

4. **CRC-16 (XMODEM) Calculation**:
   - The `calculate_precise_crc` function computes the CRC-16 checksum using the XMODEM algorithm to protect the payload from tampering.

## Installation

Add `PromptPayUtils` to your Rust project by copying this code into the desired file or by including it as a dependency if it's developed into a library.

## Example Usage

```rust
use promptpay_utils::{InputType, PromptPayUtils};

fn main() {
    let phone_number = "+66-812345678".to_string();
    let national_id = "1234567890123".to_string();
    let amount = 123.45;

    // Using phone number as input
    match PromptPayUtils::generate_payload(InputType::PhoneNumber(phone_number), amount) {
        Ok(payload) => println!("Payload (Phone): {}", payload),
        Err(err) => eprintln!("Error: {}", err),
    }

    // Using national ID as input
    match PromptPayUtils::generate_payload(InputType::NationalID(national_id), amount) {
        Ok(payload) => println!("Payload (National ID): {}", payload),
        Err(err) => eprintln!("Error: {}", err),
    }
}
