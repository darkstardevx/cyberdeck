//! # Audio Diagnostics Module
//!
//! This module provides deep introspection capabilities for the system's audio stack,
//! specifically targeting **PipeWire**, **WirePlumber**, and **PulseAudio** compatibility layers.
//!
//! ## Implementation Notes
//! - **`std::fs` & `std::io::Write`**: Used to manage the diagnostic directory tree. We utilize `OpenOptions`
//!   with `append(true)` to ensure that logs are persistent and non-destructive during repeated runs.
//! - **`std::process::Command`**: Necessary for interfacing with system-level diagnostic binaries (`pactl`, `lsusb`, `lspci`).
//!   This allows the module to extract hardware and stream state directly from the kernel/userspace boundary.
//! - **`std::time`**: Required for creating distinct, chronological timestamps to correlate audio events with system states.
//!
//! ## Diagnostic Artifacts
//! The module generates three distinct output zones:
//! 1. `raw/`: Unprocessed output from system commands.
//! 2. `parsed/`: Human-readable summaries of sink/source states.
//! 3. `graph/`: Structural representation of the PipeWire node graph.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Executes the full audio diagnostic suite and generates a structured report.
///
/// This function acts as the primary orchestrator for audio diagnostics:
/// 1. Initializes the directory structure (`raw`, `parsed`, `graph`) under the provided base path.
/// 2. Probes PipeWire services and PulseAudio sinks/sources for real-time status.
/// 3. Captures hardware identifiers via `lspci` and `lsusb` to confirm peripheral detection.
/// 4. Generates a master markdown report and exports the node graph for external analysis.
///
/// # Arguments
/// * `dir` - The base directory where diagnostic artifacts and reports will be stored.
///
/// # Returns
/// * `Ok(String)` - The path to the primary generated `audio.md` report.
/// * `Err(std::io::Error)` - Propagates errors if directory creation or file writing fails.
pub async fn execute_audio_module(dir: &str) -> std::io::Result<String> {
    let raw_dir = format!("{}/raw", dir);
    let parsed_dir = format!("{}/parsed", dir);
    let graph_dir = format!("{}/graph", dir);

    fs::create_dir_all(&raw_dir)?;
    fs::create_dir_all(&parsed_dir)?;
    fs::create_dir_all(&graph_dir)?;

    let base_f = format!("{}/audio.md", dir);
    let raw_f = format!("{}/audio_raw.md", raw_dir);
    let graph_f = format!("{}/pipewire_graph.md", graph_dir);
    let parsed_f = format!("{}/audio_summary.md", parsed_dir);

    // Helper: Appends content to a specific file, creating it if it doesn't exist
    let write_to = |path: &str, content: &str| -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    write_to(&base_f, &format!("# 🎧 PIPEWIRE AUDIO INTELLIGENCE\n\nTimestamp: {}\n\n## 📊 SYSTEM OVERVIEW\n- Stack: PipeWire + WirePlumber + Pulse compatibility\n", timestamp))?;

    // Helper: Executes system shell commands and returns output as a String
    let run_cmd = |args: &[&str]| -> String {
        if args.is_empty() { return String::new(); }
        Command::new(args[0])
        .args(&args[1..])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_else(|_| format!("{} not available\n", args[0]))
    };

    write_to(&base_f, "\n## ⚙️ PIPEWIRE SERVICES\n")?;
    for service in &["pipewire", "wireplumber", "pipewire-pulse"] {
        let out = Command::new("systemctl")
        .args(&["--user", "status", service])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
        write_to(&base_f, &out)?;
    }

    write_to(&base_f, "\n## 🔊 PULSE COMPAT LAYER\n")?;
    let pactl_info = run_cmd(&["pactl", "info"]);
    write_to(&base_f, &pactl_info)?;
    write_to(&base_f, &run_cmd(&["pactl", "list", "short", "sinks"]))?;
    write_to(&base_f, &run_cmd(&["pactl", "list", "short", "sources"]))?;

    write_to(&base_f, "\n## 🔗 PIPEWIRE NODE GRAPH\n")?;
    let nodes = run_cmd(&["pw-cli", "ls", "Node"]);
    let links = run_cmd(&["pw-cli", "ls", "Link"]);

    let mut gf = fs::File::create(&graph_f)?;
    gf.write_all(nodes.as_bytes())?;
    gf.write_all(links.as_bytes())?;
    write_to(&base_f, &format!("Graph exported to: {}\n", graph_f))?;

    write_to(&base_f, "\n## 🎛️ AUDIO HARDWARE\n")?;
    let lspci_audio = Command::new("sh").args(&["-c", "lspci | grep -i audio"]).output().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default();
    let lsusb_audio = Command::new("sh").args(&["-c", "lsusb | grep -i audio"]).output().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default();
    write_to(&base_f, &lspci_audio)?;
    write_to(&base_f, &lsusb_audio)?;

    write_to(&base_f, "\n## 🧿 HYPR AUDIO INTEGRATION\n")?;
    let hypr = run_cmd(&["hyprpwcenter", "--version"]);
    write_to(&base_f, &hypr)?;

    let mut rf = fs::File::create(&raw_f)?;
    rf.write_all(run_cmd(&["lscpu"]).as_bytes())?;
    rf.write_all(run_cmd(&["lspci"]).as_bytes())?;
    rf.write_all(run_cmd(&["lsusb"]).as_bytes())?;
    rf.write_all(pactl_info.as_bytes())?;
    rf.write_all(nodes.as_bytes())?;

    let mut pf = fs::File::create(&parsed_f)?;
    pf.write_all("# 🎧 AUDIO SUMMARY\n\n## 🔊 Active Sinks\n".as_bytes())?;
    pf.write_all(run_cmd(&["pactl", "list", "short", "sinks"]).as_bytes())?;
    pf.write_all("\n## 🎤 Active Sources\n".as_bytes())?;
    pf.write_all(run_cmd(&["pactl", "list", "short", "sources"]).as_bytes())?;

    write_to(&base_f, "\n## 🧠 AUDIO HEALTH\n")?;
    for svc in &["pipewire", "wireplumber"] {
        let is_active = Command::new("systemctl")
        .args(&["--user", "is-active", svc])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

        let status_msg = if is_active { format!("✔ {}: running\n", svc) } else { format!("❌ {}: not running\n", svc) };
        write_to(&base_f, &status_msg)?;
    }

    Ok(base_f)
}
