use actix_web::{web, App, HttpServer};
use log::info;

mod api;
use crate::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index))
            .route("/api/message", web::get().to(routes::api))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
