# Implementation Plan

## Phase 1: Project Setup and Basic Structure
- [x] Initialize a new Rust project with Cargo
- [x] Add necessary dependencies:
  - [x] `actix-web` for the web framework
  - [x] `serde` and `serde_json` for JSON handling
  - [x] `base64` for encryption/decryption
  - [x] `hmac` and `sha2` for signing/verification
- [x] Set up basic project structure:
  - [x] `src/main.rs` for application entry point
  - [x] `src/routes.rs` for endpoint definitions
  - [x] `src/crypto.rs` for cryptographic operations
  - [x] `src/models.rs` for data structures

## Phase 2: Encryption/Decryption Implementation
- [x] Implement Base64 encryption/decryption utilities in `crypto.rs`
- [x] Create JSON processing utilities to handle nested structures
- [x] Implement `/encrypt` endpoint:
  - [x] Handle JSON input
  - [x] Encrypt top-level properties
  - [x] Return encrypted JSON
- [x] Implement `/decrypt` endpoint:
  - [x] Handle JSON input
  - [x] Decrypt Base64-encoded properties
  - [x] Return original JSON
- [x] Add unit tests for encryption/decryption functionality

## Phase 3: Signing/Verification Implementation
- [x] Implement HMAC signing utilities in `crypto.rs`
- [x] Create JSON canonicalization for consistent property ordering
- [x] Implement `/sign` endpoint:
  - [x] Handle JSON input
  - [x] Generate HMAC signature
  - [x] Return JSON with signature
- [x] Implement `/verify` endpoint:
  - [x] Handle JSON input with signature and data
  - [x] Verify HMAC signature
  - [x] Return appropriate HTTP status
- [x] Add unit tests for signing/verification functionality

## Phase 4: Integration and Testing
- [x] Add integration tests for all endpoints
- [x] Implement error handling and proper HTTP status codes
- [x] Add input validation
- [x] Create example usage documentation
- [x] Add logging and monitoring

## Phase 5: Refinement and Optimization
- [x] Add performance benchmarks:
  - [x] Integrate a benchmarking framework (e.g., `criterion`).
  - [x] Write benchmarks for each endpoint (`/encrypt`, `/decrypt`, `/sign`, `/verify`).
  - [x] Write benchmarks for core cryptographic functions in `crypto.rs`.
  - [x] Analyze benchmark results and identify areas for optimization.
- [x] Review and optimize cryptographic operations:
  - [x] Identify potential performance bottlenecks in `crypto.rs` based on benchmark results.
  - [x] (moot) Investigate alternative cryptographic libraries or implementations if necessary.
  - [x] Refactor `crypto.rs` for clarity and efficiency based on review and benchmarks.
- [x] Implement standardized error responses:
  - [x] Review existing error handling in `routes.rs` and `main.rs`.
  - [x] Define a consistent JSON error response format (e.g., `{"error": "message"}`).
  - [x] Implement user-friendly error messages for common failures (e.g., invalid input, decryption failure, verification failure).
  - [x] Ensure appropriate HTTP status codes are returned for all error scenarios.
- [x] Add request/response logging:
  - [x] Integrate a logging framework (e.g., `tracing`, `env_logger`).
  - [x] Configure logging middleware in `main.rs`.
  - [x] Log incoming request details (method, path, relevant headers).
  - [x] Log outgoing response details (status code).
  - [x] Consider options for logging request/response bodies securely (e.g., redaction).
- [x] Final documentation review:
  - [x] Review `README.md` for clarity, completeness, and accuracy.
  - [x] Review code comments across the project.
  - [x] Ensure example usage documentation is up-to-date and easy to follow.
  - [x] Verify that `spec.md` accurately reflects the final implementation.

## Phase 6: Addressing Critical Review Items (from critique.md)
- [ ] **1. Security Concerns**
  - [x] Load HMAC secret key from environment variables (`HMAC_SECRET_KEY`) instead of hardcoding.
  - [ ] (Out of scope) Implement secure storage/rotation for the secret key.
  - [ ] (Clarification) Note Base64 != encryption in documentation.
- [ ] **4. Code Structure**
  - [ ] Consider splitting `crypto.rs` into modules.
  - [ ] Address potential stack overflow in `canonicalize_json`.
  - [ ] (Out of scope) Implement async support for crypto operations.
- [ ] **5. Testing**
  - [ ] Add tests for edge cases (large payloads, unicode, etc.).
- [ ] **6. Documentation**
  - [ ] Add OpenAPI/Swagger documentation.
- [ ] **10. Code Quality**
  - [ ] Refactor long functions in `crypto.rs`.
- [ ] **11. Configuration**
  - [x] Make server port configurable via environment variable (`PORT`).
  - [x] Add `.env` file support (`dotenvy`).
  - [x] (Minor) Review logging configuration (using `env_logger` which is good).
  - [ ] (Out of scope) Support different configuration files per environment.
- [ ] **12. Dependencies**
  - [ ] Consolidate logging frameworks (if multiple were actually used - only `env_logger` seems active).
  - [ ] (Out of scope) Check dependencies for vulnerabilities.

## Success Criteria
- [x] All endpoints work as specified in the requirements
- [x] Encryption/decryption maintains data integrity
- [x] Signatures are consistent regardless of property order
- [x] Proper error handling and status codes
- [x] Comprehensive test coverage
- [x] Clean, maintainable code structure
