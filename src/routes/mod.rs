pub mod get_health;
pub mod get_posts;
pub mod post_write;

use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Database;
use sqlx::postgres::PgPool;
use sqlx::Decode;

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(PgPool);
