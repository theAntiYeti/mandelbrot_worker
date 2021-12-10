use crate::complex;

use complex::Complex;
use complex::multiply;
use complex::conjugate;
use complex::norm_sq;

/// Return number of iterations before escape (corresponds to colour)
pub fun mb_iterations(c: Complex, limit: u32) -> u32 {
    let iters: u32 = 0;
    let z: Complex = c;

    while (iter < limit && norm_sq(z) < 4.0) {
        z = add(multiply(z,z), c);
    }

    iter
}