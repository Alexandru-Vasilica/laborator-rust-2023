use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("The sum of {0} and {1} exceeds the u32 limit")]
    // {0} will take the first value of the variant
    OverflowSum(u32, u32),
    #[error("The multiplication of {0} and {1} exceeds the u32 limit")]
    // {0} will take the first value of the variant
    OverflowMultiply(u32, u32),
}

fn add(x: u32, y: u32) -> Result<u32, Error> {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        return Err(Error::OverflowSum(x, y));
    }

    Ok(sum as u32)
}

fn multiply(x: u32, y: u32) -> Result<u32, Error> {
    let prod = x as u64 * y as u64;
    if prod > std::u32::MAX as u64 {
        return Err(Error::OverflowMultiply(x, y));
    }

    Ok(prod as u32)
}

fn main() {
    match add(std::u32::MAX, 2) {
        Ok(x) => println!("value: {}", x),
        Err(e) => println!("err: {}", e),
    }
    match multiply(std::u32::MAX, 2) {
        Ok(x) => println!("value: {}", x),
        Err(e) => println!("err: {}", e),
    }
}
