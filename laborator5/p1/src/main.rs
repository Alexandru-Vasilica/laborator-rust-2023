use anyhow::Result;
use std::fs;

#[derive(Debug)]
struct Person {
    name: String,
    phone: String,
    age: u32,
}

fn main() -> Result<()> {
    let mut people: Vec<Person> = Vec::new();
    let data = fs::read_to_string("src/data.txt")?;
    for line in data.lines() {
        let mut i = 0;
        let mut name: String = String::new();
        let mut phone: String = String::new();
        let mut age: u32 = 0;
        for field in line.split(",") {
            match i {
                0 => name = field.to_string(),
                1 => phone = field.to_string(),
                _ => age = field.parse()?,
            }
            i += 1;
        }
        people.push(Person { name, phone, age })
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
