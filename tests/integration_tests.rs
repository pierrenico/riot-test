use actix_web::{test, web, App};
use serde_json::json;
use riot_api::routes;
use riot_api::models::VerifyRequest;

#[actix_web::test]
async fn test_encrypt_decrypt_flow() {
    let app = test::init_service(
        App::new()
            .route("/encrypt", web::post().to(routes::encrypt))
            .route("/decrypt", web::post().to(routes::decrypt))
            .route("/sign", web::post().to(routes::sign))
            .route("/verify", web::post().to(routes::verify))
    ).await;

    // Test data
    let test_data = json!({
        "name": "John Doe",
        "age": 30,
        "contact": {
            "email": "john@example.com",
            "phone": "123-456-7890"
        }
    });

    // Test encryption
    let req = test::TestRequest::post()
        .uri("/encrypt")
        .set_json(&test_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let encrypted_data: serde_json::Value = test::read_body_json(resp).await;

    // Test decryption
    let req = test::TestRequest::post()
        .uri("/decrypt")
        .set_json(&encrypted_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let decrypted_data: serde_json::Value = test::read_body_json(resp).await;

    // Verify data integrity
    assert_eq!(test_data, decrypted_data);
}

#[actix_web::test]
async fn test_sign_verify_flow() {
    let app = test::init_service(
        App::new()
            .route("/encrypt", web::post().to(routes::encrypt))
            .route("/decrypt", web::post().to(routes::decrypt))
            .route("/sign", web::post().to(routes::sign))
            .route("/verify", web::post().to(routes::verify))
    ).await;

    // Test data
    let test_data = json!({
        "message": "Hello World",
        "timestamp": 1616161616
    });

    // Test signing
    let req = test::TestRequest::post()
        .uri("/sign")
        .set_json(&test_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let signed_data: serde_json::Value = test::read_body_json(resp).await;

    // Test verification
    let verify_data = VerifyRequest {
        data: test_data,
        signature: signed_data["signature"].as_str().unwrap().to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/verify")
        .set_json(&verify_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204);
}

#[actix_web::test]
async fn test_invalid_verification() {
    let app = test::init_service(
        App::new()
            .route("/encrypt", web::post().to(routes::encrypt))
            .route("/decrypt", web::post().to(routes::decrypt))
            .route("/sign", web::post().to(routes::sign))
            .route("/verify", web::post().to(routes::verify))
    ).await;

    // Test data with invalid signature
    let verify_data = VerifyRequest {
        data: json!({
            "message": "Hello World",
            "timestamp": 1616161616
        }),
        signature: "invalid_signature".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/verify")
        .set_json(&verify_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_invalid_json_input() {
    let app = test::init_service(
        App::new()
            .route("/encrypt", web::post().to(routes::encrypt))
            .route("/decrypt", web::post().to(routes::decrypt))
            .route("/sign", web::post().to(routes::sign))
            .route("/verify", web::post().to(routes::verify))
    ).await;

    // Test with invalid JSON
    let req = test::TestRequest::post()
        .uri("/encrypt")
        .set_payload("invalid json")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_empty_json_input() {
    let app = test::init_service(
        App::new()
            .route("/encrypt", web::post().to(routes::encrypt))
            .route("/decrypt", web::post().to(routes::decrypt))
            .route("/sign", web::post().to(routes::sign))
            .route("/verify", web::post().to(routes::verify))
    ).await;

    // Test with empty JSON
    let req = test::TestRequest::post()
        .uri("/encrypt")
        .set_json(&json!({}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response, json!({}));
} 