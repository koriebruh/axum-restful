use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token : String,
    pub token_type : String,
    pub expires_in : usize,
}