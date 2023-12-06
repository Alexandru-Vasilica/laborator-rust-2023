use anyhow::Result;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt")?;
    let mut map: HashMap<String, i32> = HashMap::new();
    for word in input
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter(|word| !word.is_empty())
    {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let mut v: Vec<(String, i32)> = map.into_iter().collect();
    let maxlen = v.iter().map(|e| e.0.len()).max();
    if let Some(len) = maxlen {
        v.sort_by_key(|pair| -1 * pair.1);
        for (word, count) in v {
            println!("{word:<length$} => {count}", length = len);
        }
        return Ok(());
    }
    Ok(())
}
