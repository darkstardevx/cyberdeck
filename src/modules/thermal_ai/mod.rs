use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_thermal_ai_module(dir: &str) -> std::io::Result<String> {
    // 1. Initialize Subdirectory Tree Structures
    let raw_dir = format!("{}/raw", dir);
    let parsed_dir = format!("{}/parsed", dir);

    fs::create_dir_all(&raw_dir)?;
    fs::create_dir_all(&parsed_dir)?;

    let base_f = format!("{}/thermal_ai.md", dir);
    let raw_f = format!("{}/raw_temps.md", raw_dir);
    let parsed_f = format!("{}/parsed_temps.md", parsed_dir);

    // Reusable file access closures
    let write_to = |path: &str, content: &str| -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    };

    let overwrite_to = |path: &str, content: &str| -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    };

    // Native implementations mimicking the `output_writer.fish` formats
    let cyber_write_header = |path: &str, title: &str, emoji: &str| -> std::io::Result<()> {
        let block = format!(
            "<div style='background:#6a0dad;color:white;padding:6px;'>{} {}</div>\n\n",
            emoji, title
        );
        overwrite_to(path, &block)
    };

    let cyber_write_section = |path: &str, title: &str, emoji: &str| -> std::io::Result<()> {
        let section = format!("\n## {} {}\n", emoji, title);
        write_to(path, &section)
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // 2. Base Header Configuration
    cyber_write_header(&base_f, "THERMAL INTELLIGENCE", "🌡️")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n", timestamp))?;

    // Reset tracking counters
    let mut max_temp: f64 = 0.0;
    let mut sensor_count = 0;
    overwrite_to(&raw_f, "")?;

    // 3. Safe Sensor Discovery Loops
    if let Ok(thermal_entries) = fs::read_dir("/sys/class/thermal") {
        let mut zones = Vec::new();
        for entry in thermal_entries.flatten() {
            let filename = entry.file_name().into_string().unwrap_or_default();
            if filename.starts_with("thermal_zone") {
                zones.push(entry.path().join("temp"));
            }
        }

        if !zones.is_empty() {
            cyber_write_section(&base_f, "RAW SENSOR DATA", "📡")?;

            for (index, zone_path) in zones.iter().enumerate() {
                if let Ok(raw_str) = fs::read_to_string(zone_path) {
                    if let Ok(raw_val) = raw_str.trim().parse::<f64>() {
                        let val = raw_val / 1000.0;
                        sensor_count += 1;

                        write_to(&base_f, &format!("Zone {}: {:.2}°C\n", index + 1, val))?;
                        write_to(&raw_f, &format!("{:.2}\n", val))?;

                        if val > max_temp {
                            max_temp = val;
                        }
                    }
                }
            }
        } else {
            // Fallback: Check for hardware lm-sensors interface profiles
            fallback_lm_sensors(&base_f, &raw_f, &cyber_write_section).await?;
        }
    } else {
        fallback_lm_sensors(&base_f, &raw_f, &cyber_write_section).await?;
    }

    // 4. Parsed Summary Mapping Output
    overwrite_to(&parsed_f, "# 🌡️ PARSED TEMPERATURE SUMMARY\n")?;
    if sensor_count > 0 {
        write_to(&parsed_f, &format!("- Peak temperature: {:.2}°C\n", max_temp))?;
    } else {
        write_to(&parsed_f, "- Peak temperature: unknown\n")?;
    }
    write_to(&parsed_f, &format!("- Sensors detected: {}\n", sensor_count))?;

    // 5. Final Diagnostic Summary Injection
    cyber_write_section(&base_f, "THERMAL SUMMARY", "🧠")?;
    write_to(&base_f, &format!("- Peak temperature: {:.2}°C\n", max_temp))?;
    write_to(&base_f, &format!("- Sensor count: {}\n", sensor_count))?;

    // 6. Cache Sync Hook
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/thermal_ai.md", cache_dir));
    }

    Ok(base_f)
}

async fn fallback_lm_sensors<F>(base_f: &str, raw_f: &str, section_writer: &F) -> std::io::Result<()>
where
F: Fn(&str, &str, &str) -> std::io::Result<()>,
{
    let mut file = OpenOptions::new().append(true).open(base_f)?;
    writeln!(file, "\n## ⚠️ Thermal zones not found")?;

    let output = Command::new("sensors").output();
    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            section_writer(base_f, "LM-SENSORS OUTPUT", "🧪")?;
            file.write_all(stdout.as_bytes())?;

            let mut r_file = OpenOptions::new().write(true).truncate(true).open(raw_f)?;
            r_file.write_all(stdout.as_bytes())?;
        }
        _ => {
            writeln!(file, "No thermal sensors available")?;
        }
    }
    Ok(())
}
