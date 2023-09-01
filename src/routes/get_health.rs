use crate::routes::{DbStatus, HealthStatus, MainDb};
use crate::API_VERSION;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_db_pools::sqlx::Connection;

#[catch(default)]
pub fn not_avaliable(status: Status, req: &Request) -> Json<HealthStatus> {
    Json(HealthStatus {
        status: status.code,
        version: API_VERSION,
        db_status: None,
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
        db_status: Some(DbStatus {
            ping: db_status,
            version: db.server_version_num(),
        }),
    })
}
