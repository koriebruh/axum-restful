use sqlx::{MySqlPool, Row};
use crate::models::user::User;
use crate::repositories::auth_repository::AuthRepository;
use crate::utils::errors::ErrCustom;
use crate::utils::hash::hash_password;
use crate::utils::hash::verify_password;
use log::{info, error, debug};


pub struct AuthRepositoryImpl {
    db: MySqlPool,
}

impl AuthRepositoryImpl {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}
#[async_trait::async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn create_user(&self, user: User) -> Result<bool, ErrCustom> {
        let hashed_pass = hash_password(user.password.as_str())?;

        let execute = sqlx::query(
            "INSERT INTO users (username, password, email, created_at) VALUES (?,?,?,?)",
        ).bind(user.username)
            .bind(hashed_pass)
            .bind(user.email)
            .bind(user.created_at)
            .execute(&self.db).await;

        match execute {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("Duplicate entry") {
                    if e.to_string().contains("username") {
                        Err(ErrCustom::UsernameExists)
                    } else if e.to_string().contains("email") {
                        Err(ErrCustom::EmailExists)
                    } else {
                        Err(ErrCustom::DatabaseError(e))
                    }
                } else {
                    Err(ErrCustom::DatabaseError(e))
                }
            }
        }
    }

    // ? maksud nya Jika hasilnya adalah Err(E) (untuk Result) atau None (untuk Option),
    // maka fungsi akan mengembalikan error tersebut (atau None)
    async fn login(&self, user: User) -> Result<bool, ErrCustom> {
        let execute = sqlx::query(
            "SELECT username, password FROM users WHERE username = ?"
        ).bind(user.username)
            .fetch_optional(&self.db).await?;

        match execute {
            Some(row) => {
                let stored_hash: String = row.try_get("password")?;
                let valid = verify_password(&user.password, &stored_hash)?;
                Ok(valid)
            }

            None => Err(ErrCustom::InvalidCredentials),
        }
    }

    async fn exist_kah(&self, username: &str) -> Result<bool, ErrCustom> {
        // Query untuk cek apakah username ada di tabel users, kalau ada return true
        let result = sqlx::query(
                "SELECT 1 FROM users WHERE username = ? LIMIT 1",
            ).bind(username)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }
}