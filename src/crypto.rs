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

// Secret key for HMAC - in a real application, this should be properly managed
const SECRET_KEY: &[u8] = b"your-secret-key-here";

/// Helper function to encode data
fn encode(data: &[u8]) -> String {
    BASE64.encode(data)
}

/// Helper function to decode data
fn decode(data: &str) -> Result<Vec<u8>, String> {
    BASE64.decode(data).map_err(|e| format!("Failed to decode: {}", e))
}

/// Helper function to create a new instance
fn create_signing_instance() -> Result<HmacSha256, String> {
    HmacSha256::new_from_slice(SECRET_KEY)
        .map_err(|e| format!("Failed to create: {}", e))
}

/// Helper function to compute signature
fn compute(data: &[u8]) -> Result<String, String> {
    let mut instance = create_signing_instance()?;
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

pub fn sign_data(data: &Value) -> Result<String, String> {
    // Canonicalize the JSON to ensure consistent property ordering
    let canonical = canonicalize_json(data);
    
    // Convert to string for hashing
    let json_str = canonical.to_string();
    
    // Compute signature
    compute(json_str.as_bytes())
}

pub fn verify_signature(data: &Value, signature: &str) -> Result<bool, String> {
    // Generate signature for the provided data
    let expected_signature = sign_data(data)?;
    
    // Compare the signatures
    Ok(expected_signature == signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

        // Test signing
        let signature = sign_data(&input).unwrap();
        assert!(!signature.is_empty());

        // Test verification with correct signature
        let is_valid = verify_signature(&input, &signature).unwrap();
        assert!(is_valid);

        // Test verification with incorrect signature
        let is_valid = verify_signature(&input, "invalid-signature").unwrap();
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

        // Signatures should be the same regardless of property order
        let signature1 = sign_data(&input1).unwrap();
        let signature2 = sign_data(&input2).unwrap();
        assert_eq!(signature1, signature2);
    }
}
