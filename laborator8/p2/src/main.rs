use anyhow::Result;
use serde_derive::Deserialize;
use std::fmt::{Display, Write};
use std::io;
use ureq;

#[derive(Debug, Deserialize)]
struct JsonRespone {
    results: Vec<Spell>,
}

#[derive(Debug, Deserialize)]
struct Spell {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct SpellData {
    name: String,
    desc: Vec<String>,
    level: i32,
}

impl Spell {
    fn get_details(&self) -> Result<SpellData> {
        let mut s = String::new();
        write!(&mut s, "https://www.dnd5eapi.co{}", self.url)?;
        // println!("{s}");
        let body: String = ureq::get(&s).call()?.into_string()?;
        // println!("{body}");
        let data: SpellData = serde_json::from_str(&body[0..])?;
        // println!("{:?}", data);
        Ok(data)
    }
}

impl Display for SpellData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = self.desc.join(" ");
        writeln!(
            f,
            "Name: {}\nLevel:{}\nDescription:{}",
            self.name, self.level, description
        )
    }
}
fn main() -> Result<()> {
    let mut input = String::new();
    println!("Input a term to search for:");
    io::stdin().read_line(&mut input)?;
    let body: String = ureq::get("https://www.dnd5eapi.co/api/spells")
        .call()?
        .into_string()?;
    let data: JsonRespone = serde_json::from_str(&body[0..])?;
    data.results
        .into_iter()
        .filter(|res| {
            res.name
                .to_lowercase()
                .contains(&input.trim().to_lowercase())
        })
        .map(|e| e.get_details())
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .for_each(|e| print!("{e}"));

    Ok(())
}
