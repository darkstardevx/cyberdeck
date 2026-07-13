//! # Cyberdeck: Power Module
//!
//! Manages CPU frequency scaling, power profiles, and core health telemetry.

use crate::diagnostics::run_cmd;
use std::fs;

pub async fn execute(dir: &str) -> std::io::Result<String> {
    let power_dir = format!("{}/power", dir);
    fs::create_dir_all(&power_dir)?;
    let power_report = format!("{}/power.md", power_dir);

    let mut content = String::from("# 🔋 POWER PROFILE\n\n");

    // Power Info
    let cpupower_info = run_cmd("cpupower", &["frequency-info"]);
    content.push_str(&format!("## CPUPOWER\n{}\n", cpupower_info));

    // Scaling/Governor Info
    if let Ok(driver) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_driver") {
        content.push_str(&format!("## Scaling Driver\n{}\n", driver));
    }
    if let Ok(gov) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor") {
        content.push_str(&format!("## Active Governor\n{}\n", gov));
    }

    // Frequency Matrix
    content.push_str("\n## 📊 CPU FREQUENCY MATRIX\n");
    if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu") {
        for entry in entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit()) {
                let freq_path = format!("/sys/devices/system/cpu/{}/cpufreq/scaling_cur_freq", name);
                if let Ok(raw_f) = fs::read_to_string(freq_path) {
                    if let Ok(freq) = raw_f.trim().parse::<f64>() {
                        content.push_str(&format!("Core {}: {:.2} MHz\n", &name[3..], freq / 1000.0));
                    }
                }
            }
        }
    }

    fs::write(&power_report, content)?;
    Ok(power_report)
}
