use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_fan_module(dir: &str) -> std::io::Result<String> {
    // 1. Initialize Subdirectory Tree Structures
    let hw_dir = format!("{}/hwmon", dir);
    let sensors_dir = format!("{}/sensors", dir);
    let thermal_dir = format!("{}/thermal", dir);

    fs::create_dir_all(&hw_dir)?;
    fs::create_dir_all(&sensors_dir)?;
    fs::create_dir_all(&thermal_dir)?;

    let base_f = format!("{}/fan.md", dir);
    let sensors_raw_f = format!("{}/sensors_raw.md", sensors_dir);
    let hwmon_list_f = format!("{}/list.md", hw_dir);

    // Dynamic file writers
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

    let run_cmd = |cmd: &str, args: &[&str]| -> String {
        Command::new(cmd)
        .args(args)
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_default()
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // 2. Base Header Configuration
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>🌬️ COOLING & FAN INTELLIGENCE SYSTEM</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n## 🧠 SYSTEM OVERVIEW\n", timestamp))?;

    let uname_r = run_cmd("uname", &["-r"]);
    write_to(&base_f, &format!("{}\n", uname_r.trim()))?;

    // 3. lm-sensors Diagnostics
    write_to(&base_f, "\n## 🌡️ SENSOR STACK (lm-sensors)\n")?;
    let sensors_out = run_cmd("sensors", &[]);
    if sensors_out.is_empty() || sensors_out.contains("not found") {
        write_to(&base_f, "lm-sensors not installed\n")?;
    } else {
        write_to(&base_f, &sensors_out)?;
        overwrite_to(&sensors_raw_f, &sensors_out)?;
    }

    // 4. Hardware Monitor Core Interface Matrix Iteration
    write_to(&base_f, "\n## 🌪️ FAN RPM (HWMON)\n")?;
    if let Ok(hwmon_entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in hwmon_entries.flatten() {
            let path = entry.path();
            if let Ok(name) = fs::read_to_string(path.join("name")) {
                write_to(&base_f, &format!("\n### Device: {}", name.trim()))?;

                // Scan interior fan target points inside this device folder
                if let Ok(sub_entries) = fs::read_dir(&path) {
                    for sub_entry in sub_entries.flatten() {
                        let filename = sub_entry.file_name().into_string().unwrap_or_default();
                        if filename.starts_with("fan") && filename.ends_with("_input") {
                            if let Ok(rpm_str) = fs::read_to_string(sub_entry.path()) {
                                write_to(&base_f, &format!("- {}: {} RPM\n", filename, rpm_str.trim()))?;
                            }
                        }
                    }
                }
            }
        }
    }

    // 5. Fan Control Mapping Configuration
    write_to(&base_f, "\n## ⚙️ FAN CONTROL LAYER\n")?;
    if let Ok(fancontrol_cfg) = fs::read_to_string("/etc/fancontrol") {
        write_to(&base_f, "fancontrol config detected\n")?;
        write_to(&base_f, &fancontrol_cfg)?;
    } else {
        write_to(&base_f, "fancontrol not configured\n")?;
    }

    // 6. Direct Hardware Thermal Zone Loops Evaluation
    write_to(&base_f, "\n## 🧩 ASUS / ACPI LAYER\n")?;
    if let Ok(thermal_entries) = fs::read_dir("/sys/class/thermal") {
        for entry in thermal_entries.flatten() {
            let filename = entry.file_name().into_string().unwrap_or_default();
            if filename.starts_with("thermal_zone") {
                if let Ok(temp_str) = fs::read_to_string(entry.path().join("temp")) {
                    if let Ok(raw_temp) = temp_str.trim().parse::<f64>() {
                        write_to(&base_f, &format!("thermal zone ({}): {:.2}°C\n", filename, raw_temp / 1000.0))?;
                    }
                }
            }
        }
    }

    // 7. System PCI/USB Bus Architecture Snapshots
    write_to(&base_f, "\n## 📦 RAW SYSTEM SNAPSHOT\n")?;
    write_to(&base_f, &run_cmd("lspci", &[]))?;
    write_to(&base_f, &run_cmd("lsusb", &[]))?;

    // Log the flat hwmon directory tree listing out
    if let Ok(hwmon_dir_list) = fs::read_dir("/sys/class/hwmon") {
        let mut list_buf = String::new();
        for item in hwmon_dir_list.flatten() {
            list_buf.push_with_sz(item.file_name().to_str().unwrap_or_default(), "\n");
        }
        overwrite_to(&hwmon_list_f, &list_buf)?;
    }

    // 8. Health and Modules Integrity Summary
    write_to(&base_f, "\n## 🧠 COOLING SUMMARY\n")?;
    if !sensors_out.is_empty() {
        write_to(&base_f, "✔ lm-sensors active\n")?;
    } else {
        write_to(&base_f, "❌ lm-sensors missing\n")?;
    }

    if fs::metadata("/sys/class/hwmon").is_ok() {
        write_to(&base_f, "✔ hwmon interfaces detected\n")?;
    } else {
        write_to(&base_f, "❌ no hwmon data found\n")?;
    }

    // 9. Cache Replication Engine Export
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/fan.md", cache_dir));
    }

    Ok(base_f)
}

// Extensible local string builder helper trait assignment
trait StringAppendExt {
    fn push_with_sz(&mut self, s: &str, sep: &str);
}
impl StringAppendExt for String {
    fn push_with_sz(&mut self, s: &str, sep: &str) {
        if !self.is_empty() { self.push_str(sep); }
        self.push_str(s);
    }
}
