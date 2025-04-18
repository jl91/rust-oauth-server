use async_trait::async_trait;
use crate::domain::entities::user::User;
use uuid::Uuid;
use std::error::Error;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, username: String, password_hash: String, is_active: bool) -> Result<User, Box<dyn Error>>;
    async fn find_by_external_id(&self, external_id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>>;
    async fn update(&self, external_id: Uuid, username: Option<String>, password_hash: Option<String>, is_active: Option<bool>) -> Result<Option<User>, Box<dyn Error>>;
    async fn delete(&self, external_id: Uuid) -> Result<bool, Box<dyn Error>>;
}