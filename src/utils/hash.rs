use argon2::Config;
use crate::utils::errors::ErrCustom;

pub(crate) fn verify_password(password: &str, hash: &str) -> Result<bool, ErrCustom> {
    argon2::verify_encoded(hash, password.as_bytes())
        .map_err(|e| ErrCustom::HashError(e))
}
pub(crate) fn hash_password(password: &str) -> Result<String, ErrCustom> {
    let salt = rand::random::<[u8; 32]>();
    let config = Config::default();

    argon2::hash_encoded(
        password.as_bytes(),
        &salt,
        &config,
    ).map_err(|e| ErrCustom::HashError(e))
}