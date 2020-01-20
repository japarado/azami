use serde::{Deserialize, Serialize};

pub mod auth_controller;
pub mod user_controller;
pub mod post_controller;

#[derive(Serialize, Deserialize)]
pub struct IdPath {
    id: i32,
}

