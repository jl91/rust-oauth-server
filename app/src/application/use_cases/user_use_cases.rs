use crate::application::dtos::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::domain::services::user_service::UserService;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserUseCases {
    user_service: Arc<UserService>,
}

impl UserUseCases {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse, Box<dyn Error>> {
        let user = self.user_service
            .create_user(request.username, request.password, request.is_active)
            .await?;

        Ok(UserResponse::from(user))
    }

    pub async fn get_user(&self, external_id: Uuid) -> Result<Option<UserResponse>, Box<dyn Error>> {
        let user_option = self.user_service.get_user(external_id).await?;

        match user_option {
            Some(user) => Ok(Some(UserResponse::from(user))),
            None => Ok(None),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, Box<dyn Error>> {
        let users = self.user_service.get_all_users().await?;

        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn update_user(&self, external_id: Uuid, request: UpdateUserRequest) -> Result<Option<UserResponse>, Box<dyn Error>> {
        let updated_user = self.user_service
            .update_user(external_id, request.username, request.password, request.is_active)
            .await?;

        match updated_user {
            Some(user) => Ok(Some(UserResponse::from(user))),
            None => Ok(None),
        }
    }

    pub async fn delete_user(&self, external_id: Uuid) -> Result<bool, Box<dyn Error>> {
        self.user_service.delete_user(external_id).await
    }
}
