fn add(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        panic!("Overflow sum!");
    }

    return sum as u32;
}

fn multiply(x: u32, y: u32) -> u32 {
    let prod = x as u64 * y as u64;
    if prod > std::u32::MAX as u64 {
        panic!("Overflow multiplication!");
    }

    return prod as u32;
}

fn main() {
    let x = multiply(211, 213);
    let y = add(std::u32::MAX, 2);
    println!("{x} {y}");
}
