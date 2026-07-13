//! # Cyberdeck: CPU Diagnostic Module
//!
//! Handles processor architecture reporting, raw `/proc/cpuinfo` parsing,
//! and identification of compute cores.

use crate::diagnostics::run_cmd;
use std::fs;

/// Executes the CPU diagnostic sweep.
///
/// Captures high-level lscpu details and dumps granular /proc/cpuinfo
/// for full auditability.
pub async fn execute(dir: &str) -> std::io::Result<String> {
    let cpu_dir = format!("{}/cpu", dir);
    fs::create_dir_all(&cpu_dir)?;

    let report_path = format!("{}/cpu.md", cpu_dir);

    // Capture lscpu output
    let lscpu_data = run_cmd("lscpu", &[]);

    // Capture detailed cpuinfo
    let cpuinfo_data = fs::read_to_string("/proc/cpuinfo").unwrap_or_else(|_| "Unavailable".to_string());

    let report_content = format!(
        "# 🧠 CPU REPORT\n\n## Summary\n{}\n\n## Detailed Info\n```\n{}\n```",
        lscpu_data,
        cpuinfo_data
    );

    fs::write(&report_path, report_content)?;
    Ok(report_path)
}
