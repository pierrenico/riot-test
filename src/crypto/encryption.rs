use serde_json::Value;
use super::encoding::{encode, decode};

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