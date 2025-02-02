use crate::models::user::User;
use crate::utils::errors::ErrCustom;
#[async_trait::async_trait]
pub trait AuthRepository {
    async fn create_user(&self, user: User) -> Result<bool, ErrCustom>;
    async fn login(&self, user: User) -> Result<bool, ErrCustom>;
    async fn exist_kah(&self, username: &str) -> Result<bool, ErrCustom>;
}