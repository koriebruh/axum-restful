use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(
        length(min = 8, max = 80, message = "should be between 8 and 80 characters")
    )]
    pub username: String,

    #[validate(
        length(min = 8, max = 80, message = "should be between 8 and 80 characters")
    )]
    pub password: String,

    #[validate(
        length(min = 3, max = 20, message = "should be between 3 and 20 characters"),
        email(message = "Invalid email address")
    )]
    pub email: String,
}
