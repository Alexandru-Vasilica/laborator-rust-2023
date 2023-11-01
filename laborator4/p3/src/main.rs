use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let text: String = fs::read_to_string("src/text.txt")?;
    let mut result_str: String = String::new();
    let abbreviations: Vec<(&str, &str)> = vec![
        ("pentru", "pt"),
        ("pentru", "ptr"),
        ("domnul", "dl"),
        ("doamna", "dna"),
    ];
    for word in text.split(" ") {
        let mut final_word: String = String::from(word);
        for (long, short) in abbreviations.iter() {
            if &word == short {
                final_word = long.to_string();
            }
        }
        result_str.push_str(&final_word);
        result_str.push(' ');
    }
    println!("{result_str}");
    Ok(())
}
