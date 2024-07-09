//! # POST: /write
//! This route is used for "writing" a quote. The POST request is [deserialized][rocket::serde::Deserialize] into a [`PostWriteData`] structure, and should look like this:
//! ```json
//! {
//!     "quote": "The thing you want to say to the world",
//!     "sign": "Who are you"
//! }
//! ```
//! # Response
//! This route's response use the [`PostWriteResponse`] structure serialization.
//! It should return a success similar to this:
//! ```json
//! {
//!     "success": true,
//!     "error_output": null
//! }
//! ```
//! If `success` turns out to be `false`, `error_output` will start with:
//! * `"API ERROR: ..."`, if the database responded correctly but the API didn't accept the request. This can be caused if *the sign alredy exists in today's posts* of if *it contained linebreaks*.
//! * `"DB ERROR: ..."`, if the database did not response correctly.
//!
//! # Example of fail
//! This is an example of a failed request:
//! ```json
//! {
//!     "quote": "I can use linebreaks here? \nOh yes I can!",
//!     "sign": "Can \nI \nuse \nlinebreaks here?\n"
//! }
//! ```
//! Response:
//! ```json
//! {
//!     "success": false,
//!     "error_output": "API ERROR: linebreaks are not allowed in the 'sign' field"
//! }
//! ```
use crate::routes::MainDb;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::query;

///The query for inserting a quote, checking if there is alredy another quote with that sign today.
const QUERY_WRITE: &str = "INSERT INTO quotes (\"date\", \"sign\", \"quote\") SELECT NOW(), $1, $2 WHERE NOT EXISTS (SELECT 1 FROM quotes WHERE date::date = NOW()::date AND \"sign\"=$3);";

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
///It represents the data sent by the POST request at /write
pub struct PostWriteData {
    quote: String,
    sign: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
///It represents the response given by POST /write.
pub struct PostWriteResponse {
    success: bool,
    error_output: Option<String>,
}

#[post("/write", data = "<post>")]
pub async fn write(
    mut db: Connection<MainDb>,
    post: Json<PostWriteData>,
) -> Json<PostWriteResponse> {
    //!It inserts to the database a proper `PostWriteData` if it is valid.
    if has_line_breaks(post.sign.clone()) {
        return Json(PostWriteResponse {
            success: false,
            error_output: Some(format!(
                "API ERROR: linebreaks are not allowed in the 'sign' field"
            )),
        });
    }
    match query(QUERY_WRITE)
        .bind(post.sign.clone())
        .bind(post.quote.clone())
        .bind(post.sign.clone())
        .execute(&mut **db)
        .await
    {
        Ok(q) => {
            if q.rows_affected() == 0 {
                Json(PostWriteResponse {
                    success: false,
                    error_output: Some(format!("API ERROR: that sign has alredy been used today")),
                })
            } else {
                Json(PostWriteResponse {
                    success: true,
                    error_output: None,
                })
            }
        }
        Err(e) => Json(PostWriteResponse {
            success: false,
            error_output: Some(format!("DB ERROR {e}")),
        }),
    }
}

fn has_line_breaks(content: String) -> bool {
    //!It checks if a `String` has linebreaks
    for c in content.chars() {
        if c == 0xA as char {
            return true;
        }
    }
    return false;
}