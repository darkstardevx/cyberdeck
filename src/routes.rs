//! # CYBERDECK Routes
//!
//! The HTTP interface for the CYBERDECK system.
//! This module maps API endpoints and web UI requests to the underlying command dispatcher
//! and system state management.

#![warn(missing_docs)]

use axum::{extract::State, response::Html, Json};
use crate::types::{SharedCyberdeckState, CyberdeckCommand};
use crate::dispatcher;

/// Renders the primary HTML index page.
///
/// Injects the current system state, including display status, stealth mode,
/// report counts, and the execution log, into the ui.html template.
pub async fn get_index_page(State(state): State<SharedCyberdeckState>) -> Html<String> {
    let s = state.lock().await;
    let template = include_str!("ui.html");
    let logs = s.execution_log.iter().rev()
    .map(|log| format!("<li>{}</li>", log))
    .collect::<Vec<String>>().join("");

    template
    .replace("{{DISPLAY_STATUS}}", if s.display_active { "ACTIVE" } else { "OFFLINE" })
    .replace("{{STEALTH_MODE}}", if s.stealth_mode { "ON" } else { "OFF" })
    .replace("{{REPORTS_COUNT}}", &s.reports_generated.to_string())
    .replace("{{SYSTEM_LOGS}}", &logs)
    .into()
}

/// API endpoint to retrieve the current snapshot of the Cyberdeck system state.
pub async fn get_cyberdeck_state(State(state): State<SharedCyberdeckState>) -> Json<crate::types::CyberdeckState> {
    let s = state.lock().await;
    Json(s.clone())
}

/// API endpoint to receive and execute a command via the dispatcher.
pub async fn post_cyberdeck_command(State(state): State<SharedCyberdeckState>, Json(cmd): Json<CyberdeckCommand>) -> Json<String> {
    dispatcher::execute_cyberdeck_command(cmd, &state).await;
    Json("Instruction pipeline advanced successfully.".to_string())
}
