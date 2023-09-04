//! This module manages the routes of the API.
pub mod get_health;
pub mod get_posts;
pub mod post_write;

use rocket_db_pools::Database;
use sqlx::postgres::PgPool;

#[derive(Database)]
#[database("main_db")]
///Structure necessary for Rocket to manage the database pool.
pub struct MainDb(PgPool);
