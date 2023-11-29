use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new<T1, T2>(real: T1, imag: T2) -> Self
    where
        f64: From<T1> + From<T2>,
        T1: Sized,
        T2: Sized,
    {
        Self {
            real: f64::from(real),
            imag: f64::from(imag),
        }
    }
    fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imag: self.imag * -1.0,
        }
    }
    fn to_string(&self) -> String {
        return format!("{self}");
    }
}

impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Complex {
            real: f64::from(value),
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex {
            real: value,
            imag: 0.0,
        }
    }
}

impl<T> Add<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;

    fn add(self, rhs: T) -> Self::Output {
        let to_add = Complex::from(rhs);
        Complex {
            real: self.real + to_add.real,
            imag: self.imag + to_add.imag,
        }
    }
}

impl<T> Sub<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;
    fn sub(self, rhs: T) -> Self::Output {
        let to_sub = Complex::from(rhs);
        Complex {
            real: self.real - to_sub.real,
            imag: self.imag - to_sub.imag,
        }
    }
}

impl<T> Mul<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;
    fn mul(self, rhs: T) -> Self::Output {
        let to_mul = Complex::from(rhs);
        Complex {
            real: self.real * to_mul.real - self.imag * to_mul.imag,
            imag: self.real * to_mul.imag + self.imag * to_mul.real,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex {
            real: self.real * -1.0,
            imag: self.imag * -1.0,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.real != 0.0 {
            write!(f, "{}", self.real)?;
        }
        if self.imag > 0.0 && self.real != 0.0 {
            write!(f, "+")?;
        }

        if self.imag != 0.0 {
            return write!(f, "{}i", self.imag);
        } else {
            if self.real == 0.0 {
                return write!(f, "{}", self.imag);
            } else {
                return write!(f, "");
            }
        }
    }
}

impl<T> AddAssign<T> for Complex
where
    Complex: From<T>,
{
    fn add_assign(&mut self, rhs: T) {
        let to_add = Complex::from(rhs);
        self.real += to_add.real;
        self.imag += to_add.imag;
    }
}

impl<T> SubAssign<T> for Complex
where
    Complex: From<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        let to_sub = Complex::from(rhs);
        self.real -= to_sub.real;
        self.imag -= to_sub.imag;
    }
}

impl<T> MulAssign<T> for Complex
where
    Complex: From<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        let to_mul = Complex::from(rhs);
        let aux = self.real;
        self.real = self.real * to_mul.real - self.imag * to_mul.imag;
        self.imag = aux * to_mul.imag + self.imag * to_mul.real;
    }
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    // println!("{:?} {:?}", a, b);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    let mut k = Complex::new(0, 0);

    k += 1;
    k += 1.0;
    assert_eq!(k.real, 2.0);
    assert_eq!(k.imag, 0.0);
    k += Complex::new(5, -2);
    assert_eq!(k.real, 7.0);
    assert_eq!(k.imag, -2.0);

    k -= 2;
    k -= -1.0;
    assert_eq!(k.real, 6.0);
    assert_eq!(k.imag, -2.0);
    k -= Complex::new(-5, 2);
    assert_eq!(k.real, 11.0);
    assert_eq!(k.imag, -4.0);

    k *= 1;
    assert_eq!(k.real, 11.0);
    assert_eq!(k.imag, -4.0);
    k *= Complex::new(5, -2);
    assert_eq!(k.real, 47.0);
    assert_eq!(k.imag, -42.0);

    println!("ok!");
}
