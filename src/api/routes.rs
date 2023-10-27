use actix_web::{web, HttpRequest, Responder};
use log::info;
use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
struct Info {
    message: String,
}

pub async fn api(req: HttpRequest) -> impl Responder {
    info!("Handling index request!");
    info!(
        "Request: Origin {:?}, Referer {:?}",
        req.headers().get("origin"),
        req.headers().get("Referer")
    );
    if let Some(_) = req.headers().get("origin") {
        return web::Json(Info {
            message: "Hello World!".into(),
        });
    }

    web::Json(Info { message: "".into() })
}

pub async fn random() -> impl Responder {
    info!("Handling random number request!");
    let mut rng = rand::thread_rng();
    web::Json(Info {
        message: rng.gen_range(0..100).to_string(),
    })
}
