///This module
pub mod get_health;
pub mod get_posts;
pub mod post_write;

use rocket_db_pools::Database;
use sqlx::postgres::PgPool;

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(PgPool);
