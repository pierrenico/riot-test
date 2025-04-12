use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use crate::crypto::{encrypt_data, decrypt_data, sign_data, verify_signature};
use crate::models::{SignRequest, VerifyRequest};

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
    match sign_data(&data.into_inner()) {
        Ok(signature) => HttpResponse::Ok().json(serde_json::json!({
            "signature": signature
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))
    }
}

pub async fn verify(data: web::Json<VerifyRequest>) -> impl Responder {
    let verify_request = data.into_inner();
    match verify_signature(&verify_request.data, &verify_request.signature) {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid signature"
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))
    }
}
