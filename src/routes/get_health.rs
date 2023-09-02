use crate::routes::{DbStatus, GetHealth, MainDb};
use crate::API_VERSION;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_db_pools::sqlx::Connection;

#[catch(default)]
pub fn not_available(status: Status, _req: &Request) -> Json<GetHealth> {
    //!Returns a ``GetHealth`` response in case there's an error.
    Json(GetHealth {
        status: status.code,
        description: Status::from_code(status.code)
            .unwrap_or(Status::Conflict)
            .reason_lossy(),
        version: API_VERSION,
        db_status: None,
    })
}

#[get("/health")]
pub async fn health(mut db: rocket_db_pools::Connection<MainDb>) -> Json<GetHealth> {
    //!Returns a ``GetHealth`` response, .
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
        description: Status::from_code(200).unwrap().reason_lossy(),
        version: API_VERSION,
        db_status: Some(DbStatus {
            ping: db_status,
            version: db.server_version_num(),
        }),
    })
}
