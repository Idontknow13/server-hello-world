// #[macro_use] extern crate rocket;
use rocket::{get, launch, routes};

#[get("/?<name>")]
fn greet(name: Option<&str>) -> String {
    let name = name.unwrap_or("World");
    format!("Hello, {name}!\n")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![greet])
}
