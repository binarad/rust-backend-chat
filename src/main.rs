use std::process::ExitCode;

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct UserRegisterRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    id: i32,
    username: String,
}

use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHasher, generate_salt},
};
use password_hash::phc::{PasswordHash, SaltString};
fn password_hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Salt generation
    let salt = generate_salt(); // Dont't know what's better 
    let salt2 = SaltString::generate(); // What's better?

    // Argon2 instance with default parameters
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password_with_salt(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, stored_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(&stored_hash) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to parse stored hash: {}", e);
            return false;
        }
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

async fn register_request(
    State(pool): State<PgPool>,
    Json(user): Json<UserRegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), StatusCode> {
    // Kinda validation
    if user.username.trim().is_empty()
        || user.username.chars().count() > 50
        || user.password.is_empty()
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let hashed_password = match password_hash(&user.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Internal Server Error: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let request = sqlx::query!(
        "INSERT INTO users (username, password_hash) 
        VALUES ($1, $2) 
        ON CONFLICT (username) DO NOTHING
        RETURNING id",
        user.username,
        hashed_password,
    )
    .fetch_optional(&pool)
    .await;

    match request {
        Ok(Some(row)) => {
            return Ok((
                StatusCode::CREATED,
                Json(RegisterResponse {
                    id: row.id,
                    username: user.username,
                }),
            ));
        }
        Ok(None) => return Err(StatusCode::CONFLICT),
        Err(err) => {
            tracing::error!("Internale Server Error: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health_response() -> Json<HealthResponse> {
    let response = HealthResponse { status: "ok" };
    Json(response)
}

#[tokio::main]
async fn main() -> ExitCode {
    // build our application with a single route
    // tracing
    tracing_subscriber::fmt::init();

    //db connection
    dotenvy::dotenv().ok();
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(err) => {
            tracing::error!("cOULD NOT READ DATABASE_URL: {}", err);
            return std::process::ExitCode::FAILURE;
        }
    };

    let pool = match PgPool::connect(&db_url).await {
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!("COULD NOT CONNECT TO DATABASE: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let app = Router::new()
        .route("/health", get(health_response))
        .route("/auth/register", post(register_request))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    ExitCode::SUCCESS
}
