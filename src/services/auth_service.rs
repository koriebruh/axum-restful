use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::utils::errors::ErrCustom;

pub trait AuthService {
    async fn login(&self, request : LoginRequest) -> Result<String, ErrCustom>;
    async fn register(&self, request : RegisterRequest) -> Result<String, ErrCustom>;
}