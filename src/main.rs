use std::fs;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    message: String,
}

async fn api() -> impl Responder {
    info!("Handling index request!");
    web::Json(Info {
        message: "Hello World!".into(),
    })
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(fs::read_to_string("frontend/index.html").unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/message", web::get().to(api))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
