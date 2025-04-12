use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;

pub async fn encrypt(data: web::Json<Value>) -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Not implemented yet"
    }))
}

pub async fn decrypt(data: web::Json<Value>) -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Not implemented yet"
    }))
}

pub async fn sign(data: web::Json<Value>) -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Not implemented yet"
    }))
}

pub async fn verify(data: web::Json<Value>) -> impl Responder {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Not implemented yet"
    }))
}
