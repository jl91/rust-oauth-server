use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use actix_web::web;

use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db::connection::DbPool;
use crate::infrastructure::db::schema::users;

// Diesel models
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
struct UserModel {
    pub id: i64,
    pub external_id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUserModel {
    pub username: String,
    pub password_hash: String,
    pub is_active: bool,
}

// Converters
impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        User::new(
            model.id,
            model.external_id,
            model.username,
            model.password_hash,
            model.is_active,
            model.created_at,
            model.updated_at,
            model.deleted_at,
        )
    }
}

#[derive(Clone)]
pub struct DieselUserRepository {
    pool: Arc<DbPool>,
}

impl DieselUserRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for DieselUserRepository {
    async fn create(&self, username: String, password_hash: String, is_active: bool) -> Result<User, Box<dyn Error>> {
        let new_user = NewUserModel {
            username,
            password_hash,
            is_active,
        };

        let pool = self.pool.clone();

        let user_model = web::block(move || {
            let mut conn = pool.get().map_err(|e| Box::new(e) as Box<dyn Error>)?;

            diesel::insert_into(users::table)
                .values(&new_user)
                .returning(UserModel::as_returning())
                .get_result(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
            .await?
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(User::from(user_model))
    }

    async fn find_by_external_id(&self, external_id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let pool = self.pool.clone();
        let user_id = external_id;

        let user_model = web::block(move || {
            let mut conn = pool.get().map_err(|e| Box::new(e) as Box<dyn Error>)?;

            users::table
                .filter(users::external_id.eq(user_id))
                .filter(users::deleted_at.is_null())
                .first::<UserModel>(&mut conn)
                .optional()
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
            .await?
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(user_model.map(User::from))
    }

    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let pool = self.pool.clone();

        let user_models = web::block(move || {
            let mut conn = pool.get().map_err(|e| Box::new(e) as Box<dyn Error>)?;

            users::table
                .filter(users::deleted_at.is_null())
                .load::<UserModel>(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
            .await?
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(user_models.into_iter().map(User::from).collect())
    }

    async fn update(
        &self,
        external_id: Uuid,
        username: Option<String>,
        password_hash: Option<String>,
        is_active: Option<bool>
    ) -> Result<Option<User>, Box<dyn Error>> {
        let pool = self.pool.clone();
        let user_id = external_id;

        let user_model = web::block(move || {
            let mut conn = pool.get().map_err(|e| Box::new(e) as Box<dyn Error>)?;

            // Encontrar o usuário primeiro para verificar se existe
            let target = users::table
                .filter(users::external_id.eq(user_id))
                .filter(users::deleted_at.is_null());

            // Construir o update dinamicamente
            let mut update_operation = diesel::update(target);

            // Definir campos a serem atualizados

            if let Some(username_value) = username {
                update_operation = update_operation.set(users::username.eq(username_value));
            }

            if let Some(password_hash_value) = password_hash {
                update_operation = update_operation.set(users::password_hash.eq(password_hash_value));
            }

            if let Some(is_active_value) = is_active {
                update_operation = update_operation.set(users::is_active.eq(is_active_value));
            }

            // Executar o update
            update_operation
                .returning(UserModel::as_returning())
                .get_result(&mut conn)
                .optional()
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
            .await?
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        Ok(user_model.map(User::from))
    }

    async fn delete(&self, external_id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let pool = self.pool.clone();
        let user_id = external_id;

        let deleted_count = web::block(move || {
            let mut conn = pool.get().map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

            let now = Utc::now().naive_utc();
            let target = users::table
                .filter(users::external_id.eq(user_id))
                .filter(users::deleted_at.is_null());

            diesel::update(target)
                .set(users::deleted_at.eq(now))
                .execute(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        })
            .await?
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        Ok(deleted_count > 0)
    }
}