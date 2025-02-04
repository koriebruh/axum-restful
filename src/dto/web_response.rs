use std::any::Any;
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::errors::ErrCustom;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponse {
    pub status: String,
    pub message: String,
    pub data: Option<Value>,
}

// INTO RESPONSE ITU CONTRACT  BAWAAN AXUM
impl IntoResponse for ErrCustom {
    fn into_response(self) -> Response {
        match self {
            ErrCustom::ValidationError(msg) =>{
                let error_response = WebResponse {
                    status: "error".to_string(),
                    message: msg,
                    data: None,
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },
            ErrCustom::InvalidCredentials =>{
                let error_response = WebResponse {
                    status: "error".to_string(),
                    message: ErrCustom::InvalidCredentials.to_string(),
                    data: None,
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },ErrCustom::UsernameExists =>{
                let error_response = WebResponse {
                    status: "error".to_string(),
                    message: ErrCustom::UsernameExists.to_string(),
                    data: None,
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },ErrCustom::EmailExists =>{
                let error_response = WebResponse {
                    status: "error".to_string(),
                    message: ErrCustom::EmailExists.to_string(),
                    data: None,
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },ErrCustom::DatabaseError(msg) =>{
                let error_response = WebResponse {
                    status: "error".to_string(),
                    message: msg.to_string(),
                    data: None,
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },
            _ => {
                let error_response = WebResponse {
                    status: "error".to_string(),
                    message: "unknown error".to_string(),
                    data: None,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}