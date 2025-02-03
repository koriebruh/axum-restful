use std::sync::Arc;
use axum::Json;
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
    async fn Login(&self, Json(request): Json<LoginRequest>) -> Json<WebResponse> {
        match self.service.login(request).await {
            Ok(result) => {
                let response = WebResponse {
                    status: "success".to_string(),
                    message: "Login successful".to_string(),
                    data: Some(json!(result)),
                };
                Json(response)
            },
            Err(e) => {
                let response = WebResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
                    data: None,
                };
                Json(response)
            },
        }
    }

    async fn Register(&self, Json(request): Json<RegisterRequest>) -> Json<WebResponse> {
        match self.service.register(request).await {
            Ok(result)=> {
                let response = WebResponse {
                    status: "success".to_string(),
                    message: "register successful".to_string(),
                    data: Some(json!(result)),
                };
                Json(response)
            },
            Err(e) => {
                let response = WebResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
                    data: None,
                };
                Json(response)
            },
        }
    }

    async fn Logout(&self) -> Json<WebResponse> {
        todo!()
    }
}