#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/?<name>")]
fn greet(name: Option<String>) -> String {
    name.map(|name| format!("Hello, {name}!\n"))
        .unwrap_or_else(|| String::from("Hello, World!\n"))
}

fn main() {
    rocket::ignite().mount("/", routes![greet]).launch();
}
