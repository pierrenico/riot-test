use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub data: serde_json::Value,
    pub signature: String,
}
