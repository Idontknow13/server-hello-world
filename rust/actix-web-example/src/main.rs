use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

type IOResult<T> = std::io::Result<T>;

const LOCALHOST: (&str, u16) = ("127.0.0.1", 8080);

#[actix_web::main]
async fn main() -> IOResult<()> {
    println!("Running server at {}:{}...", LOCALHOST.0, LOCALHOST.1);

    HttpServer::new(|| App::new().service(greet))
        .bind(LOCALHOST)?
        .run()
        .await
}

#[derive(Deserialize)]
struct Greeter {
    name: Option<String>,
}

#[get("/")]
async fn greet(web::Query(info): web::Query<Greeter>) -> impl Responder {
    let name = info.name.unwrap_or_else(|| String::from("World"));
    HttpResponse::Ok().body(format!("Hello, {name}!\n"))
}
