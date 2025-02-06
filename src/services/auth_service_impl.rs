use chrono::Utc;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::repositories::auth_repository::AuthRepository;
use crate::services::auth_service::AuthService;
use crate::utils::errors::ErrCustom;
use crate::repositories::auth_repository_impl::AuthRepositoryImpl;
use crate::models::user::User;
use std::sync::Arc;
use log::error;
use validator::Validate;


pub struct AuthServiceImpl {
    repository: Arc<AuthRepositoryImpl>,
}

impl AuthServiceImpl {
    pub fn new(repository: Arc<AuthRepositoryImpl>) -> Self {
        Self { repository }
    }
}

impl AuthService for AuthServiceImpl {
    async fn login(&self, request: LoginRequest) -> Result<String, ErrCustom> {
        if let Err(validation_errors) = request.validate() {
            return Err(ErrCustom::ValidationError(validation_errors.to_string()));
        }

        let new_login = User {
            id: None,
            username: request.username,
            password: request.password,
            email: "-".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: None,
        };

        match self.repository.login(new_login).await {
            Ok(true) => Ok("login user success ".to_string()),
            Ok(false) => Err(ErrCustom::InvalidCredentials),
            Err(e) => Err(e),
        }
    }

    async fn register(&self, request: RegisterRequest) -> Result<String, ErrCustom> {
        request.validate().map_err(|e| ErrCustom::ValidationError(e.to_string()))?;

        //CHECK APAKAH USER UDAH EXIST
        if self.repository.exist_kah(request.username.as_str()).await? {
            return Err(ErrCustom::UsernameExists);
        }

        // MAPPING
        let new_user = User {
            id: None,
            username: request.username.clone(),
            password: request.password,
            email: request.email,
            created_at: Utc::now().timestamp_millis(),
            updated_at: None,
        };

        self.repository.create_user(new_user).await?;
        Ok(format!("register success"))
    }
}