//! Application entry point and server setup.
//! Configures logging, routes, and starts the Actix web server.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;
use std::env;
use dotenvy::dotenv;

pub mod routes;
pub mod crypto;
pub mod models;
pub mod middleware;

/// Simple health check endpoint.
/// Returns a 200 OK response with a JSON body `{"status": "a-ok"}`.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "a-ok"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();

    // Initialize logger with more detailed configuration
    env_logger::Builder::from_env(Env::default()
        .default_filter_or("info")
        .default_write_style_or("always"))
        .format_timestamp_millis()
        .format_module_path(false)
        .init();

    info!("Starting Riot API server...");

    // Read config from environment variables
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let hmac_secret_key = env::var("HMAC_SECRET_KEY")
        .expect("HMAC_SECRET_KEY must be set");

    let hmac_secret_key_bytes = hmac_secret_key.into_bytes(); // Convert to bytes here

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(hmac_secret_key_bytes.clone())) // Store key bytes in app data
            .wrap(middleware::Logger)
            .route("/health", web::get().to(health_check))
            .route("/encrypt", web::post().to(routes::encrypt))
            .route("/decrypt", web::post().to(routes::decrypt))
            .route("/sign", web::post().to(routes::sign))
            .route("/verify", web::post().to(routes::verify))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
