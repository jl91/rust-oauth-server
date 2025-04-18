use std::sync::Arc;
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use uuid::Uuid;
use std::error::Error;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, username: String, password: String, is_active: bool) -> Result<User, Box<dyn Error>> {
        // Validações de domínio
        if username.is_empty() {
            return Err("Username cannot be empty".into());
        }

        if password.len() < 6 {
            return Err("Password must be at least 6 characters".into());
        }

        // Hash da senha
        let password_hash = hash(password, DEFAULT_COST)?;

        // Persistir o usuário
        self.user_repository.create(username, password_hash, is_active).await
    }

    pub async fn get_user(&self, external_id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        self.user_repository.find_by_external_id(external_id).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        self.user_repository.find_all().await
    }

    pub async fn update_user(
        &self,
        external_id: Uuid,
        username: Option<String>,
        password: Option<String>,
        is_active: Option<bool>
    ) -> Result<Option<User>, Box<dyn Error>> {
        // Validações de domínio
        if let Some(ref username) = username {
            if username.is_empty() {
                return Err("Username cannot be empty".into());
            }
        }

        // Hash da senha se fornecida
        let password_hash = match password {
            Some(pwd) => {
                if pwd.len() < 6 {
                    return Err("Password must be at least 6 characters".into());
                }
                Some(hash(pwd, DEFAULT_COST)?)
            },
            None => None,
        };

        // Atualizar o usuário
        self.user_repository.update(external_id, username, password_hash, is_active).await
    }

    pub async fn delete_user(&self, external_id: Uuid) -> Result<bool, Box<dyn Error>> {
        self.user_repository.delete(external_id).await
    }
}
