use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod routes;
mod crypto;
mod models;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "a-ok"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/encrypt", web::post().to(routes::encrypt))
            .route("/decrypt", web::post().to(routes::decrypt))
            .route("/sign", web::post().to(routes::sign))
            .route("/verify", web::post().to(routes::verify))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
