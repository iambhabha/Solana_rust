mod auth;
mod config;
mod db;
mod encryption;
mod errors;
mod routes;
mod solana_service;
mod wallet;

use routes::create_router;
use std::net::SocketAddr;
use std::sync::Arc;
use serde_json::json;
use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    // Load configuration from environment variables
    let config = Arc::new(config::Config::load());

    // Initialize database
    let database = Arc::new(
        db::Database::new(&config)
            .expect("Failed to initialize database"),
    );
    database
        .init_schema()
        .expect("Failed to initialize database schema");

    // Initialize Solana service
    let solana_service = Arc::new(solana_service::SolanaService::new((*config).clone()));

    // Create application state
    let app_state = routes::AppState {
        db: database,
        solana_service,
        config,
    };

    // Create router with all routes
    let app = create_router(app_state).route("/health", get(health_check));

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 Solana Token Backend Server running at http://{}", addr);
    println!("📝 Make sure to set all required environment variables in .env file");
    
    // Add CORS and Tracing middleware
    let app = app
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

/// GET /health - Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

