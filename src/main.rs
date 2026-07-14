//! # CYBERDECK Core
//! The entry point and primary controller for the CYBERDECK system.

#![warn(missing_docs)]

mod types;
mod parser;
mod modules;
mod dispatcher;
mod routes;

use crate::types::CyberdeckState;
use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::parser::parse_cyberdeck_script;
use tower_http::services::ServeDir; // 1. Added import

/// Primary entry point.
#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(CyberdeckState {
        display_active: false,
        active_modules: Vec::new(),
                                    stealth_mode: false,
                                    reports_generated: 0,
                                    execution_log: Vec::new(),
    }));

    let env_script = std::env::var("Cyberdeck_ENV").unwrap_or_else(|_| "init_display".to_string());
    let commands = parse_cyberdeck_script(&env_script);

    for cmd in commands {
        dispatcher::execute_cyberdeck_command(cmd, &state).await;
    }

    let app = Router::new()
    .route("/api/themes", get(routes::get_themes_list))
    .route("/", get(routes::get_index_page))
    // Updated paths to /cyberdeck/api/
    .route("/cyberdeck/api/state", get(routes::get_cyberdeck_state))
    .route("/cyberdeck/api/command", post(routes::post_cyberdeck_command))
    .route("/cyberdeck/api/action", post(routes::post_cyberdeck_action)) // Add this for the new popup
    .route("/cyberdeck/api/list", get(routes::list_diagnostics))
    .fallback_service(ServeDir::new("static"))
    .with_state(state);

    let port = std::env::var("CYBERDECK_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    println!("[Cyberdeck Core] API active on http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
