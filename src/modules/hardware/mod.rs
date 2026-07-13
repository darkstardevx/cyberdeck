use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_hardware_module(dir: &str) -> std::io::Result<String> {
    // 1. Target Output Architecture Configurations
    let base_f = format!("{}/hardware.md", dir);

    // Reusable file writing handlers
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
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>🖥️ HARDWARE CORE REPORT</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n", timestamp))?;

    // 3. Central Processor Profile Matrix
    write_to(&base_f, "## 🧠 CPU\n")?;
    write_to(&base_f, &run_cmd("lscpu", &[]))?;

    // 4. Peripheral Component Interconnect Mapping
    write_to(&base_f, "\n## 🧩 PCI DEVICES\n")?;
    write_to(&base_f, &run_cmd("lspci", &[]))?;

    // 5. Universal Serial Bus Node Mappings
    write_to(&base_f, "\n## 🔌 USB DEVICES\n")?;
    write_to(&base_f, &run_cmd("lsusb", &[]))?;

    // 6. Deep Structural Hardware Subsystem Tree Listing
    write_to(&base_f, "\n## ⚙️ DETAILED HARDWARE TREE\n")?;
    let lshw_out = run_cmd("sudo", &["lshw", "-short"]);
    if !lshw_out.is_empty() {
        write_to(&base_f, &lshw_out)?;
    } else {
        write_to(&base_f, "lshw diagnostics unavailable\n")?;
    }

    // 7. Engine Cache Exporter Sync Hook
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/hardware.md", cache_dir));
    }

    Ok(base_f)
}
