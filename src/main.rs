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
    let app = create_router(app_state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Solana Token Backend Server running at http://{}", addr);
    println!("📝 Make sure to set all required environment variables in .env file");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
