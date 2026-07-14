//! # CYBERDECK Core
//!
//! The entry point and primary controller for the CYBERDECK system.
//! This module acts as the "Kernel," managing the asynchronous runtime,
//! system state synchronization, and the HTTP API routing interface.

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

/// Primary entry point.
/// Initializes the multi-threaded tokio runtime and blocks on the async initialization.
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
    .route("/", get(routes::get_index_page))
    .route("/api/cyberdeck/state", get(routes::get_cyberdeck_state))
    .route("/api/cyberdeck/command", post(routes::post_cyberdeck_command))
    .with_state(state);

    let port = std::env::var("CYBERDECK_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    println!("[Cyberdeck Core] API active on http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
