extern crate bcrypt;

use axum::http::StatusCode;

pub fn hash_password<P: AsRef<[u8]>>(password: P) -> Result<String, StatusCode> {
    bcrypt::hash(password, 12).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
