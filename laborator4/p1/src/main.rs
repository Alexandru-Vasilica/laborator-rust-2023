use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let text = fs::read_to_string("src/text.txt")?;
    let mut line_bytes: &str = "";
    let mut line_chars: &str = "";
    let mut max_bytes: u32 = 0;
    let mut max_chars: u32 = 0;
    for line in text.lines() {
        let mut nr_chars: u32 = 0;
        let mut nr_bytes: u32 = 0;
        for _char in line.chars() {
            nr_chars += 1;
        }
        for _byte in line.bytes() {
            nr_bytes += 1;
        }
        if nr_bytes > max_bytes {
            max_bytes = nr_bytes;
            line_bytes = line.clone();
        }
        if nr_chars > max_chars {
            max_chars = nr_chars;
            line_chars = line.clone();
        }
    }
    println!("Longest line considering nr of characters: {}", line_chars);
    println!("Longest line considering nr of bytes: {}", line_bytes);
    Ok(())
}
