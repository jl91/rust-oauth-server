use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

