fn add_chars_n(mut s: String, c: char, n: u8) -> String {
    let mut i: u8 = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
    return s;
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
