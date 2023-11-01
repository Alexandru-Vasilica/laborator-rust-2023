use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let text: String = fs::read_to_string("src/host_copy.txt")?;
    for mut line in text.lines() {
        if !line.starts_with("#") {
            line = line.trim();
            println!("{line}");
        }
    }
    Ok(())
}
