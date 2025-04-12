//! This module handles the core cryptographic operations:
//! - Encoding/decoding for the /encrypt and /decrypt endpoints.
//! - Signing and verification for the /sign and /verify endpoints.
//! It also includes JSON canonicalization logic to ensure signatures are consistent.

mod encoding;
mod signing;
mod json;
mod encryption;

pub use encoding::{encode, decode};
pub use signing::{create_signing_instance, compute, sign_data, verify_signature};
pub use json::canonicalize_json;
pub use encryption::{encrypt_data, decrypt_data};

#[cfg(test)]
mod tests; 