use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

const LOCALHOST: (&str, u16) = ("127.0.0.1", 8080);

type IOResult<T> = std::io::Result<T>;

#[actix_web::main]
async fn main() -> IOResult<()> {
    // Enable logging
    use env_logger::Env;
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    println!("Running server at {}:{}...", LOCALHOST.0, LOCALHOST.1);

    // View logging semantics [here](https://actix.rs/docs/middleware/).
    HttpServer::new(|| App::new().wrap(Logger::default()).service(greet))
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
