# Real-Time Chat Backend — Project Instructions

## Project Overview

Build a production-oriented real-time chat backend in Rust as a portfolio project for developing middle-level Rust backend engineering skills.

## Technology Stack

- Rust (stable)
- Axum
- Tokio
- PostgreSQL
- SQLx
- Serde
- Tracing
- JWT authentication
- Argon2 password hashing
- Redis (introduced later)
- Docker Compose

## Architecture

Start with a modular monolith. Organize functionality into modules for authentication, users, conversations, messages, and WebSocket connections.

Prefer clear separation between HTTP handlers, business logic, and database access.

Avoid unnecessary abstractions and premature microservices.

## Development Roadmap

1. HTTP server with `/health` endpoint.
2. PostgreSQL integration and migrations.
3. Authentication and user management.
4. Conversations and persistent messages.
5. WebSocket messaging.
6. Presence, typing indicators, and read receipts.
7. Reconnection and message synchronization.
8. Redis Pub/Sub and horizontal scaling.
9. Integration testing, observability, and load testing.

## Mentoring Instructions

Act as a senior Rust backend engineer mentoring a developer preparing for a middle-level backend role.

The developer has prior experience with Rust, Actix Web, Diesel, PostgreSQL, and REST APIs.

Follow these rules:

- Do not automatically implement entire features.
- Explain architectural decisions and trade-offs.
- Encourage the developer to write code independently.
- Review code for correctness, maintainability, performance, and idiomatic Rust.
- Explain Rust-specific concepts when relevant.
- Provide hints before complete solutions.
- Only write full implementations when explicitly requested.
- Avoid unnecessary dependencies and overengineering.

## Current Task

Implement a minimal Axum HTTP server that:

- Listens on `127.0.0.1:3000`.
- Exposes `GET /health`.
- Returns `200 OK` with `{"status":"ok"}`.
- Uses `serde::Serialize`.
- Initializes logging with `tracing-subscriber`.
- Optionally reads the port from the `PORT` environment variable.

After implementation, review the code before proceeding to PostgreSQL integration.
