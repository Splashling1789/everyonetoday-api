use crate::routes::{DbStatus, GetHealth, MainDb};
use crate::API_VERSION;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_db_pools::sqlx::Connection;

#[catch(default)]
pub fn not_avaliable(status: Status, req: &Request) -> Json<GetHealth> {
    Json(GetHealth {
        status: status.code,
        description: format!("Yet to implement"),
        version: API_VERSION,
        db_status: None,
    })
}

#[get("/health")]
pub async fn health(mut db: rocket_db_pools::Connection<MainDb>) -> Json<GetHealth> {
    let db_status = match db.ping().await {
        Ok(()) => {
            format!("OK")
        }
        Err(e) => {
            format!("{e}")
        }
    };

    Json(GetHealth {
        status: 200,
        description: format!("Success"),
        version: API_VERSION,
        db_status: Some(DbStatus {
            ping: db_status,
            version: db.server_version_num(),
        }),
    })
}
