//! This module manages the routes of the API.
//! # GET Routes:
//! * [/health][get_health]
//! * [/posts][get_posts]
//! # POST Routes:
//! * [/write][post_write]
pub mod get_health;
pub mod get_posts;
pub mod post_write;

use rocket_db_pools::Database;
use sqlx::postgres::PgPool;

#[derive(Database)]
#[database("main_db")]
///Structure necessary for Rocket to manage the database pool.
pub struct MainDb(PgPool);
