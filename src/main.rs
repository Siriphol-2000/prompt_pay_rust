use prompt_pay::PromptPayUtils;
fn main(){
println!("{}",PromptPayUtils::generate_payload("+66-812345678".to_string(), 123.45).unwrap());
}