use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    message: String,
}

async fn index() -> impl Responder {
    web::Json(Info {
        message: "Hello World!".into(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
