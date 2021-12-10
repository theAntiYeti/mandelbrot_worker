use mandelbrot_worker::complex;
use complex::multiply;
use complex::Complex;

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
