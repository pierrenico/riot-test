use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

/// Helper function to create a new instance
pub fn create_signing_instance(secret_key: &[u8]) -> Result<HmacSha256, String> {
    HmacSha256::new_from_slice(secret_key)
        .map_err(|e| format!("Failed to create: {}", e))
}

/// Helper function to compute signature
pub fn compute(data: &[u8], secret_key: &[u8]) -> Result<String, String> {
    let mut instance = create_signing_instance(secret_key)?;
    instance.update(data);
    let result = instance.finalize();
    Ok(BASE64.encode(&result.into_bytes()))
}

pub fn sign_data(data: &serde_json::Value, secret_key: &[u8]) -> Result<String, String> {
    // Canonicalize the JSON to ensure consistent property ordering
    let canonical = super::json::canonicalize_json(data);
    
    // Convert to string for hashing
    let json_str = canonical.to_string();
    
    // Compute signature
    compute(json_str.as_bytes(), secret_key)
}

pub fn verify_signature(data: &serde_json::Value, signature: &str, secret_key: &[u8]) -> Result<bool, String> {
    // Generate signature for the provided data
    let expected_signature = sign_data(data, secret_key)?;
    
    // Compare the signatures
    Ok(expected_signature == signature)
} 