use std::sync::Arc;
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use log::{error, info};
use serde_json::json;
use crate::controllers::auth_controller::AuthController;
use crate::dto::login_request::LoginRequest;
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
        match self.service.login(request).await {
            Ok(message) => {
                let success_response = WebResponse {
                    status: "success".to_string(),
                    message: message.to_string(),
                    data: None,
                };
                (StatusCode::OK, Json(success_response)).into_response()
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