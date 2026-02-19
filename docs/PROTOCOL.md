# Secure Vault Protocol Specification

Version: 1.0
Transport: TLS over TCP
Encoding: JSON (UTF-8)

---

## 1. Overview

This protocol defines communication between:

- Vault CLI Client
- Vault Server

All communication MUST occur over TLS.
All messages are JSON-encoded.
Each request receives exactly one response.

---

## 2. Message Framing

Each message is:

- A single JSON object
- UTF-8 encoded
- Delimited by newline (`\n`)

Example:

```json
{ ...json... }\n
```

---

## 3. Request Format

Every request MUST include a `command` field.

### 3.1 Add Secret

Stores a new encrypted secret.

```json
{
  "command": "add",
  "username": "alice",
  "password": "plaintext_password",
  "service": "github",
  "secret": "ghp_token"
}
```

#### Semantics

- If user does not exist → create user
- Secret MUST be encrypted before storage
- Overwrites existing service entry

### 3.2 List Services

Returns list of service names for a user.

```json
{
  "command": "list",
  "username": "alice",
  "password": "plaintext_password"
}
```

### 3.3 Fetch Secret

Returns decrypted secret for a service.

```json
{
  "command": "fetch",
  "username": "alice",
  "password": "plaintext_password",
  "service": "github"
}
```

---

## 4. Response Format

### 4.1 Success (Generic)

```json
{
  "status": "ok"
}
```

### 4.2 Secret Response

```json
{
  "status": "ok",
  "service": "github",
  "secret": "ghp_token"
}
```

### 4.3 Services List

```json
{
  "status": "ok",
  "services": ["github", "aws", "gmail"]
}
```

### 4.4 Error Response

```json
{
  "status": "error",
  "message": "Invalid credentials"
}
```

## 5. Authentication Model

- Authentication is password-based.
- Password is used to derive a cryptographic key.
- Server NEVER stores plaintext passwords.
- Server stores:
  - password hash
  - salt
- All secrets are stored encrypted using a derived key.

## 6. Security Guarantees

Layer 1 — TLS

- Protects data in transit

Layer 2 — Password-derived encryption

- Protects data at rest

Layer 3 — Per-secret nonce

- Prevents replay and reuse attacks

---

## 7. Error Conditions

Server MUST return error for:

- Invalid credentials
- Service not found
- Malformed JSON
- Missing required fields

---

## 8. Future Extensions

- Session tokens
- Multi-factor authentication
- Versioned protocol header
- Binary encoding (protobuf)

---

## 9. Versioning

Current protocol version: 1.0

Breaking changes MUST increment major version.
