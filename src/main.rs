use std::ops::{Add,AddAssign};

#[derive(Debug, Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    fn magnitude_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Complex) {
        self.re += other.re;
        self.im += other.im;
    }
}

fn main() {
    println!("Hello, world!");
}
