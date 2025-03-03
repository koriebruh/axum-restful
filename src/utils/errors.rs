use sqlx::mysql::MySqlPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrCustom {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Username already exists")]
    UsernameExists,

    #[error("Email already exists")]
    EmailExists,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Hashing error: {0}")]
    HashError(#[from] argon2::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("JWT Error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Not provide token")]
    NotProvideToken,

    #[error("invalidate token")]
    Unauthorized,
}