fn is_prime(x: i32) -> bool {
    let mut i: i32 = 2;
    if x <= 1 {
        return false;
    }
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }

    return true;
}

fn p1() {
    let mut x = 0;
    while x <= 100 {
        if is_prime(x) {
            println!("{x}");
        }
        x += 1;
    }
}

fn are_coprime(mut a: i32, mut b: i32) -> bool {
    let mut r: i32 = a % b;
    while r > 0 {
        a = b;
        b = r;
        r = a % b;
    }
    return b == 1;
}

fn p2() {
    let mut i: i32 = 0;
    while i <= 100 {
        let mut j = i + 1;
        while j <= 100 {
            if are_coprime(i, j) {
                println!("{i} and {j} are coprime!");
            }
            j += 1;
        }
        i += 1;
    }
}

fn p3() {
    let mut bottles: i32 = 99;
    while bottles > 1 {
        println!(
            "{bottles} bottles of beer on the wall,
         {bottles} bottles of beer.
         Take one down,pass it around, 
          {} bottles of beer on the wall.\n",
            bottles - 1,
        );
        bottles -= 1;
    }
    println!(
        "1 bottle of beer on the wall,
    1 bottle of beer.
    Take one down, pass it around,
    No bottles of beer on the wall.\n"
    );
}

fn main() {
    p1();
    p2();
    p3();
}
