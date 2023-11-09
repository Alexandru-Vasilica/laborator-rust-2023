use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    phone: String,
    age: u32,
}
fn main() -> Result<()> {
    let mut people: Vec<Person> = Vec::new();
    let data = fs::read_to_string("src/data.json")?;
    for line in data.lines() {
        println!("{line}");
        let p: Person = serde_json::from_str(line)?;
        people.push(p);
    }
    if people.len() == 0 {
        anyhow::bail!("No data provided");
    }
    let mut min = 0;
    let mut max = 0;
    for i in 1..people.len() {
        if people[i].age > people[max].age {
            max = i;
        }
        if people[i].age < people[min].age {
            min = i;
        }
    }
    println!("Person with minimum age: {:?}", people[min]);
    println!("Person with minimum age: {:?}", people[max]);
    Ok(())
}
