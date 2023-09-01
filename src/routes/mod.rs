pub mod get_health;
use rocket::serde::{Serialize};
use rocket_db_pools::sqlx::Connection;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(sqlx::PgPool);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HealthStatus {
    status: String,
    version: &'static str,
    db_status: DbStatus,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DbStatus {
    ping: String,
    version: Option<u32>,
}