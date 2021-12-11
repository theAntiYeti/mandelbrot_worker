mod complex;
use complex::Complex;

mod generate;

fn main() {
    let top_left = Complex {
        re: -1.0,
        im:  1.0,
    };

    let bottom_right = Complex {
        re:  0.0,
        im:  0.0,
    };

    let grid = generate::create_escape_grid(top_left, bottom_right, 150, 150, 99);

    for i in 0..150 {
        for j in 0..150 {
            print!("{:>2} ", grid[j][i]);
        }
        println!("");
    }
}
