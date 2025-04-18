use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub external_id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(
        id: i64,
        external_id: Uuid,
        username: String,
        password_hash: String,
        is_active: bool,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        deleted_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            external_id,
            username,
            password_hash,
            is_active,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}