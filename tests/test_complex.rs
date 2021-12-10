use mandelbrot_worker::complex;

use complex::Complex;
use complex::multiply;
use complex::conjugate;
use complex::norm_sq;

/* Test Multiplication */
#[test]
fn test_multiplcation_1() {
    let z = Complex {
        re: 1.0,
        im: 1.0,
    };

    let expected = Complex {
        re: 0.0,
        im: 2.0,
    };
    assert_eq!(expected, multiply(z,z))
}

#[test]
fn test_multiplcation_2() {
    let z = Complex {
        re: 4.0,
        im: 3.0,
    };
    
    let w = Complex {
        re: 4.0,
        im: -3.0,
    };

    let expected = Complex {
        re: 25.0,
        im: 0.0,
    };
    assert_eq!(expected, multiply(z, w))
}

/* Test conjugate */
#[test]
fn test_conjugation() {
    let z = Complex {
        re: 7.5,
        im: 2.4,
    };

    let expected = Complex {
        re: 7.5,
        im: -2.4,
    };
    
    assert_eq!(expected, conjugate(z))
}

/* Test norm square */
#[test]
fn test_norm_sq() {
    let z = Complex {
        re: 3.0,
        im: 4.0,
    };

    assert_eq!(25.0, norm_sq(z))
}