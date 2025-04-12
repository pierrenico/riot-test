// src/models.rs

//! Defines data structures used for API request and response bodies.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Only the `/verify` uses this, as others simply use `serde_json::Value`.
/// Represents the expected JSON structure for the `/verify` endpoint request body.
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    /// The original data that was signed.
    pub data: Value,
    /// The HMAC signature to be verified.
    pub signature: String,
}
