use std::ops::{Add,AddAssign};
use std::iter::successors;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle, ProgressIterator};

// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image width in pixels
    #[arg(long, default_value_t = 1000)]
    width: usize,

    /// Image height in pixels
    #[arg(long, default_value_t = 1000)]
    height: usize,
    

    /// Maximum number of iterations for the escape time algorithm
    #[arg(long, default_value_t = 1000)]
    max_iter: usize,

    /// Output filename
    #[arg(long, default_value = "fractal.ppm")]
    output: String,
}
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

    fn square(&self) -> Complex {
        Complex {
            re: self.re * self.re - self.im * self.im,
            im: 2.0 * self.re * self.im,
        }
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
    let args = Args::parse();

    let img = generate_image(args.width, args.height, args.max_iter);

    write_ppm_p6(&args.output, args.width, args.height, &img)
}

/// Generate a Mandelbrot image
fn generate_image(width: usize, height: usize, max_iter: usize) -> Vec<Color> {
    
    // Progress bar setup
    let total = (width * height) as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)"
        ).unwrap()
    );

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .progress_with(pb) // <- magia
        .map(|(x, y)| {
            let c = map_screen_to_complex(x, y, width, height);
            match mandelbrot(c, max_iter) {
                // Points in the set
                None => Color { r: 0, g: 0, b: 0 },
                // Points outside the set, colored depending on the escape time
                Some(s) => color(s),
            }
        })
        .collect()
}

/// Map screen plane coordinates to complex plane coordinates
fn map_screen_to_complex(x: usize, y: usize, width: usize, height: usize) -> Complex {

    let  x_interval = (-1.5, 1.5);
    let  y_interval = (-1.5, 1.5);

    let re = (x as f64 / width as f64) * (x_interval.1 - x_interval.0) + x_interval.0;
    let im = (y as f64 / height as f64) * (y_interval.1 - y_interval.0) + y_interval.0;
    Complex { re, im }
}

/// Compute the escape time for a point in the Mandelbrot set.
fn mandelbrot(c: Complex, max_iter: usize) -> Option<f64>  {

    // Generate the sequence z_{n+1} = z_n^2 + c, starting from z_0 = 0
    // Stop if the magnitude of z exceeds 2 (i.e., magnitude_squared > 4)
    let esc = successors(Some(Complex { re: 0.0, im: 0.0 }), move |&z| Some(z.square() + c))
        .take(max_iter)                 // Limit the number of iterations
        .enumerate()                 // Keep track of the iteration count
        .find(|(_, z)| z.magnitude_squared() > 4.0);        // Escape condition


    // Apply smoothing formula if the point escaped
    // https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Continuous_(smooth)_coloring
    esc.map(|(n, z)| {
        let zn = z.magnitude();
        let nu = (zn.ln()).ln() / 2.0_f64.ln(); // ln(ln(|z_n|))/ln(2)
        (n as f64) + 1.0 - nu // Smooth iteration count
    })
}

/// Map the escape time to a color
fn color(escape_time: f64) -> Color {
    return Color {
        r: (escape_time * 9.0) as u8,
        g: (escape_time * 7.0) as u8,
        b: (escape_time * 5.0) as u8,
    };
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
