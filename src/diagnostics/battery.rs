//! # Cyberdeck: Battery Module
//!
//! Handles battery status telemetry, including charge level, status (Charging/Discharging),
//! and technology type.

use std::fs;

/// Executes the battery diagnostic sweep.
///
/// Probes the `/sys/class/power_supply/` tree to extract real-time
/// power state data.
pub async fn execute(dir: &str) -> std::io::Result<String> {
    let bat_dir = format!("{}/battery", dir);
    fs::create_dir_all(&bat_dir)?;
    let report_path = format!("{}/battery.md", bat_dir);

    // Common path for primary battery
    let bat_path = "/sys/class/power_supply/BAT0";

    let mut report = String::from("# 🔋 BATTERY DIAGNOSTIC\n\n");

    if fs::metadata(bat_path).is_ok() {
        let capacity = fs::read_to_string(format!("{}/capacity", bat_path)).unwrap_or_default();
        let status = fs::read_to_string(format!("{}/status", bat_path)).unwrap_or_default();
        let tech = fs::read_to_string(format!("{}/technology", bat_path)).unwrap_or_default();

        report.push_str(&format!("- **Capacity:** {}%\n", capacity.trim()));
        report.push_str(&format!("- **Status:** {}\n", status.trim()));
        report.push_str(&format!("- **Technology:** {}\n", tech.trim()));
    } else {
        report.push_str("⚠️ No primary battery (BAT0) detected or accessible.");
    }

    fs::write(&report_path, report)?;
    Ok(report_path)
}
