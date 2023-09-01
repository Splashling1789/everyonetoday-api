use rocket::http::Status;
use rocket::Request;
use crate::routes::{DbStatus, MainDb, HealthStatus};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Connection;
use crate::API_VERSION;

#[catch(default)]
pub fn not_avaliable(status:Status, req: &Request) -> Json<HealthStatus> {
    Json(HealthStatus {
        status: status.code,
        version: API_VERSION,
        db_status: None
    })
}

#[get("/health")]
pub async fn health(mut db: rocket_db_pools::Connection<MainDb>) -> Json<HealthStatus> {

    let db_status = match db.ping().await {
        Ok(()) => {
            format!("OK")
        }
        Err(e) => {
            format!("{e}")
        }
    };

    Json(HealthStatus {
        status: 200,
        version: API_VERSION,
        db_status: Some(
            DbStatus {
            ping: db_status,
            version: db.server_version_num()
        })
    })
}