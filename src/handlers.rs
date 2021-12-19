use crate::{messages, Result, complex, generate};
use warp::{http::StatusCode, reply, Reply};

pub async fn generate_mandelbrot_render(ib_mssg: messages::RequestData) -> Result<impl Reply> {
    let top_left = complex::Complex {
        re: ib_mssg.top_left_re,
        im: ib_mssg.top_left_im,
    };

    let bottom_right = complex::Complex {
        re: ib_mssg.bottom_right_re,
        im: ib_mssg.bottom_right_im,
    };

    let result = generate::create_escape_grid(
        top_left,
        bottom_right,
        ib_mssg.width,
        ib_mssg.height,
        ib_mssg.iterations,
    );

    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}
