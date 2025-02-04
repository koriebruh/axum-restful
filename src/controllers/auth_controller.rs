use axum::Json;
use axum::response::Response;
use http::StatusCode;
use crate::dto::web_response::WebResponse;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;

pub trait AuthController {
    async fn login(&self, request: Json<LoginRequest>) -> Response;
    async fn register(&self, request: Json<RegisterRequest>) ->  Response;
    async fn logout(&self) ->  Response;
}