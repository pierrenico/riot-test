use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Helper function to encode data
pub fn encode(data: &[u8]) -> String {
    BASE64.encode(data)
}

/// Helper function to decode data
pub fn decode(data: &str) -> Result<Vec<u8>, String> {
    BASE64.decode(data).map_err(|e| format!("Failed to decode: {}", e))
} 