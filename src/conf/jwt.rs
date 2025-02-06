use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // what u want save, like id_user
    company: String,
    exp: usize,
}
const SECRET_KEY: &[u8] = b"my_secret_key";
const COMPANY: &str = "Korium"; // my company name in future before 2030 and will be big after 2030 add run in blockchain tech

pub fn gen_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(10)) // lifetime token
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        company: COMPANY.to_string(),
        exp: expiration as usize,
    };

    let token = encode(&Header::default(),
                       &claims,
                       &EncodingKey::from_secret(SECRET_KEY),
    )?;

    Ok(token)
}

// MENGEMBALIKAN DATA, DAN JIKA SUDAH KADALUARSA TOKEN AKAN DI KEMBALKAN ERRORR
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true; // Pastikan token dicek masa kedaluwarsanya

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &validation,
    )?;

    let claims = decoded.claims;
    let now = Utc::now().timestamp() as usize;

    println!("🔹 SEKARANG: {}, EXPIRED DI: {}", now, claims.exp);

    if now > claims.exp {
        return Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature));
    }

    Ok(claims)
}
