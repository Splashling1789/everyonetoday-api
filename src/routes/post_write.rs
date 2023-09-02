use rocket::serde::json::Json;
use crate::routes::PostWrite_data;

#[post("/write"), data=<post>]
pub async fn write(post: Json<PostWrite_data>) {

}