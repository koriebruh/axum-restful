use std::sync::Arc;
use axum::Json;
use axum::response::{IntoResponse, Response};
use cookie::Cookie;
use cookie::time::{Duration, OffsetDateTime};
use http::{header, HeaderValue, StatusCode};
use serde_json::json;
use crate::conf::jwt::{gen_jwt, validate_jwt};
use crate::controllers::auth_controller::AuthController;
use crate::dto::login_request::LoginRequest;
use crate::dto::login_response::LoginResponse;
use crate::dto::register_request::RegisterRequest;
use crate::dto::web_response::{WebResponse, WebResponseNoData};
use crate::services::auth_service::AuthService;
use crate::services::auth_service_impl::AuthServiceImpl;


pub struct AuthControllerImpl {
    service: Arc<AuthServiceImpl>,
}

impl AuthControllerImpl {
    pub fn new(service: Arc<AuthServiceImpl>) -> Self {
        Self { service }
    }
}

impl AuthController for AuthControllerImpl {
    async fn login(&self, Json(request): Json<LoginRequest>) -> Response {

        //  TESTING LOGIN
        match self.service.login(request.clone()).await {
            Ok(message) => {
                // GENERATE JWT
                let token = gen_jwt(&request.username).unwrap();

                //VALIDATION AND SEND TOKEN WITH PAYLOAD,Ambil datanya aja  makanya pake unwrap
                let claims = validate_jwt(&token).unwrap();

                //CREATE COOKIE
                let cookie = Cookie::build(("auth_token", token.clone()))
                    .path("/")
                    .secure(true)  // Only sent over HTTPS
                    .http_only(true)  // Not accessible via JavaScript
                    .same_site(cookie::SameSite::Strict)
                    .expires(OffsetDateTime::now_utc() + Duration::minutes(15))
                    .build();

                let success_response = WebResponse {
                    status: "success".to_string(),
                    message: message.to_string(),
                    data: Some(json!(LoginResponse{
                        access_token: token,
                        token_type: "Bearer".to_string(),
                        expires_in: claims.exp,
                    })),
                };

               let mut response = (StatusCode::OK, Json(success_response)).into_response();
                response.headers_mut().insert(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&cookie.to_string()).unwrap()
                );

                response

            }
            Err(err) => err.into_response(),
        }
    }

    async fn register(&self, Json(request): Json<RegisterRequest>) -> Response {
        match self.service.register(request).await {
            Ok(message) => {
                let response = WebResponseNoData {
                    status: "success".to_string(),
                    message: message.to_string(),
                };
                (StatusCode::CREATED, Json(response)).into_response()
            }
            Err(err) => err.into_response(),
        }
    }

    async fn logout(&self) -> Response {
        todo!()
    }
}