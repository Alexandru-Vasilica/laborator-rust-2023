#[derive(Debug)]
enum Error {
    NoSolutions,
}
//solves the equation: a*x^2+b*x+c=0 for real numbers
fn compute_real_solutions(a: f32, b: f32, c: f32) -> Result<(f32, f32), Error> {
    let delta: f32 = b * b - 4.0 * a * c;
    if delta < 0.0 {
        return Err(Error::NoSolutions);
    }
    let x1: f32 = (-1.0 * b + delta.sqrt()) / (2.0 * a);
    let x2: f32 = (-1.0 * b - delta.sqrt()) / (2.0 * a);
    Ok((x1, x2))
}

fn main() {
    let a: f32 = 1.0;
    let b: f32 = -5.0;
    let c: f32 = 4.0;
    match compute_real_solutions(a, b, c) {
        Ok((x1, x2)) => println!("The solutions are {x1} and {x2}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
