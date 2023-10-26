use std::fs;

use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    message: String,
}

pub async fn api() -> impl Responder {
    info!("Handling index request!");
    web::Json(Info {
        message: "Hello World!".into(),
    })
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(fs::read_to_string("frontend/index.html").unwrap())
}
