fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }
    let mut d: u32 = 2;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut a = x as u32 + 1;
    while !is_prime(a) {
        a += 1;
    }
    if a > std::u16::MAX as u32 {
        return None;
    }
    return Some(a as u16);
}

fn main() {
    let mut i: u16 = 1;
    while let Some(x) = next_prime(i) {
        i = x;
        println! {"{} ",i};
    }
}
