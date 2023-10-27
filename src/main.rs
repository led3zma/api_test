use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use log::info;

mod api;
use crate::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server...");
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().send_wildcard();
        App::new()
            .wrap(cors)
            .route("/api/message", web::get().to(routes::api))
            .route("/api/random", web::get().to(routes::random))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
