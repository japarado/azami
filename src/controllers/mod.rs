use serde::{Deserialize, Serialize};

pub mod auth_controller;
pub mod user_controller;

#[derive(Serialize, Deserialize)]
pub struct IdPath {
    id: i32,
}

