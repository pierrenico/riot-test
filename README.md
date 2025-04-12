# Riot API

A Rust web service that provides encryption, decryption, signing, and verification endpoints for JSON data.

## Features

- **Encryption/Decryption**: Base64 encoding/decoding of top-level JSON properties.
- **Signing/Verification**: HMAC-SHA256 based signature generation and verification, resistant to JSON property order changes.
- **JSON Support**: Handles arbitrary JSON structures.
- **Error Handling**: Standardized JSON error responses and appropriate HTTP status codes.
- **Logging**: Configurable request/response logging.
- **Health Check**: `/health` endpoint for service monitoring.

## Dependencies

This project relies on several key crates:

- `actix-web`: For the web server framework.
- `serde` / `serde_json`: For JSON serialization and deserialization.
- `base64`: For Base64 encoding/decoding.
- `hmac` / `sha2`: For HMAC-SHA256 signature generation and verification.
- `log` / `env_logger`: For logging.

## API Endpoints

### 1. Encryption (`/encrypt`)
Encrypts all top-level properties of a JSON payload using Base64 encoding.

**Request:**
```bash
curl -X POST http://localhost:8080/encrypt \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "age": 30,
    "contact": {
      "email": "john@example.com",
      "phone": "123-456-7890"
    }
  }'
```

**Response:**
```json
{
  "name": "Sm9obiBEb2U=",
  "age": "MzA=",
  "contact": "eyJlbWFpbCI6ImpvaG5AZXhhbXBsZS5jb20iLCJwaG9uZSI6IjEyMy00NTYtNzg5MCJ9"
}
```

### 2. Decryption (`/decrypt`)
Decrypts Base64-encoded properties in a JSON payload. Non-encoded properties are ignored.

**Request:**
```bash
curl -X POST http://localhost:8080/decrypt \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Sm9obiBEb2U=",
    "age": "MzA=",
    "contact": "eyJlbWFpbCI6ImpvaG5AZXhhbXBsZS5jb20iLCJwaG9uZSI6IjEyMy00NTYtNzg5MCJ9"
  }'
```

**Response:**
```json
{
  "name": "John Doe",
  "age": 30,
  "contact": {
    "email": "john@example.com",
    "phone": "123-456-7890"
  }
}
```

### 3. Signing (`/sign`)
Generates an HMAC-SHA256 signature for a JSON payload. The signature is calculated based on a canonical representation of the JSON data, ensuring that the order of properties does not affect the result.

**Request:**
```bash
curl -X POST http://localhost:8080/sign \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Hello World",
    "timestamp": 1616161616
  }'
```

**Response:**
```json
{
  "signature": "a1b2c3d4e5f6g7h8i9j0..." // Example signature
}
```

### 4. Verification (`/verify`)
Verifies an HMAC-SHA256 signature for a JSON payload.

**Request:**
```bash
curl -X POST http://localhost:8080/verify \
  -H "Content-Type: application/json" \
  -d '{
    "signature": "a1b2c3d4e5f6g7h8i9j0...",
    "data": {
      "message": "Hello World",
      "timestamp": 1616161616
    }
  }'
```

**Response:**
- `204 No Content`: Signature is valid.
- `400 Bad Request`: Signature is invalid or input format is wrong (see Error Handling).

### 5. Health Check (`/health`)
Returns the operational status of the service.

**Request:**
```bash
curl http://localhost:8080/health
```

**Response:**
```json
{
  "status": "a-ok"
}
```

## Error Handling

The API uses standard HTTP status codes. For client-side errors (e.g., bad input, invalid signature), it returns `400 Bad Request` with a JSON body describing the error:

```json
{
  "error": "<error_message>"
}
```

Possible `<error_message>` values include:
- `Encryption failed`
- `Decryption failed`
- `Signing failed`
- `Invalid signature`
- `Verification failed`

Server-side errors might result in a `500 Internal Server Error` response.

## Logging

The application uses `env_logger` for logging. Request details (method, path, selected headers, status, duration) are logged for each request.

You can control the log level using the `RUST_LOG` environment variable. For example:

```bash
# Show info level logs (default)
RUST_LOG=info cargo run

# Show debug level logs for this crate
RUST_LOG=riot_api=debug cargo run

# Show warnings and errors only
RUST_LOG=warn cargo run
```

## Configuration

The application can be configured using environment variables. Create a `.env` file in the project root or set the variables directly.

- `PORT`: The port the server listens on. Defaults to `8080`.
- `HMAC_SECRET_KEY`: The secret key used for signing and verifying messages with HMAC-SHA256. **This must be set and should be a strong, securely generated key.**
- `RUST_LOG`: Controls the logging level (e.g., `info`, `debug`, `warn`, `error`). See the [env_logger documentation](https://docs.rs/env_logger/latest/env_logger/) for more details. Defaults to `info`.

Example `.env` file:

```dotenv
PORT=8081
HMAC_SECRET_KEY=your-super-secret-and-long-hmac-key
RUST_LOG=debug
```

## Development

### Prerequisites

- Rust (check `Cargo.toml` for the specific version, typically latest stable)
- Cargo

### Building

```bash
cargo build
```

### Running

```bash
cargo run
# Server will start on http://127.0.0.1:8080
```

### Testing

```bash
cargo test
```

## License

MIT 