use serde::{Deserialize, Serialize};

pub mod auth_controller;
pub mod post_controller;
pub mod tag_controller;
pub mod user_controller;

#[derive(Serialize, Deserialize)]
pub struct IdPath {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    message: String,
    success: bool,
}
