use super::*;
use serde_json::json;
use std::env;
use dotenvy::dotenv;

// Helper to get the secret key for tests
fn get_test_secret_key() -> Vec<u8> {
    dotenv().ok(); // Load .env if present (useful for local testing)
    env::var("HMAC_SECRET_KEY")
        .expect("HMAC_SECRET_KEY must be set for tests")
        .into_bytes()
}

#[test]
fn test_encrypt_decrypt_simple() {
    let input = json!({
        "name": "John Doe",
        "age": 30,
        "contact": {
            "email": "john@example.com",
            "phone": "123-456-7890"
        }
    });

    // Test encryption
    let encrypted = encrypt_data(&input).unwrap();
    assert!(encrypted.is_object());
    
    // Verify exact encrypted values
    assert_eq!(encrypted["name"], json!("IkpvaG4gRG9lIg=="));  // Base64 of "\"John Doe\""
    assert_eq!(encrypted["age"], json!("MzA="));               // Base64 of "30"
    assert_eq!(encrypted["contact"], json!("eyJlbWFpbCI6ImpvaG5AZXhhbXBsZS5jb20iLCJwaG9uZSI6IjEyMy00NTYtNzg5MCJ9"));  // Base64 of the contact object

    // Test decryption
    let decrypted = decrypt_data(&encrypted).unwrap();
    assert_eq!(decrypted, input);
}

#[test]
fn test_encrypt_decrypt_mixed() {
    let input = json!({
        "name": "Sm9obiBEb2U=",  // Already encrypted
        "age": 30,               // Not encrypted
        "contact": {
            "email": "john@example.com",
            "phone": "123-456-7890"
        }
    });

    // Test encryption
    let encrypted = encrypt_data(&input).unwrap();
    assert!(encrypted.is_object());
    
    // Verify exact encrypted values
    assert_eq!(encrypted["name"], json!("IlNtOW9iaUJFYjJVPSI="));  // Base64 of "\"Sm9obiBEb2U=\""
    assert_eq!(encrypted["age"], json!("MzA="));                   // Base64 of "30"
    assert_eq!(encrypted["contact"], json!("eyJlbWFpbCI6ImpvaG5AZXhhbXBsZS5jb20iLCJwaG9uZSI6IjEyMy00NTYtNzg5MCJ9"));  // Base64 of the contact object

    // Test decryption
    let decrypted = decrypt_data(&encrypted).unwrap();
    assert_eq!(decrypted["age"], json!(30));
    assert_eq!(decrypted["contact"], input["contact"]);
    assert_eq!(decrypted["name"], json!("Sm9obiBEb2U="));
}

#[test]
fn test_encrypt_invalid_input() {
    let input = json!("not an object");
    let result = encrypt_data(&input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Input must be a JSON object");
}

#[test]
fn test_decrypt_invalid_input() {
    let input = json!("not an object");
    let result = decrypt_data(&input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Input must be a JSON object");
}

#[test]
fn test_decrypt_invalid_base64() {
    let input = json!({
        "name": "not a valid base64 string"
    });
    let result = decrypt_data(&input);
    assert!(result.is_ok());
    // Invalid base64 strings should be preserved as-is
    assert_eq!(result.unwrap()["name"], json!("not a valid base64 string"));
}

#[test]
fn test_sign_verify() {
    let input = json!({
        "message": "Hello World",
        "timestamp": 1616161616
    });
    let key = get_test_secret_key();

    // Test signing
    let signature = sign_data(&input, &key).unwrap();
    assert!(!signature.is_empty());

    // Test verification with correct signature
    let is_valid = verify_signature(&input, &signature, &key).unwrap();
    assert!(is_valid);

    // Test verification with incorrect signature
    let is_valid = verify_signature(&input, "invalid_signature", &key).unwrap();
    assert!(!is_valid);
}

#[test]
fn test_signature_consistency() {
    let input1 = json!({
        "message": "Hello World",
        "timestamp": 1616161616
    });
    let input2 = json!({
        "timestamp": 1616161616,
        "message": "Hello World"
    });
    let key = get_test_secret_key();

    // Signatures should be the same regardless of property order
    let signature1 = sign_data(&input1, &key).unwrap();
    let signature2 = sign_data(&input2, &key).unwrap();
    assert_eq!(signature1, signature2);
}

#[test]
fn test_sign_verify_nested() {
    let input = json!({
        "user": {
            "name": "John Doe",
            "age": 30
        },
        "metadata": {
            "created_at": 1616161616,
            "updated_at": 1616161617
        }
    });
    let key = get_test_secret_key();

    // Test signing
    let signature = sign_data(&input, &key).unwrap();
    assert!(!signature.is_empty());

    // Test verification
    let is_valid = verify_signature(&input, &signature, &key).unwrap();
    assert!(is_valid);
} 