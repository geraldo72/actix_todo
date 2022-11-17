use std::env;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    let port = env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("0.0.0.0",port))?
    .run();


    eprintln!("Listening on 0.0.0.0:{}",port);

    server.await

}