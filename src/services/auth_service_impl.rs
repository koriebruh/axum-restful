use chrono::Utc;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::repositories::auth_repository::AuthRepository;
use crate::services::auth_service::AuthService;
use crate::utils::errors::ErrCustom;
use crate::repositories::auth_repository_impl::AuthRepositoryImpl;
use crate::models::user::User;


pub struct AuthServiceImpl {
    repository: AuthRepositoryImpl,
}

impl AuthService for AuthServiceImpl {
    async fn login(&self, request: LoginRequest) -> Result<String, ErrCustom> {
        let new_login = User {
            id: None,
            username: request.username,
            password: request.password,
            email: "-".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: None,
        };

        match self.repository.login(new_login).await {
            Ok(_) => Ok("login user success ".to_string()),
            Err(e) => Err(e),
        }
    }

    async fn register(&self, request: RegisterRequest) -> Result<String, ErrCustom> {
        // MAPPING
        let new_user = User {
            id: None,
            username: request.username,
            password: request.password,
            email: request.email,
            created_at: Utc::now().timestamp_millis(),
            updated_at: None,
        };

        match self.repository.exist_kah(&request.username).await {
            Ok(true) => {
                match self.repository.create_user(new_user) {
                    Ok(_) => Ok("login user success ".to_string()),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}