use crate::routes::{GetPosts, MainDb, Post};
use chrono::{DateTime, Utc};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::{query, Row};
use rocket_db_pools::Connection;

const QUERY_GET_POSTS: &str = "SELECT * FROM quotes;";
#[get("/posts")]
pub async fn posts(db: Connection<MainDb>) -> Json<GetPosts> {
    Json(GetPosts {
        list: query_exe(db).await,
    })
}

async fn query_exe(mut conn: Connection<MainDb>) -> Option<Vec<Post>> {
    match query(QUERY_GET_POSTS).fetch_all(&mut *conn).await {
        Ok(table) => {
            let mut result: Vec<Post> = vec![];
            for row in table {
                result.push(Post {
                    date: row.try_get::<DateTime<Utc>, _>("date").expect("Error"),
                    sign: row.try_get::<String, _>("sign").expect("Error"),
                    quote: row.try_get::<String, _>("quote").expect("Error"),
                });
            }
            Some(result)
        }
        Err(_) => None,
    }
}
