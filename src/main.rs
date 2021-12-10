mod complex;

fn main() {
    let x = complex::Complex {
        re: 1.0,
        im: 1.0,
    };

    println!("{:?}", complex::multiply(x, x))
}
