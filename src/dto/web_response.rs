use std::any::Any;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponse {
    pub status: String,
    pub message: String,
    pub data: Option<Value>,
}