use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::application::dtos::user_dto::{CreateUserRequest, UpdateUserRequest};
use crate::application::use_cases::user_use_cases::UserUseCases;

#[post("/users")]
pub async fn create_user(
    use_cases: web::Data<UserUseCases>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    match use_cases.create_user(user_data.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            log::error!("Error creating user: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[get("/users")]
pub async fn get_users(use_cases: web::Data<UserUseCases>) -> impl Responder {
    match use_cases.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            log::error!("Error getting users: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[get("/users/{user_id}")]
pub async fn get_user(
    use_cases: web::Data<UserUseCases>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();

    match use_cases.get_user(user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!("Error getting user: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[put("/users/{user_id}")]
pub async fn update_user(
    use_cases: web::Data<UserUseCases>,
    path: web::Path<Uuid>,
    user_data: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let user_id = path.into_inner();

    match use_cases.update_user(user_id, user_data.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!("Error updating user: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    use_cases: web::Data<UserUseCases>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();

    match use_cases.delete_user(user_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!("Error deleting user: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}
