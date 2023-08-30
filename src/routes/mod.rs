use rocket::serde::json::{json, Value};
use rocket_db_pools::sqlx::Connection;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(sqlx::PgPool);

#[get("/health")]
pub async fn health(mut db: rocket_db_pools::Connection<MainDb>) -> Value {
    let db_status = match db.ping().await {
        Ok(()) => {
            format!("OK")
        }
        Err(e) => {
            format!("{e}")
        }
    };
    json!({
        "status": "OK",
        "db_status": db_status
    })
}
