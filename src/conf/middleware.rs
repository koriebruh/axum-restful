use axum::{
    http::{Request, HeaderMap},
    middleware::Next,
    response::{Response, IntoResponse},
};
use axum::body::Body;
use crate::{conf::jwt::validate_jwt, utils::errors::ErrCustom};

pub async fn middleware_jwt(
    request: Request<Body>,
    next: Next,
) -> Response {
    let headers = request.headers();

    // Ekstrak header Authorization
    let auth_header = match get_auth_header(headers) {
        Ok(header) => header,
        Err(err) => return err.into_response(),
    };

    // Ekstrak dan validasi token
    let token = match extract_bearer_token(auth_header) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    // Validasi JWT
    match validate_jwt(&token) {
        Ok(_) => next.run(request).await,
        Err(_) => ErrCustom::Unauthorized.into_response(),
    }
}

fn get_auth_header(headers: &HeaderMap) -> Result<&str, ErrCustom> {
    headers
        .get("Authorization")
        .ok_or(ErrCustom::NotProvideToken)?
        .to_str()
        .map_err(|_| ErrCustom::NotProvideToken)
}

fn extract_bearer_token(header: &str) -> Result<String, ErrCustom> {
    let parts: Vec<&str> = header.splitn(2, ' ').collect();

    if parts.len() != 2 || parts[0] != "Bearer" {
        return Err(ErrCustom::Unauthorized);
    }

    let token = parts[1].trim();
    if token.is_empty() {
        return Err(ErrCustom::Unauthorized);
    }

    Ok(token.to_string())
}