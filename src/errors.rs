use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum AuthError {
    NotFound,
    Unauthorized(UnauthorizedResponse),
    InvalidCredentials,
}

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
}

use actix_web::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct UnauthorizedResponse {
    pub message: String,
}
