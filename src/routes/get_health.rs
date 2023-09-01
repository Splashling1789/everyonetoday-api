use crate::routes::{DbStatus, MainDb, HealthStatus};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Connection;
use crate::API_VERSION;
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
        status: format!("OK"),
        version: API_VERSION,
        db_status: DbStatus {
            ping: db_status,
            version: db.server_version_num()
        }
    })
}