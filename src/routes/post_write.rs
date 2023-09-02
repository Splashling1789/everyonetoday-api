use crate::routes::MainDb;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::query;

const QUERY_WRITE: &str = "INSERT INTO quotes VALUES (DEFAULT, now(), $1, $2);";

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostWriteData {
    quote: String,
    sign: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostWriteResponse {
    success: bool,
    error_output: Option<String>,
}

#[post("/write", data = "<post>")]
pub async fn write(
    mut db: Connection<MainDb>,
    post: Json<PostWriteData>,
) -> Json<PostWriteResponse> {
    if has_line_breaks(post.sign.clone()) {
        return Json(PostWriteResponse {
            success: false,
            error_output: Some(format!("API ERROR: linebreaks are not allowed in the 'sign' field")),
        });
    }
    match query(QUERY_WRITE)
        .bind(post.sign.clone())
        .bind(post.quote.clone())
        .execute(&mut *db)
        .await
    {
        Ok(_) => Json(PostWriteResponse {
            success: true,
            error_output: None,
        }),
        Err(e) => Json(PostWriteResponse {
            success: false,
            error_output: Some(format!("DB ERROR {e}")),
        }),
    }
}


fn has_line_breaks(content: String) -> bool {
    for c in content.chars() {
        if c == 0xA as char  {
            return true;
        }
    }
    return false;
}