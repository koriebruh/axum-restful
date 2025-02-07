use std::any::Any;
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::errors::ErrCustom;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponse {
    pub status: String,
    pub message: String,
    pub data: Option<Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponseNoData {
    pub status: String,
    pub message: String,

}

// INTO RESPONSE ITU CONTRACT  BAWAAN AXUM
impl IntoResponse for ErrCustom {
    fn into_response(self) -> Response {
        warn!("Error Response is {:?}", self);
        match self {
            ErrCustom::ValidationError(msg) =>{
                let error_response = WebResponseNoData {
                    status: "error".to_string(),
                    message: msg,
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },
            ErrCustom::InvalidCredentials =>{
                let error_response = WebResponseNoData {
                    status: "error".to_string(),
                    message: ErrCustom::InvalidCredentials.to_string(),
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },ErrCustom::UsernameExists =>{
                let error_response = WebResponseNoData {
                    status: "error".to_string(),
                    message: ErrCustom::UsernameExists.to_string(),
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },ErrCustom::EmailExists =>{
                let error_response = WebResponseNoData {
                    status: "error".to_string(),
                    message: ErrCustom::EmailExists.to_string(),
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },ErrCustom::DatabaseError(msg) =>{
                let error_response = WebResponseNoData {
                    status: "error".to_string(),
                    message: msg.to_string(),
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            },ErrCustom::JwtError(msg) =>{
                let error_response = WebResponseNoData {
                    status: "error".to_string(),
                    message: msg.to_string(),
                };
                (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
            },
            _ => {
                let error_response = WebResponseNoData {
                    status: "error".to_string(),
                    message: "internal server error, please try again later".to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}