fn add_spaces(s: &mut String, n: u32) {
    let mut i: u32 = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, cat: &str) {
    s.push_str(cat);
}

fn int_to_str(mut n: i32) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut n_str: String = String::from("");
    if n < 0 {
        n_str.push('-');
        n *= -1;
    }
    let mut exp: i32 = 1;
    let mut poz: i32 = 0;
    while n >= exp {
        exp *= 10;
        poz += 1;
    }
    while exp > 1 {
        let cif: i32 = n % exp / (exp / 10);
        let cif_char: char = (cif as u8 + '0' as u8) as char;
        n_str.push(cif_char);
        if poz % 3 == 1 && poz != 1 {
            n_str.push('_');
        }
        poz -= 1;
        exp /= 10;
    }
    return n_str;
}

fn int_to_str_no_sep(mut n: i32) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut n_str: String = String::from("");
    if n < 0 {
        n_str.push('-');
        n *= -1;
    }
    let mut exp: i32 = 1;
    while n >= exp {
        exp *= 10;
    }
    while exp > 1 {
        let cif: i32 = n % exp / (exp / 10);
        let cif_char: char = (cif as u8 + '0' as u8) as char;
        n_str.push(cif_char);
        exp /= 10;
    }
    return n_str;
}

fn add_integer(s: &mut String, n: i32) {
    s.push_str(&int_to_str(n));
}

fn add_float(s: &mut String, mut n: f32) {
    if n < 0.0 {
        n = n * (-1.0);
        s.push('-');
    }
    let int: i32 = n.floor() as i32;
    let mut fr: f32 = n - n.floor();
    while fr > fr.floor() {
        fr = fr * 10.0;
    }
    s.push_str(&int_to_str_no_sep(int));
    s.push('.');
    s.push_str(&int_to_str_no_sep(fr as i32));
}

fn create_string() -> String {
    let mut s = String::from("");
    add_spaces(&mut s, 40);
    add_str(&mut s, "I");
    add_spaces(&mut s, 1);
    add_str(&mut s, "💚\n");
    add_spaces(&mut s, 40);
    add_str(&mut s, "RUST\n");
    add_spaces(&mut s, 4);
    add_str(&mut s, "Most");
    add_spaces(&mut s, 12);
    add_str(&mut s, "crate");
    add_spaces(&mut s, 6);
    add_integer(&mut s, 306_437_968);
    add_spaces(&mut s, 11);
    add_str(&mut s, "and");
    add_spaces(&mut s, 5);
    add_str(&mut s, "latest");
    add_spaces(&mut s, 9);
    add_str(&mut s, "is\n");
    add_spaces(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_spaces(&mut s, 8);
    add_str(&mut s, "has");
    add_spaces(&mut s, 13);
    add_str(&mut s, "downloads");
    add_spaces(&mut s, 5);
    add_str(&mut s, "the");
    add_spaces(&mut s, 14);
    add_str(&mut s, "version");
    add_spaces(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".\n");
    return s;
}

fn main() {
    print!("{}", create_string());
}
