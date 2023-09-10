//! # GET: /posts
//! It gets all the quotes that were posted today.
//!
//! # Response
//! This route's response uses the [`GetPosts`] structure [serialization][`rocket::serde::Serialize`].
//! The response should look like this:
//!
//! ```json
//! {
//!     list: [
//!         {
//!            "sign": "Me",
//!            "quote": "Hello world",
//!            "date": "2023-09-04T13:56:05.762733+02:00"
//!         },
//!         {
//!            "sign": "Me again",
//!            "quote": "Hello world!!",
//!            "date": "2023-09-04T13:56:05.762733+02:00"
//!         },
//!     ]
//! }
//! ```
//! If `list` turns out to be null or an empty list, that means there are no quotes posted that day.


use crate::routes::MainDb;
use chrono::{DateTime, Local};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_db_pools::sqlx::{query, Row};
use rocket_db_pools::Connection;

///The query for getting the quotes.
const QUERY_GET_POSTS: &str = "SELECT * FROM quotes WHERE date::date = NOW()::date;";

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
///It represents the /posts json response.
pub struct GetPosts {
    list: Option<Vec<Post>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
///It represents a single quote, with its sign and date.
struct Post {
    sign: String,
    quote: String,
    date: DateTime<Local>,
}

#[get("/posts")]
pub async fn posts(db: Connection<MainDb>) -> Json<GetPosts> {
    //! It gets today's quotes.
    Json(GetPosts {
        list: query_exe(db).await,
    })
}

async fn query_exe(mut conn: Connection<MainDb>) -> Option<Vec<Post>> {
    //!Executes the query [`QUERY_GET_POSTS`] and returns the list of quotes.
    match query(QUERY_GET_POSTS).fetch_all(&mut *conn).await {
        Ok(table) => {
            let mut result: Vec<Post> = vec![];
            for row in table {
                result.push(Post {
                    date: row.try_get::<DateTime<Local>, _>("date").expect("Error"),
                    sign: row.try_get::<String, _>("sign").expect("Error"),
                    quote: row.try_get::<String, _>("quote").expect("Error"),
                });
            }
            Some(result)
        }
        Err(_) => None,
    }
}
