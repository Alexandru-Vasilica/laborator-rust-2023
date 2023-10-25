#[derive(Debug)]
enum Error {
    CharNotAscii,
    CharNotDigit,
    CharNotBase16,
    CharNotLetter,
    CharNotPrintable,
}

fn is_ascii(c: char) -> bool {
    if c as u16 > std::u8::MAX as u16 {
        return false;
    }
    return true;
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
    if !is_ascii(c) {
        return false;
    }
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

fn to_digit(c: char) -> u8 {
    return c as u8 - '0' as u8;
}

fn to_digit_hex(c: char) -> u8 {
    if is_digit(c) {
        return to_digit(c);
    }
    if c as u8 >= 'a' as u8 && c as u8 <= 'f' as u8 {
        return (c as u8 - 'a' as u8) + 10;
    }
    return (c as u8 - 'A' as u8) + 10;
}

fn to_uppercase(mut c: char) -> Result<char, Error> {
    if !is_letter(c) {
        return Err(Error::CharNotLetter);
    }
    if c as u8 >= 'a' as u8 && c as u8 <= 'f' as u8 {
        c = (c as u8 - 'a' as u8 + 'A' as u8) as char;
    }
    Ok(c)
}

fn to_lowercase(mut c: char) -> Result<char, Error> {
    if !is_letter(c) {
        return Err(Error::CharNotLetter);
    }
    if c as u8 >= 'A' as u8 && c as u8 <= 'F' as u8 {
        c = (c as u8 - 'A' as u8 + 'a' as u8) as char;
    }
    Ok(c)
}

fn char_to_number(c: char) -> Result<u8, Error> {
    if !is_ascii(c) {
        return Err(Error::CharNotAscii);
    }
    if !is_digit(c) {
        return Err(Error::CharNotDigit);
    }
    let digit: u8 = to_digit(c);
    Ok(digit)
}

fn char_to_number_hex(c: char) -> Result<u8, Error> {
    if !is_ascii(c) {
        return Err(Error::CharNotAscii);
    }
    if !is_digit_hex(c) {
        return Err(Error::CharNotBase16);
    }
    let digit: u8 = to_digit_hex(c);
    Ok(digit)
}

fn print_char(c: char) -> Result<(), Error> {
    if !is_printable(c) {
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
    match to_uppercase('5') {
        Ok(x) => println!("uppercase value: {}", x),
        Err(e) => print_error(e),
    }
    match to_uppercase('a') {
        Ok(x) => println!("uppercase value: {}", x),
        Err(e) => print_error(e),
    }
    match to_lowercase('2') {
        Ok(x) => println!("lowercase value: {}", x),
        Err(e) => print_error(e),
    }
    match to_lowercase('D') {
        Ok(x) => println!("lowercase value: {}", x),
        Err(e) => print_error(e),
    }
    match char_to_number('❤') {
        Ok(x) => println!("digit value: {}", x),
        Err(e) => print_error(e),
    }
    match char_to_number('9') {
        Ok(x) => println!("digit value: {}", x),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('p') {
        Ok(x) => println!("hex digit value: {}", x),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('d') {
        Ok(x) => println!("hex digit value: {}", x),
        Err(e) => print_error(e),
    }
    match print_char('❤') {
        Ok(()) => (),
        Err(e) => print_error(e),
    }
    match print_char('p') {
        Ok(()) => (),
        Err(e) => print_error(e),
    }
}
