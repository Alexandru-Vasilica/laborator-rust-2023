use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let text: String = fs::read_to_string("src/host_copy.txt")?;
    for mut line in text.lines() {
        if !line.starts_with("#") {
            line = line.trim();
            let ip: &str;
            if let Some(x) = line.find(" ") {
                ip = &line[..x];
                line = &line[x..];
            } else {
                continue;
            }
            line = line.trim();
            let name: &str;
            if let Some(x) = line.find(" ") {
                name = &line[..x];
            } else {
                name = &line;
            }
            println!("{}=>{}", name.to_string(), ip.to_string());
        }
    }
    Ok(())
}
