/// Complex numbers and complex number operations

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

/// Take two complex numbers and return the product
pub fn multiply(z: Complex, w: Complex) -> Complex {
    Complex {
        re: (z.re * w.re) - (z.im * w.im),
        im: (z.re * w.im) + (z.im * w.re),
    }
}

/// Take two complex numbers and add them
pub fn add(z: Complex, w: Complex) -> Complex {
    Complex {
        re: z.re + w.re,
        im: z.im + w.im,
    }
}

/// Take a complex number and calculate the norm square
pub fn norm_sq(z: Complex) -> f64 {
    (z.re * z.re) + (z.im * z.im)
}
