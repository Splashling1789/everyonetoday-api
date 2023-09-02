use crate::routes::MainDb;
use crate::API_VERSION;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::Request;
use rocket_db_pools::sqlx::Connection;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetHealth {
    status: u16,
    description: &'static str,
    version: &'static str,
    db_status: Option<DbStatus>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct DbStatus {
    ping: String,
    version: Option<u32>,
}

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
