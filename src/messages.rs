use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestData {
    pub top_left_re: f64,
    pub top_left_im: f64,
    pub bottom_right_re: f64,
    pub bottom_right_im: f64,
    pub height: usize,
    pub width: usize,
    pub iterations: u32,
}
