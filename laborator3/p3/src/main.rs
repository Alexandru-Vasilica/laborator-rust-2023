#[derive(Debug)]
enum Error {
    OverflowSum,
    OverflowMultiply,
}

fn add(x: u32, y: u32) -> Result<u32, Error> {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        return Err(Error::OverflowSum);
    }

    Ok(sum as u32)
}

fn multiply(x: u32, y: u32) -> Result<u32, Error> {
    let prod = x as u64 * y as u64;
    if prod > std::u32::MAX as u64 {
        return Err(Error::OverflowMultiply);
    }

    Ok(prod as u32)
}

fn main() {
    match add(std::u32::MAX, 2) {
        Ok(x) => println!("value: {}", x),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    match multiply(std::u32::MAX, 2) {
        Ok(x) => println!("value: {}", x),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
