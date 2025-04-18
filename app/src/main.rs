mod application;
mod config;
mod domain;
mod infrastructure;

use crate::config::AppConfig;
use crate::infrastructure::db::connection::establish_connection_pool;
use crate::infrastructure::http::routes::configure_routes;
use actix_web::{App, HttpServer, middleware};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;

// Define embedded migrations
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar configurações e logger
    let config = AppConfig::from_env();
    env_logger::init();

    // Set up database connection pool
    let pool = establish_connection_pool(&config.database_url);

    // Run migrations
    // {
    //     let mut conn = pool.get().expect("Failed to get DB connection from pool");
    //     conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
    // }

    let pool = Arc::new(pool);

    // Inicializar repositório
    let user_repository = infrastructure::db::repositories::diesel_user_repository::DieselUserRepository::new(pool.clone());

    // Inicializar serviço
    let user_service = domain::services::user_service::UserService::new(Arc::new(user_repository));

    // Inicializar casos de uso
    let user_use_cases = application::use_cases::user_use_cases::UserUseCases::new(Arc::new(user_service));

    // Start HTTP server
    log::info!("Starting server at http://{}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(user_use_cases.clone()))
            .wrap(middleware::Logger::default())
            .configure(configure_routes)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await
}