use actix_web::web;
use crate::infrastructure::http::handlers::user_handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(user_handlers::create_user)
            .service(user_handlers::get_users)
            .service(user_handlers::get_user)
            .service(user_handlers::update_user)
            .service(user_handlers::delete_user)
    );
}

