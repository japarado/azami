use serde::{Deserialize, Serialize};

pub mod tag_controller;
pub mod auth_controller;
pub mod post_controller;

#[derive(Serialize, Deserialize)]
pub struct IdPath {
    id: i32,
}

