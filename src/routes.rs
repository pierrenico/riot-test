use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use crate::crypto::{encrypt_data, decrypt_data, sign_data, verify_signature};
use crate::models::VerifyRequest;
use log::{info, warn};

pub async fn encrypt(data: web::Json<Value>) -> impl Responder {
    info!("Received encryption request");
    match encrypt_data(&data.into_inner()) {
        Ok(encrypted) => {
            info!("Successfully encrypted data");
            HttpResponse::Ok().json(encrypted)
        },
        Err(e) => {
            warn!("Encryption failed: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e
            }))
        }
    }
}

pub async fn decrypt(data: web::Json<Value>) -> impl Responder {
    info!("Received decryption request");
    match decrypt_data(&data.into_inner()) {
        Ok(decrypted) => {
            info!("Successfully decrypted data");
            HttpResponse::Ok().json(decrypted)
        },
        Err(e) => {
            warn!("Decryption failed: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e
            }))
        }
    }
}

pub async fn sign(data: web::Json<Value>) -> impl Responder {
    info!("Received signing request");
    match sign_data(&data.into_inner()) {
        Ok(signature) => {
            info!("Successfully generated signature");
            HttpResponse::Ok().json(serde_json::json!({
                "signature": signature
            }))
        },
        Err(e) => {
            warn!("Signing failed: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e
            }))
        }
    }
}

pub async fn verify(data: web::Json<VerifyRequest>) -> impl Responder {
    info!("Received verification request");
    let verify_request = data.into_inner();
    match verify_signature(&verify_request.data, &verify_request.signature) {
        Ok(true) => {
            info!("Signature verification successful");
            HttpResponse::NoContent().finish()
        },
        Ok(false) => {
            warn!("Signature verification failed: Invalid signature");
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid signature"
            }))
        },
        Err(e) => {
            warn!("Signature verification failed: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e
            }))
        }
    }
}
