mod cache;

use anyhow::Result;
use std::io::stdin;

fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 {
        return true;
    }
    let mut d = 2;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    return true;
}
fn print_res(x: u32, res: bool) {
    match res {
        true => println!("The number {x} is prime!"),
        false => println!("The number {x} is not prime!"),
    }
}
fn main() -> Result<()> {
    let cache = cache::Cache::new();
    loop {
        let mut buff = String::new();
        println!("Introduceti un numar natural:");
        stdin().read_line(&mut buff)?;
        let Ok(num) = buff.trim().parse::<u32>() else {
            println!("Invalid number {}.Try again!", buff);
            continue;
        };
        if let Some(res) = cache.is_cached(num) {
            println!("The result is already cached!");
            print_res(num, res);
        } else {
            println!("The result was not already cached. Calculating now...");
            let res = is_prime(num);
            print_res(num, res);
            cache.insert(num, res);
        }
    }
}
