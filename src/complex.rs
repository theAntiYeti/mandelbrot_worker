/// Complex numbers and complex number operations

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

pub fn multiply(z: Complex, w: Complex) -> Complex {
    Complex {
        re: (z.re * w.re) - (z.im * w.im),
        im: (z.re * w.im) + (z.im * w.re),
    }
}