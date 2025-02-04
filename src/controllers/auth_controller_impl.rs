use std::sync::Arc;
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;
use crate::controllers::auth_controller::AuthController;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::dto::web_response::WebResponse;
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
        match self.service.login(request).await {
            Ok(message) => {
                let success_response = WebResponse {
                    status: "success".to_string(),
                    message:"login successfully".to_string(),
                    data: None,
                };
                (StatusCode::OK, Json(success_response)).into_response()
            },
            Err(err) => err.into_response(),
        }
    }

    async fn register(&self, Json(request): Json<RegisterRequest>) -> Response {
        match self.service.register(request).await {
            Ok(result)=> {
                let response = WebResponse {
                    status: "success".to_string(),
                    message: "register successful".to_string(),
                    data: Some(json!(result)),
                };
                (StatusCode::CREATED, Json(response)).into_response()
            },
            Err(err) => err.into_response(),
        }
    }

    async fn logout(&self) -> Response {
        todo!()
    }
}