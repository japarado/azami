#[derive(Debug)]
pub enum AuthError {
    NotFound,
    Unauthorized,
    InvalidCredentials,
}

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
}
