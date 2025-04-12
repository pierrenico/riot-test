//! This module handles the core cryptographic operations:
//! - Base64 encoding/decoding for the /encrypt and /decrypt endpoints.
//! - HMAC-SHA256 signing and verification for the /sign and /verify endpoints.
//! It also includes JSON canonicalization logic to ensure signatures are consistent.

use serde_json::Value;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

/// Helper function to encode data
fn encode(data: &[u8]) -> String {
    BASE64.encode(data)
}

/// Helper function to decode data
fn decode(data: &str) -> Result<Vec<u8>, String> {
    BASE64.decode(data).map_err(|e| format!("Failed to decode: {}", e))
}

/// Helper function to create a new instance
fn create_signing_instance(secret_key: &[u8]) -> Result<HmacSha256, String> {
    HmacSha256::new_from_slice(secret_key)
        .map_err(|e| format!("Failed to create: {}", e))
}

/// Helper function to compute signature
fn compute(data: &[u8], secret_key: &[u8]) -> Result<String, String> {
    let mut instance = create_signing_instance(secret_key)?;
    instance.update(data);
    let result = instance.finalize();
    Ok(encode(&result.into_bytes()))
}

pub fn encrypt_data(data: &Value) -> Result<Value, String> {
    match data {
        Value::Object(obj) => {
            let mut result = serde_json::Map::new();
            for (key, value) in obj {
                // Convert the value to a string and encode it
                let encoded = encode(&value.to_string().into_bytes());
                result.insert(key.clone(), Value::String(encoded));
            }
            Ok(Value::Object(result))
        }
        _ => Err("Input must be a JSON object".to_string()),
    }
}

pub fn decrypt_data(data: &Value) -> Result<Value, String> {
    match data {
        Value::Object(obj) => {
            let mut result = serde_json::Map::new();
            for (key, value) in obj {
                match value {
                    Value::String(s) => {
                        // Try to decode the string
                        if let Ok(decoded_bytes) = decode(s) {
                            if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                                // Try to parse the decoded string back into a JSON value
                                if let Ok(parsed_value) = serde_json::from_str::<Value>(&decoded_str) {
                                    result.insert(key.clone(), parsed_value);
                                } else {
                                    // If it's not valid JSON, keep it as a string
                                    result.insert(key.clone(), Value::String(decoded_str));
                                }
                            } else {
                                // If it's not valid UTF-8, keep the original value
                                result.insert(key.clone(), value.clone());
                            }
                        } else {
                            // If it's not valid base64, keep the original value
                            result.insert(key.clone(), value.clone());
                        }
                    }
                    _ => {
                        // Keep non-string values as is
                        result.insert(key.clone(), value.clone());
                    }
                }
            }
            Ok(Value::Object(result))
        }
        _ => Err("Input must be a JSON object".to_string()),
    }
}

/// Recursively sorts JSON objects by key to create a canonical representation.
///
/// This is crucial for the signing process (/sign and /verify) to ensure that
/// the signature remains consistent even if the order of properties in the
/// input JSON object changes.
/// Arrays are processed element by element, but their order is preserved.
/// Other JSON types (String, Number, Boolean, Null) are returned as is.
fn canonicalize_json(value: &Value) -> Value {
    match value {
        Value::Object(obj) => {
            let mut sorted = BTreeMap::new();
            for (key, value) in obj {
                sorted.insert(key.clone(), canonicalize_json(value));
            }
            // Convert BTreeMap to serde_json::Map
            let mut map = serde_json::Map::new();
            for (key, value) in sorted {
                map.insert(key, value);
            }
            Value::Object(map)
        }
        Value::Array(arr) => {
            let mut sorted = Vec::new();
            for value in arr {
                sorted.push(canonicalize_json(value));
            }
            Value::Array(sorted)
        }
        _ => value.clone(),
    }
}

pub fn sign_data(data: &Value, secret_key: &[u8]) -> Result<String, String> {
    // Canonicalize the JSON to ensure consistent property ordering
    let canonical = canonicalize_json(data);
    
    // Convert to string for hashing
    let json_str = canonical.to_string();
    
    // Compute signature
    compute(json_str.as_bytes(), secret_key)
}

pub fn verify_signature(data: &Value, signature: &str, secret_key: &[u8]) -> Result<bool, String> {
    // Generate signature for the provided data
    let expected_signature = sign_data(data, secret_key)?;
    
    // Compare the signatures
    Ok(expected_signature == signature)
}

#[cfg(test)]
mod tests {
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
        let is_valid = verify_signature(&input, "invalid-signature", &key).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_signature_consistency() {
        let json1 = json!({
            "name": "Alice",
            "age": 30
        });
        let json2 = json!({
            "age": 30,
            "name": "Alice"
        });
        let key = get_test_secret_key();

        let sig1 = sign_data(&json1, &key).unwrap();
        let sig2 = sign_data(&json2, &key).unwrap();

        assert_eq!(sig1, sig2);

        let is_valid1 = verify_signature(&json1, &sig1, &key).unwrap();
        let is_valid2 = verify_signature(&json2, &sig1, &key).unwrap();
        assert!(is_valid1);
        assert!(is_valid2);
    }

    #[test]
    fn test_sign_verify_nested() {
        let input = json!({
            "user": {
                "id": 123,
                "details": {
                    "name": "Bob"
                }
            },
            "timestamp": 1616161617
        });
        let key = get_test_secret_key();

        // Test signing
        let signature = sign_data(&input, &key).unwrap();
        assert!(!signature.is_empty());

        // Test verification
        let is_valid = verify_signature(&input, &signature, &key).unwrap();
        assert!(is_valid);
    }
}
