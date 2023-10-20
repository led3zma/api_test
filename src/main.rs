use actix_web::{web, App, HttpServer, Responder};
use log::info;
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    message: String,
}

async fn index() -> impl Responder {
    info!("Handling index request!");
    web::Json(Info {
        message: "Hello World!".into(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server...");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
