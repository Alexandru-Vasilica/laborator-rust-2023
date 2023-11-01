use thiserror::Error;
#[derive(Error, Debug)]
enum Error {
    #[error("All characters must be Ascii character")]
    CharNotAscii(),
}

fn encrypt(s: &String) -> Result<String, Error> {
    let mut result_str: String = String::new();
    for mut c in s.chars() {
        if !c.is_ascii() {
            return Err(Error::CharNotAscii());
        }
        if c.is_ascii_lowercase() {
            c = ('a' as u8 + ((c as u8 - 'a' as u8) + 13) % 26) as char;
        } else if c.is_ascii_uppercase() {
            c = ('A' as u8 + ((c as u8 - 'A' as u8) + 13) % 26) as char;
        }
        result_str.push(c);
    }
    Ok(result_str)
}

fn main() {
    let s1 = String::from("ywz");
    let s2 = String::from("YaZ");
    let s3 = String::from("A🐘Z");
    match encrypt(&s1) {
        Ok(result) => println!("The encrypted string: {result}"),
        Err(e) => println!("Error: {e}"),
    }
    match encrypt(&s2) {
        Ok(result) => println!("The encrypted string: {result}"),
        Err(e) => println!("Error: {e}"),
    }
    match encrypt(&s3) {
        Ok(result) => println!("The encrypted string: {result}"),
        Err(e) => println!("Error: {e}"),
    }
}
