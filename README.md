# Riot API

A Rust web service that provides encryption, decryption, signing, and verification endpoints for JSON data.

## Features

- **Encryption/Decryption**: Base64 encoding/decoding of JSON properties
- **Signing/Verification**: HMAC-based signature generation and verification
- **JSON Support**: Full support for nested JSON structures
- **Error Handling**: Proper HTTP status codes and error messages

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
Decrypts Base64-encoded properties in a JSON payload.

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
Generates an HMAC signature for a JSON payload.

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
  "signature": "a1b2c3d4e5f6g7h8i9j0..."
}
```

### 4. Verification (`/verify`)
Verifies an HMAC signature for a JSON payload.

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
- Status 204: Signature is valid
- Status 400: Signature is invalid

## Error Handling

The API returns appropriate HTTP status codes and error messages:

- 400 Bad Request: Invalid input data or signature
- 500 Internal Server Error: Server-side errors

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## License

MIT 