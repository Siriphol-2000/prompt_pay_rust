# PromptPayUtils

`PromptPayUtils` เป็นโมดูลเครื่องมือที่ช่วยในการสร้าง **PromptPay Payload** ที่สอดคล้องกับมาตรฐานของระบบ **PromptPay** ในประเทศไทย ซึ่งสามารถนำไปใช้ในการโอนเงินหรือชำระเงินผ่านหมายเลขโทรศัพท์มือถือที่ลงทะเบียนไว้ในระบบ PromptPay หรือหมายเลขบัตรประชาชน

## วิธีการทำงาน

`PromptPayUtils` มีการทำงานดังต่อไปนี้:

1. **การปรับรูปแบบหมายเลขโทรศัพท์และหมายเลขบัตรประชาชน**:
   - ฟังก์ชัน `sanitize_phone_number` จะช่วยลบอักขระที่ไม่จำเป็น (เช่น `-`, `+`) ออกจากหมายเลขโทรศัพท์ และตรวจสอบความถูกต้องให้หมายเลขโทรศัพท์มีความยาว 9 หลักตามมาตรฐาน
   - ฟังก์ชัน `sanitize_national_id` จะช่วยลบเครื่องหมายขีดกลางในหมายเลขบัตรประชาชน และตรวจสอบความถูกต้องให้หมายเลขบัตรประชาชนมีความยาว 13 หลักตามมาตรฐาน

2. **การแปลงจำนวนเงินเป็นสตางค์**:
   - จำนวนเงินที่ใส่เข้ามา (หน่วยบาท) จะถูกคำนวณและแปลงเป็นหน่วยสตางค์ (1 บาท = 100 สตางค์) และทำการปัดเศษทศนิยมให้เป็น 2 ตำแหน่ง

3. **การสร้าง Payload**:
   - ฟังก์ชัน `generate_payload` จะนำหมายเลขโทรศัพท์หรือหมายเลขบัตรประชาชนที่ปรับรูปแบบแล้วและจำนวนเงินมาสร้างเป็น **PromptPay Payload** ซึ่งรวมถึงการเติมค่าตรวจสอบ (CRC) เพื่อความถูกต้อง

4. **การคำนวณ CRC-16 (XMODEM)**:
   - ใช้ฟังก์ชัน `calculate_precise_crc` สำหรับคำนวณค่า CRC-16 เพื่อเพิ่มความปลอดภัยและป้องกันการเปลี่ยนแปลงของข้อมูล

## การติดตั้ง

เพิ่ม `PromptPayUtils` เข้าไปในโปรเจกต์ Rust ของคุณ โดยการคัดลอกโค้ดนี้ไปยังไฟล์ที่ต้องการ หรือใช้เครื่องมือจัดการ dependency หากคุณพัฒนาเป็นไลบรารี

## ตัวอย่างการใช้งาน

```rust
use promptpay_utils::{InputType, PromptPayUtils};

fn main() {
    let phone_number = "+66-812345678".to_string();
    let national_id = "1234567890123".to_string();
    let amount = 123.45;

    // ใช้หมายเลขโทรศัพท์เป็นข้อมูลเข้า
    match PromptPayUtils::generate_payload(InputType::PhoneNumber(phone_number), amount) {
        Ok(payload) => println!("Payload (Phone): {}", payload),
        Err(err) => eprintln!("Error: {}", err),
    }

    // ใช้หมายเลขบัตรประชาชนเป็นข้อมูลเข้า
    match PromptPayUtils::generate_payload(InputType::NationalID(national_id), amount) {
        Ok(payload) => println!("Payload (National ID): {}", payload),
        Err(err) => eprintln!("Error: {}", err),
    }
}
