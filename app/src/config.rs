use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .expect("PORT must be a number"),
        }
    }
}
