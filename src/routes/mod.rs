use std::ops::Deref;
use rocket_db_pools::{Database, sqlx};
use rocket_db_pools::sqlx::Connection;
use rocket::serde::json::{json, Value};

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(sqlx::PgPool);

#[get("/health")]
pub async fn health(mut db: rocket_db_pools::Connection<MainDb>) -> Value {
    let mut db_status = String::default();
    match db.ping().await {
        Ok(()) => {
            db_status = format!("OK");
        }
        Err(e) => {
            db_status = format!("{e}");
        }
    }
    json!({
        "status":"OK",
        "db_status":db_status
    })
}