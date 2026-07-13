//! # Cyberdeck Diagnostics
//!
//! Root module for hardware diagnostic sub-components.
//! Orchestrates CPU, motherboard, memory, and power modules.

pub mod battery;
pub mod cpu;
pub mod memory;
pub mod motherboard;
pub mod power;
/// Added new module - RunBatteryModule
use std::process::Command;

/// Shared diagnostic utility: Executes shell commands
/// Accessible by all modules via `crate::diagnostics::run_cmd`
pub fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_else(|_| format!("{} not available\n", cmd))
}

/// Shared utility: Appends content to a file
pub fn write_to(path: &str, content: &str) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
