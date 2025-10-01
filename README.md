# Mandelbrot Fractal

![Mandelbrot Example](examples/fractal.png)

A simple **Mandelbrot set renderer** written in Rust.  
It generates fractal images with smooth coloring and parallel computation.

## Features
- Pure Rust implementation
- Smooth coloring (continuous escape time)
- Parallel computation with [Rayon](https://crates.io/crates/rayon)
- Progress bar with [Indicatif](https://crates.io/crates/indicatif)
- Argument parsing with [Clap](https://crates.io/crates/clap)
- Output to **PNG**

## Usage

Generate a fractal with default parameters:
```bash
cargo run --release
