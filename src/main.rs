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

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() -> std::io::Result<()> {
    let width = 1000;
    let height = 1000;
    let max_iter = 1000;

    let img =  vec![Color { r: 173, g: 255, b: 47 }; width * height];

    write_ppm_p6("fractal.ppm", width, height, &img)
}

/// Map screen plane coordinates to complex plane coordinates
fn map_screen_to_complex(x: u32, y: u32, width: u32, height: u32) -> Complex {

    let  x_interval = (-1.5, 1.5);
    let  y_interval = (-1.5, 1.5);

    let re = (x as f64 / width as f64) * (x_interval.1 - x_interval.0) + x_interval.0;
    let im = (y as f64 / height as f64) * (y_interval.1 - y_interval.0) + y_interval.0;
    Complex { re, im }
}

/// Convert the image data to PPM format
fn ppm_bytes(width: usize, height: usize, img: &[Color]) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(format!("P6\n{} {}\n255\n", width, height).as_bytes());
    data.extend(img.iter().flat_map(|p| [p.r, p.g, p.b]));
    data
}

/// Write a PPM P6 image file.
fn write_ppm_p6(filename: &str, width: usize, height: usize, img: &[Color]) -> std::io::Result<()> {
    std::fs::write(filename, ppm_bytes(width, height, img))
}
