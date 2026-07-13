use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_power_module(dir: &str) -> std::io::Result<String> {
    // Write out execution flag initialization to trace debug targets
    if let Ok(mut dbg) = OpenOptions::new().create(true).append(true).open("/tmp/cyberdeck_debug.log") {
        let _ = writeln!(dbg, "RUNNING POWER MODULE...");
    }

    // 1. Initialize Subdirectory Tree Structures
    let cpu_dir = format!("{}/cpu", dir);
    let gov_dir = format!("{}/governor", dir);
    let state_dir = format!("{}/state", dir);
    let energy_dir = format!("{}/energy", dir);
    let raw_dir = format!("{}/raw", dir);

    fs::create_dir_all(&cpu_dir)?;
    fs::create_dir_all(&gov_dir)?;
    fs::create_dir_all(&state_dir)?;
    fs::create_dir_all(&energy_dir)?;
    fs::create_dir_all(&raw_dir)?;

    let base_f = format!("{}/power.md", dir);
    let cpu_f = format!("{}/cpu.md", cpu_dir);
    let freq_f = format!("{}/frequency.md", state_dir);
    let raw_f = format!("{}/raw.md", raw_dir);

    // Dynamic file injection abstractions
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
    overwrite_to(&base_f, "# 🔋 POWER & PERFORMANCE INTELLIGENCE SYSTEM\n\n## 🧠 SYSTEM OVERVIEW\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n", timestamp))?;
    write_to(&base_f, &run_cmd("uname", &["-r"]))?;
    write_to(&base_f, &run_cmd("uptime", &[]))?;

    // 3. CPU Core State Mapping
    write_to(&base_f, "\n## ⚡ CPU POWER STATE\n")?;
    let lscpu_out = run_cmd("lscpu", &[]);
    write_to(&base_f, &lscpu_out)?;

    overwrite_to(&cpu_f, "# ⚡ CPU INFO\n")?;
    write_to(&cpu_f, &lscpu_out)?;

    // 4. Extract Dynamic Core Scaling Governors (Sysfs Traversals)
    write_to(&base_f, "\n## 🎛️ CPU GOVERNOR / SCALING\n")?;
    if let Ok(cpu_entries) = fs::read_dir("/sys/devices/system/cpu") {
        for entry in cpu_entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if name.starts_with("cpu") && name.chars().nth(3).map_or(false, |c| c.is_ascii_digit()) {
                let gov_path = entry.path().join("cpufreq/scaling_governor");
                if let Ok(gov) = fs::read_to_string(gov_path) {
                    let core_id = &name[3..];
                    write_to(&base_f, &format!("Core {}: {}\n", core_id, gov.trim()))?;
                }
            }
        }
    }

    if let Ok(avail_govs) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors") {
        write_to(&base_f, &format!("\n### AVAILABLE GOVERNORS\n{}", avail_govs))?;
    }

    // 5. UPower Device Bus Diagnostics
    write_to(&base_f, "\n## 🔋 ENERGY STATUS\n")?;
    let upower_devices = run_cmd("upower", &["-e"]);
    if !upower_devices.is_empty() && !upower_devices.contains("not found") {
        if let Some(first_dev) = upower_devices.lines().next() {
            let upower_info = run_cmd("upower", &["-i", first_dev.trim()]);
            write_to(&base_f, &upower_info)?;
        } else {
            write_to(&base_f, "upower: no devices detected\n")?;
        }
    } else {
        write_to(&base_f, "upower not installed\n")?;
    }

    // 6. AMD P-State Configuration Verification Layer
    write_to(&base_f, "\n### 🧠 AMD PSTATE DRIVER\n")?;
    if let Ok(pstate_status) = fs::read_to_string("/sys/devices/system/cpu/amd_pstate/status") {
        write_to(&base_f, &pstate_status)?;
    } else {
        write_to(&base_f, "amd_pstate not active or unavailable\n")?;
    }

    if let Ok(scaling_driver) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_driver") {
        write_to(&base_f, &format!("\n### ⚙️ SCALING DRIVER\n{}", scaling_driver))?;
    }

    // 7. Core Frequency Metrics Snapshot Engine ($\text{kHz} \rightarrow \text{MHz}$)
    write_to(&base_f, "\n## 📊 FREQUENCY SNAPSHOT\n")?;
    overwrite_to(&freq_f, "# 📊 FREQUENCY\n")?;

    if let Ok(cpu_entries) = fs::read_dir("/sys/devices/system/cpu") {
        // Collect, parse, sort cores to write clean ascending sequential profiles
        let mut cores = Vec::new();
        for entry in cpu_entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if name.starts_with("cpu") && name.chars().nth(3).map_or(false, |c| c.is_ascii_digit()) {
                if let Ok(core_id) = name[3..].parse::<u32>() {
                    let freq_path = entry.path().join("cpufreq/scaling_cur_freq");
                    if let Ok(freq_str) = fs::read_to_string(freq_path) {
                        if let Ok(khz) = freq_str.trim().parse::<f64>() {
                            cores.push((core_id, khz / 1000.0));
                        }
                    }
                }
            }
        }
        cores.sort_by_key(|k| k.0);
        for (id, mhz) in cores {
            let line = format!("Core {}: {:.2} MHz\n", id, mhz);
            write_to(&base_f, &line)?;
            write_to(&freq_f, &line)?;
        }
    }

    // 8. Sleep States / Global ACPI Power Registers
    write_to(&base_f, "\n## 🔌 SYSTEM POWER STATE\n")?;
    if let Ok(sys_power) = fs::read_to_string("/sys/power/state") {
        write_to(&base_f, &sys_power)?;
    }

    // 9. Load Metrics Snapshots
    write_to(&base_f, "\n## 📈 SYSTEM LOAD\n")?;
    let loadavg = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    write_to(&base_f, &loadavg)?;
    write_to(&base_f, &run_cmd("vmstat", &["1", "1"]))?;

    // 10. Raw Bus Topologies Dump Output
    write_to(&base_f, "\n## 📦 RAW POWER DATA\n")?;
    write_to(&base_f, &run_cmd("lspci", &[]))?;
    write_to(&base_f, &run_cmd("lsusb", &[]))?;

    overwrite_to(&raw_f, "# 📦 RAW POWER\n")?;
    write_to(&raw_f, &loadavg)?;
    if let Ok(sys_power) = fs::read_to_string("/sys/power/state") {
        write_to(&raw_f, &sys_power)?;
    }

    // 11. Integrity Modules Summary Dumps
    write_to(&base_f, "\n## 🧠 POWER SUMMARY\n")?;
    let cpupower_check = run_cmd("which", &["cpupower"]);
    if !cpupower_check.is_empty() && !cpupower_check.contains("no") {
        write_to(&base_f, "✔ cpupower active\n")?;
    }
    if fs::metadata("/sys/devices/system/cpu/amd_pstate/status").is_ok() {
        write_to(&base_f, "✔ amd_pstate detected\n")?;
    }

    // 12. Engine Cache Management Layer Mirroring
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/power.md", cache_dir));
    }

    Ok(base_f)
}
