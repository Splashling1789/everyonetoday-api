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
    match query(QUERY_WRITE)
        .bind(post.sign.clone())
        .bind(post.quote.clone())
        .execute(&mut *db)
        .await
    {
        Ok(q) => Json(PostWriteResponse {
            success: true,
            error_output: None,
        }),
        Err(e) => Json(PostWriteResponse {
            success: false,
            error_output: Some(format!("{e}")),
        }),
    }
}
