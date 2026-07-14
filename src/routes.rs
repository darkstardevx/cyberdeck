//! # CYBERDECK Routes
//!
//! The HTTP interface for the CYBERDECK system.
//! This module maps API endpoints and web UI requests to the underlying command dispatcher
//! and system state management.

#![warn(missing_docs)]

use axum::{
    extract::State,
    response::{Html, IntoResponse}, // Add IntoResponse here
    Json,
};

use crate::types::{SharedCyberdeckState, CyberdeckCommand};
use crate::dispatcher;

use serde::{Deserialize};
use std::process::Command;

/// Renders the primary HTML index page.
///
/// Injects the current system state, including display status, stealth mode,
/// report counts, and the execution log, into the ui.html template.
pub async fn get_index_page(State(state): State<SharedCyberdeckState>) -> Html<String> {
    let s = state.lock().await;
    let template = include_str!("../static/ui.html");
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

use serde_json::{json, Value};
use std::fs;

pub async fn get_themes_list() -> impl IntoResponse {
    let mut themes = json!({});
    let base_path = "static/themes/sub-cyber";

    // Scan the 6 sub-folders
    if let Ok(folders) = fs::read_dir(base_path) {
        for folder in folders.filter_map(|f| f.ok()) {
            if folder.path().is_dir() {
                let folder_name = folder.file_name().into_string().unwrap_or_default();
                let mut theme_list = Vec::new();

                // Scan for .json files in these folders
                if let Ok(files) = fs::read_dir(folder.path()) {
                    for file in files.filter_map(|f| f.ok()) {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("json") {
                            let theme_name = file.path().file_stem().unwrap().to_str().unwrap().to_string();
                            theme_list.push(theme_name);
                        }
                    }
                }
                themes[folder_name] = json!(theme_list);
            }
        }
    }
    Json(themes)
}

#[derive(Deserialize)]
pub struct DeckAction {
    pub action: String, // "open", "copy", "archive"
}

pub async fn handle_deck_action(Json(payload): Json<DeckAction>) -> String {
    match payload.action.as_str() {
        "open" => {
            let _ = open::that("./output"); // Requires 'open' crate
            "Opening folder...".to_string()
        },
        "archive" => {
            // Simple logic to zip the directory
            let _ = Command::new("zip").args(["-r", "output.zip", "./output"]).output();
            "Archiving...".to_string()
        },
        _ => "Action Unknown".to_string()
    }
}
