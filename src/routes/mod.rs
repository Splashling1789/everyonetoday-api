use rocket::serde::{json::{json, Value}, Serialize};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Connection;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(sqlx::PgPool);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StatusResponse {
    status: String,
    db_status: DbStatus,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DbStatus {
    ping: String,
    version: Option<u32>,
}

#[get("/health")]
pub async fn health(mut db: rocket_db_pools::Connection<MainDb>) -> Json<StatusResponse> {
    let db_status = match db.ping().await {
        Ok(()) => {
            format!("OK")
        }
        Err(e) => {
            format!("{e}")
        }
    };

    Json(StatusResponse {
        status: format!("OK"),
        db_status: DbStatus {
            ping: db_status,
            version: db.server_version_num()
        }
    })
}
