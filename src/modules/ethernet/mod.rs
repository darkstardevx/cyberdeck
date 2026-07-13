use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_ethernet_module(dir: &str) -> std::io::Result<String> {
    // 1. Initialize Subdirectory Tree Structures
    let if_dir = format!("{}/interfaces", dir);
    let drv_dir = format!("{}/driver", dir);
    let net_dir = format!("{}/network", dir);
    let stats_dir = format!("{}/stats", dir);

    fs::create_dir_all(&if_dir)?;
    fs::create_dir_all(&drv_dir)?;
    fs::create_dir_all(&net_dir)?;
    fs::create_dir_all(&stats_dir)?;

    let base_f = format!("{}/ethernet.md", dir);

    // File writing abstractions
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
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>🌐 ETHERNET INTELLIGENCE (RTL8125 / r8125)</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n## 🧠 SYSTEM OVERVIEW\n", timestamp))?;

    let hostname = run_cmd("hostname", &[]);
    write_to(&base_f, &format!("{}\n", hostname.trim()))?;

    // 3. Global Network Stack Snapshot
    write_to(&base_f, "\n## 🌐 NETWORK STACK\n")?;
    write_to(&base_f, &run_cmd("ip", &["a"]))?;
    write_to(&base_f, &run_cmd("ip", &["link"]))?;
    write_to(&base_f, &run_cmd("ip", &["route"]))?;

    // 4. Safe Interface Enumeration (Sysfs Matrix Iterator)
    write_to(&base_f, "\n## 🔌 INTERFACES\n")?;

    let entries = fs::read_dir("/sys/class/net/")
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;

    let mut interfaces = Vec::new();

    for entry in entries.flatten() {
        if let Some(if_name) = entry.file_name().to_str() {
            interfaces.push(if_name.to_string());
        }
    }

    for i in &interfaces {
        let safe_i = i.replace('/', "_");
        let f = format!("{}/{}.md", if_dir, safe_i);

        overwrite_to(&f, &format!("# 🔌 INTERFACE: {}\n\n## 📊 IP CONFIG\n", i))?;
        write_to(&f, &run_cmd("ip", &["addr", "show", i]))?;

        write_to(&f, "\n## ⚙️ LINK STATUS\n")?;
        if let Ok(operstate) = fs::read_to_string(format!("/sys/class/net/{}/operstate", i)) {
            write_to(&f, &format!("{}\n", operstate.trim()))?;
        }

        // Link Speed Evaluation
        if let Ok(speed) = fs::read_to_string(format!("/sys/class/net/{}/speed", i)) {
            write_to(&f, &format!("\n## 🚀 LINK SPEED (Mbps)\n{}\n", speed.trim()))?;
        }

        // Ethtool Diagnostic Hooks
        let ethtool_out = run_cmd("ethtool", &[i]);
        if !ethtool_out.is_empty() {
            write_to(&f, &format!("\n## 🔗 ETH TOOL STATUS\n{}", ethtool_out))?;
        }
    }

    // 5. Driver Layer Verification mapping
    write_to(&base_f, "\n## 🧩 DRIVER LAYER (r8125)\n")?;
    for i in &interfaces {
        let driver_path = format!("/sys/class/net/{}/device/driver", i);
        if let Ok(target) = fs::read_link(driver_path) {
            if let Some(drv_name) = target.file_name().and_then(|n| n.to_str()) {
                write_to(&base_f, &format!("### {} driver:\n{}\n", i, drv_name))?;
            }
        }
    }

    // 6. Active Kernel Drivers Parsing & Conflict Auditing
    write_to(&base_f, "\n## 🧠 LOADED KERNEL MODULES (NIC RELATED)\n")?;
    let lsmod_out = run_cmd("sh", &["-c", "lsmod | grep -E 'r8125|r8169|realtek|e1000|igc'"]);
    write_to(&base_f, &lsmod_out)?;

    write_to(&base_f, "\n## ⚠️ DRIVER CONFLICT CHECK\n")?;
    if lsmod_out.contains("r8169") {
        write_to(&base_f, "⚠️ r8169 detected (may conflict with r8125)\n")?;
    } else {
        write_to(&base_f, "✔ no r8169 conflict detected\n")?;
    }

    // 7. System PCI/USB Bus Raw Snapshots
    write_to(&base_f, "\n## 📦 RAW NETWORK DATA\n")?;
    write_to(&base_f, &run_cmd("lspci", &[]))?;
    write_to(&base_f, &run_cmd("lsusb", &[]))?;

    // 8. Individual Interface Telemetry Statistics
    write_to(&base_f, "\n## 📊 INTERFACE STATS\n")?;
    for i in &interfaces {
        let safe_i = i.replace('/', "_");
        let f = format!("{}/{}-stats.md", stats_dir, safe_i);

        overwrite_to(&f, &format!("# 📊 STATS: {}\n", i))?;
        let stats_path = format!("/sys/class/net/{}/statistics", i);

        if fs::metadata(&stats_path).is_ok() {
            let rx_pkts = fs::read_to_string(format!("{}/rx_packets", stats_path)).unwrap_or_default();
            let tx_pkts = fs::read_to_string(format!("{}/tx_packets", stats_path)).unwrap_or_default();
            let rx_bytes = fs::read_to_string(format!("{}/rx_bytes", stats_path)).unwrap_or_default();
            let tx_bytes = fs::read_to_string(format!("{}/tx_bytes", stats_path)).unwrap_or_default();

            write_to(&f, &format!("## RX/TX PACKETS\nRX: {}\nTX: {}\n\n## RX/TX BYTES\nRX: {}\nTX: {}\n",
                                  rx_pkts.trim(), tx_pkts.trim(), rx_bytes.trim(), tx_bytes.trim()))?;
        }
    }

    // 9. Cache Replication Engine Export
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/ethernet.md", cache_dir));
    }

    Ok(base_f)
}
