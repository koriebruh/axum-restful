use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(
        min = 8,
        max = 80,
        message = "Username should be between 8 and 80 characters long"
    ))]
    pub username: String,

    #[validate(length(
        min = 8,
        max = 80,
        message = "Password should be between 8 and 80 characters long"
    ))]
    pub password: String,
}
