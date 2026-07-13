//! # CYBERDECK: Battery & Power Diagnostics Module
//!
//! Provides comprehensive power telemetry, including multi-battery status,
//! AC adapter detection, and energy flow metrics (Voltage, Power draw).
//!
//! ## Implementation Notes
//! - **Target**: Linux sysfs (`/sys/class/power_supply/`).
//! - **Compatibility**: Supports multi-battery setups (BAT0, BAT1), AC adapters (AC, ADP1).
//! - **Metrics**: Reads real-time capacity, energy, voltage, and power usage.

use std::fs;
use std::io::Write;
use crate::state::AppState;

/// Executes the battery diagnostic sweep.
///
/// Scans the system's power management tree to identify and report on
/// all detected power supply controllers.
pub async fn execute(_state: &AppState, params: &str) -> Result<String, String> {
    let dir = params;
    let bat_dir = format!("{}/battery", dir);

    fs::create_dir_all(&bat_dir).map_err(|e| e.to_string())?;
    let report_path = format!("{}/battery.md", bat_dir);

    let mut report = String::from("# 🔋 CYBERDECK: SYSTEM POWER & BATTERY TELEMETRY\n\n");
    let power_supply_root = "/sys/class/power_supply";

    if let Ok(entries) = fs::read_dir(power_supply_root) {
        let mut found_any = false;

        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().into_string().unwrap_or_default();

            // Skip non-directory entries
            if !path.is_dir() { continue; }
            found_any = true;

            report.push_str(&format!("## ⚡ Controller: `{}`\n", name));

            // Attributes to attempt to read
            let attributes = [
                ("Status", "status"),
                ("Capacity", "capacity"),
                ("Technology", "technology"),
                ("Model", "model_name"),
                ("Manufacturer", "manufacturer"),
                ("Voltage (µV)", "voltage_now"),
                ("Current Energy (µWh)", "energy_now"),
                ("Full Energy (µWh)", "energy_full"),
                ("Power Draw (µW)", "power_now"),
            ];

            for (label, file) in attributes {
                let attr_path = path.join(file);
                if let Ok(content) = fs::read_to_string(attr_path) {
                    report.push_str(&format!("- **{}:** {}\n", label, content.trim()));
                }
            }
            report.push('\n');
        }

        if !found_any {
            report.push_str("⚠️ No power controllers found in `/sys/class/power_supply/`.");
        }
    } else {
        report.push_str("❌ Access Denied: Cannot read `/sys/class/power_supply/`.");
    }

    fs::write(&report_path, report).map_err(|e| e.to_string())?;
    Ok(report_path)
}
