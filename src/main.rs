use warp::{Filter, Rejection, Reply};

mod complex;
use complex::Complex;

mod generate;

mod messages;
mod handlers;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health").and_then(health_handler);

    let mdb_route = warp::path!("mdb")
        .and(warp::get())
        .and(warp::body::json())
        .and_then(handlers::generate_mandelbrot_render);
    
    let routes = health_route.or(mdb_route).with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}
