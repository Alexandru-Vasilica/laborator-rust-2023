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

fn int_to_str(mut n: u32) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut n_str: String = String::from("");
    let mut m: u32 = 1;
    while n >= m {
        let mut n2: u32 = n;
        let mut poz: u32 = 0;
        while n2 > m * 10 {
            n2 = n2 / 10;
            poz += 1;
        }
        let cif: u32 = n2 % 10;
        println!("{cif}");
        let c = (cif as u8 + '0' as u8) as char;
        n_str.push(c);
        if poz % 3 == 0 {
            n_str.push(' ');
        }
        m = m * 10;
    }

    return n_str;
}

fn int_to_str_no_sep(mut n: u32) -> String {
    if n == 0 {
        return String::from("0");
    }
    let mut n_str: String = String::from("");
    let mut m: u32 = 1;
    while n >= m {
        let mut n2: u32 = n;
        let mut poz: u32 = 0;
        while n2 > m * 10 {
            n2 = n2 / 10;
            poz += 1;
        }
        let cif: u32 = n2 % 10;
        println!("{cif}");
        let c = (cif as u8 + '0' as u8) as char;
        n_str.push(c);
        m = m * 10;
    }

    return n_str;
}

fn add_integer(s: &mut String, mut n: u32) {
    s.push_str(&int_to_str(n));
}

fn add_float(s: &mut String, mut n: f32) {
    let int: i32 = n.floor() as i32;
    let mut fr: f32 = n - n.floor();
    while fr > fr.floor() {
        fr = fr * 10.0;
    }
    add_integer(&mut s, int as u32);
    s.push('.');
    add_integer(&mut s, fr as u32);
}

fn main() {
    let mut s = String::from("Hello");
    add_float(&mut s, 1123.25);
    print!("{s}");
    // print!("{s}");
    // print!("Done");
}
