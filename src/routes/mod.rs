pub mod get_health;
pub mod get_posts;

use chrono::Utc;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Database;
use sqlx::postgres::PgPool;
use sqlx::types::chrono::DateTime;
use sqlx::Decode;

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(PgPool);

#[derive(Decode, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    sign: String,
    quote: String,
    date: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetPosts {
    list: Option<Vec<Post>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetHealth {
    status: u16,
    description: &'static str,
    version: &'static str,
    db_status: Option<DbStatus>,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DbStatus {
    ping: String,
    version: Option<u32>,
}
