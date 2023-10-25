#[derive(Debug)]
enum Error {
    CharNotAscii,
    CharNotDigit,
    CharNotBase16,
    CharNotLetter,
    CharNotPrintable,
}

fn is_letter(c: char) -> bool {
    if c as u8 >= 'a' as u8 && c as u8 <= 'z' as u8 {
        return true;
    }
    if c as u8 >= 'A' as u8 && c as u8 <= 'Z' as u8 {
        return true;
    }
    return false;
}

fn is_digit(c: char) -> bool {
    if c as u8 >= '0' as u8 && c as u8 <= '9' as u8 {
        return true;
    }
    return false;
}

fn is_digit_hex(c: char) -> bool {
    if c as u8 >= '0' as u8 && c as u8 <= '9' as u8 {
        return true;
    }
    if c as u8 >= 'a' as u8 && c as u8 <= 'f' as u8 {
        return true;
    }
    if c as u8 >= 'A' as u8 && c as u8 <= 'F' as u8 {
        return true;
    }
    return false;
}

fn is_printable(c: char) -> bool {
    if is_letter(c) {
        return true;
    }
    if is_digit(c) {
        return true;
    }
    if c as u8 == ' ' as u8
        || c as u8 == '.' as u8
        || c as u8 == ',' as u8
        || c as u8 == ';' as u8
        || c as u8 == '>' as u8
        || c as u8 == '<' as u8
    {
        return true;
    }
    return false;
}

fn is_ascii(c: char) -> bool {
    if c as u16 > std::u8::MAX as u16 {
        return false;
    }
    return true;
}

fn to_digit(c: char) -> u8 {
    return c as u8 - '0' as u8;
}

fn to_digit_hex(c:char)->u8{
    if
}

fn to_uppercase(mut c: char) -> Result<char, Error> {
    if !c.is_ascii_alphabetic() {
        return Err(Error::CharNotLetter);
    }
    if c.is_ascii_lowercase() {
        c = c.to_ascii_uppercase();
    }
    Ok(c)
}

fn to_lowercase(mut c: char) -> Result<char, Error> {
    if !c.is_ascii_alphabetic() {
        return Err(Error::CharNotLetter);
    }
    if c.is_ascii_uppercase() {
        c = c.to_ascii_lowercase();
    }
    Ok(c)
}

fn char_to_number(c: char) -> Result<u8, Error> {
    if !c.is_ascii() {
        return Err(Error::CharNotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(Error::CharNotDigit);
    }
    let digit: u32 = c.to_digit(10).unwrap();
    Ok(digit as u8)
}

fn char_to_number_hex(c: char) -> Result<u8, Error> {
    if !c.is_ascii() {
        return Err(Error::CharNotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(Error::CharNotBase16);
    }
    let digit: u32 = c.to_digit(16).unwrap();
    Ok(digit as u8)
}

fn print_char(c: char) -> Result<(), Error> {
    if !c.is_ascii() {
        return Err(Error::CharNotPrintable);
    }
    println!("{c}");
    Ok(())
}

fn print_error(err: Error) {
    match err {
        Error::CharNotAscii => println!("Input character must be ASCII!"),
        Error::CharNotLetter => println!("Input character must be a letter!"),
        Error::CharNotDigit => println!("Input character must be digit!"),
        Error::CharNotBase16 => println!("Input character must be digit in base 16!"),
        Error::CharNotPrintable => println!("Input character must be printable!"),
    }
}

fn main() {
    // match to_uppercase('a') {
    //     Ok(x) => println!("value: {}", x),
    //     Err(e) => print_error(e),
    // }
    // match to_lowercase('B') {
    //     Ok(x) => println!("value: {}", x),
    //     Err(e) => print_error(e),
    // }
    // match char_to_number('❤') {
    //     Ok(x) => println!("value: {}", x),
    //     Err(e) => print_error(e),
    // }
    // match char_to_number_hex('F') {
    //     Ok(x) => println!("value: {}", x),
    //     Err(e) => print_error(e),
    // }
    // match print_char('F') {
    //     Ok(()) => (),
    //     Err(e) => print_error(e),
    // }
    println!("{}", is_ascii('A'));
}
