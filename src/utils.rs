use crate::errors::{AuthError, ServiceError};
use argonautica::{Hasher, Verifier};
use std::env;

pub fn get_secret_key() -> String {
    env::var("SECRET_KEY").unwrap_or("1234".repeat(8))
}

pub fn get_domain() -> String {
    env::var("DOMAIN").unwrap_or(String::from("localhost"))
}

fn get_salt() -> String {
    env::var("SALT").unwrap_or("4321".repeat(8))
}

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(get_secret_key())
        .with_salt(get_salt())
        .hash()
        .map_err(|_| ServiceError::InternalServerError)
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(get_secret_key())
        .verify()
        .map_err(|_| ServiceError::InternalServerError)
}
