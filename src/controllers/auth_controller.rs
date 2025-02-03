use axum::Json;
use crate::dto::web_response::WebResponse;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;

pub trait AuthController {
    async fn Login(&self ,request: Json<LoginRequest>) -> Json<WebResponse>;
    async fn Register(&self, request: Json<RegisterRequest>) -> Json<WebResponse>;
    async fn Logout(&self) -> Json<WebResponse>;

}