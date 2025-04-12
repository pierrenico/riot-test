//! Defines the Actix route handlers for the API endpoints.
//! Each function corresponds to an API endpoint and handles request
//! processing, calls the appropriate cryptographic functions, and
//! constructs the HTTP response.

use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use crate::crypto::{encrypt_data, decrypt_data, sign_data, verify_signature};
use crate::models::VerifyRequest;
use log::{info, warn, error};

/// Handles POST requests to `/encrypt`.
///
/// Takes a JSON object in the request body, encrypts its top-level values
/// using Base64 encoding, and returns the modified JSON object.
///
/// # Errors
/// Returns a 400 Bad Request if the input is not a valid JSON object or if
/// encryption fails internally.
pub async fn encrypt(data: web::Json<Value>) -> impl Responder {
    info!("Received encryption request");
    match encrypt_data(&data.into_inner()) {
        Ok(encrypted) => {
            info!("Successfully encrypted data");
            HttpResponse::Ok().json(encrypted)
        },
        Err(e) => {
            error!("Encryption failed internally: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Encryption failed"
            }))
        }
    }
}

/// Handles POST requests to `/decrypt`.
///
/// Takes a JSON object in the request body, attempts to decrypt any Base64-encoded
/// string values at the top level, and returns the modified JSON object.
/// Non-string values or strings that are not valid Base64 are preserved.
///
/// # Errors
/// Returns a 400 Bad Request if the input is not a valid JSON object or if
/// decryption fails internally (e.g., decoding error).
pub async fn decrypt(data: web::Json<Value>) -> impl Responder {
    info!("Received decryption request");
    match decrypt_data(&data.into_inner()) {
        Ok(decrypted) => {
            info!("Successfully decrypted data");
            HttpResponse::Ok().json(decrypted)
        },
        Err(e) => {
            error!("Decryption failed internally: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Decryption failed"
            }))
        }
    }
}

/// Handles POST requests to `/sign`.
///
/// Takes a JSON object in the request body, generates an HMAC-SHA256 signature
/// based on its canonical representation, and returns the signature in a JSON object.
/// The HMAC secret key is retrieved from application data.
///
/// Importantly it ensures key ordering can be arbitrary.
/// 
/// # Errors
/// Returns a 400 Bad Request if signing fails internally.
/// Returns a 500 Internal Server Error if the secret key is missing in app data.
pub async fn sign(data: web::Json<Value>, hmac_key: web::Data<Vec<u8>>) -> impl Responder {
    info!("Received signing request");
    let key = hmac_key.get_ref();
    match sign_data(&data.into_inner(), key) {
        Ok(signature) => {
            info!("Successfully generated signature");
            HttpResponse::Ok().json(serde_json::json!({
                "signature": signature
            }))
        },
        Err(e) => {
            error!("Signing failed internally: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Signing failed"
            }))
        }
    }
}

/// Handles POST requests to `/verify`.
///
/// Takes a JSON object containing `data` and `signature` fields. It verifies
/// if the provided signature matches the expected HMAC-SHA256 signature for the `data`.
/// The HMAC secret key is retrieved from application data.
///
/// Importantly it expects arbitrary key ordering.
/// 
/// # Responses
/// - `204 No Content`: If the signature is valid.
/// - `400 Bad Request`: If the signature is invalid or if verification fails internally.
/// - `500 Internal Server Error`: If the secret key is missing in app data.
pub async fn verify(data: web::Json<VerifyRequest>, hmac_key: web::Data<Vec<u8>>) -> impl Responder {
    info!("Received verification request");
    let verify_request = data.into_inner();
    let key = hmac_key.get_ref();
    match verify_signature(&verify_request.data, &verify_request.signature, key) {
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
            error!("Signature verification failed internally: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Verification failed"
            }))
        }
    }
}
