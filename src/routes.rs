use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use crate::crypto::{encrypt_data, decrypt_data};

pub async fn encrypt(data: web::Json<Value>) -> impl Responder {
    match encrypt_data(&data.into_inner()) {
        Ok(encrypted) => HttpResponse::Ok().json(encrypted),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))
    }
}

pub async fn decrypt(data: web::Json<Value>) -> impl Responder {
    match decrypt_data(&data.into_inner()) {
        Ok(decrypted) => HttpResponse::Ok().json(decrypted),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))
    }
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
