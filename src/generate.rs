use crate::complex;

use complex::Complex;
use complex::multiply;
use complex::norm_sq;
use complex::add;

/// Return number of iterations before escape (corresponds to colour)
pub fn mb_iterations(c: Complex, limit: u32) -> u32 {
    let mut iters: u32 = 0;
    let mut z: Complex = c;

    while iters < limit && norm_sq(z) < 4.0 {
        z = add(multiply(z,z), c);
        iters += 1;
    }

    iters
}

/// Create grid of iterations for given region
pub fn create_escape_grid(top_left: Complex, bottom_right: Complex, width: usize, height: usize, limit: u32) -> Vec<Vec<u32>>{
    let re_step: f64 = (bottom_right.re - top_left.re) / (width as f64);
    let im_step: f64 = (bottom_right.im - top_left.im) / (height as f64);
    
    let mut grid: Vec<Vec<u32>> = vec![vec![0; height]; width];

    for i in 0..width {
        for j in 0..height {
            let offset = Complex {
                re: re_step * (i as f64),
                im: im_step * (j as f64),
            };
            grid[i][j] = mb_iterations(add(top_left, offset), limit);
        }
    }

    grid
}