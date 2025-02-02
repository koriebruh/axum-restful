use sqlx::MySqlPool;
use crate::models::user::User;
use crate::repositories::auth_repository::AuthRepository;
use crate::utils::errors::ErrCustom;
use crate::utils::hash::hash_password;
use crate::utils::hash::verify_password;


pub struct AuthRepositoryImpl {
    db: MySqlPool,
}

impl AuthRepositoryImpl {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}
impl AuthRepository for AuthRepositoryImpl {
    async fn create_user(&self, user: User) -> Result<bool, ErrCustom> {
        let hashed_pass = hash_password(user.password.as_str())?;

        let execute = sqlx::query!(
            "INSERT INTO users (username, password, email, created_at) VALUES (?,?,?,?)",
            user.username,
            hashed_pass,
            user.email,
            user.created_at
        ).execute(&self.db).await;

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
        let execute = sqlx::query!(
            "SELECT username, password FROM users WHERE username = ?",user.username
        ).fetch_optional(&self.db).await?;

        match execute {
            Some(execute) => {
                if let Some(stored_hash) = execute.password {
                    let valid = verify_password(user.password, &stored_hash)?;
                    Ok(valid)
                }
            }
            None => Ok(false)
        }
    }

    async fn exist_kah(&self, username: &str) -> bool {
        let execute = sqlx::query!(
            "SELECT EXISTS ( SELECT 1 FROM users WHERE username = ?)", username
        ).fetch_optional(&self.db).await?;

        match execute {
            Some(record) => {
                Ok(record.exists == 1)
            }
            None => {
                Ok(false)
            }
        }
    }
}