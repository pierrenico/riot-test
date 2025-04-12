use serde_json::Value;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub fn encrypt_data(data: &Value) -> Result<Value, String> {
    match data {
        Value::Object(obj) => {
            let mut result = serde_json::Map::new();
            for (key, value) in obj {
                // Convert the value to a string and encode it
                let encoded = BASE64.encode(value.to_string());
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
                        if let Ok(decoded_bytes) = BASE64.decode(s) {
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

pub fn sign_data(data: &Value) -> Result<String, String> {
    Err("Not implemented yet".to_string())
}

pub fn verify_signature(data: &Value, signature: &str) -> Result<bool, String> {
    Err("Not implemented yet".to_string())
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
}
