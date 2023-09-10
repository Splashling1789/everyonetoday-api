//! # GET: /health
//! This will return a status of the API.
//!
//! # Response:
//! This route's response uses the [`GetHealthResponse`] structure [serialization][`rocket::serde::Serialize`].
//! * The `"status"` field will give the http status code.
//! * The `"description"` field will provide a description about the http code.
//! * The `"version"` field will provide the version of the rust crate.
//! * The `"db_status"` field will provide a serialization of the [`DbStatus`] structure.
//! It can return None if some error ocurred.
//! Inside this field, `"ping"` should be `"OK"`, and `"version"` will indicate the database server version.
//!
//! A successful response should look like this:
//! ```json
//! {
//!     "status": 200,
//!     "description": "OK",
//!     "version": "API_VERSION",
//!     "db_status": {
//!         "ping": "OK",
//!         "version": 150004
//!     }
//! }
//! ```
//! # Error catcher
//! This module also catches the errors of any route, and returns a [`GetHealthResponse`] with the information.

use crate::routes::MainDb;
use crate::API_VERSION;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::Request;
use rocket_db_pools::sqlx::Connection;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
/// It represents a response to /health
pub struct GetHealthResponse {
    status: u16,
    description: &'static str,
    version: &'static str,
    db_status: Option<DbStatus>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
/// It contains data about database status.
struct DbStatus {
    ping: String,
    version: Option<u32>,
}

#[catch(default)]
pub fn not_available(status: Status, _req: &Request) -> Json<GetHealthResponse> {
    //!Returns a ``GetHealth`` response in case there's an error.
    Json(GetHealthResponse {
        status: status.code,
        description: Status::from_code(status.code)
            .unwrap_or(Status::Conflict)
            .reason_lossy(),
        version: API_VERSION,
        db_status: None,
    })
}

#[get("/health")]
pub async fn health(mut db: rocket_db_pools::Connection<MainDb>) -> Json<GetHealthResponse> {
    //!Returns a ``GetHealth`` response.
    let db_status = match db.ping().await {
        Ok(()) => {
            format!("OK")
        }
        Err(e) => {
            format!("{e}")
        }
    };

    Json(GetHealthResponse {
        status: 200,
        description: Status::from_code(200).unwrap().reason_lossy(),
        version: API_VERSION,
        db_status: Some(DbStatus {
            ping: db_status,
            version: db.server_version_num(),
        }),
    })
}
