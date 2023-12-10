pub mod auth;
pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;
