#[macro_use] extern crate rocket;


#[get("/health")]
fn health(name: &str, age: u8) -> String {
    format!("status: OK")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health])
}