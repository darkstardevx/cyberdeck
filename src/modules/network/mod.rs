use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_network_module(dir: &str) -> std::io::Result<String> {
    // 1. Initialize Subdirectory Tree Structures
    let iface_dir = format!("{}/interfaces", dir);
    let route_dir = format!("{}/routes", dir);
    let dns_dir = format!("{}/dns", dir);
    let socket_dir = format!("{}/sockets", dir);
    let raw_dir = format!("{}/raw", dir);

    fs::create_dir_all(&iface_dir)?;
    fs::create_dir_all(&route_dir)?;
    fs::create_dir_all(&dns_dir)?;
    fs::create_dir_all(&socket_dir)?;
    fs::create_dir_all(&raw_dir)?;

    let base_f = format!("{}/network.md", dir);
    let routes_f = format!("{}/routes.md", route_dir);
    let resolv_f = format!("{}/resolv.conf", dns_dir);
    let sockets_f = format!("{}/sockets.md", socket_dir);
    let raw_f = format!("{}/raw.md", raw_dir);

    // Helpers for file manipulation
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
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>🌐 NETWORK INTELLIGENCE SYSTEM (NIC / ROUTES / SOCKETS)</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n", timestamp))?;

    // 3. Network Stack Snapshot
    write_to(&base_f, "## 📦 NETWORK STACK SNAPSHOT\n")?;
    let ip_a = run_cmd("ip", &["a"]);
    let ip_link = run_cmd("ip", &["link"]);
    let ip_route = run_cmd("ip", &["route"]);
    let ss_out = run_cmd("ss", &["-tulpn"]);

    write_to(&base_f, &ip_a)?;
    write_to(&base_f, &ip_link)?;
    write_to(&base_f, &ip_route)?;
    write_to(&base_f, &ss_out)?;

    // 4. Interface Enumeration Loops
    write_to(&base_f, "\n## 🔌 INTERFACES (DETAILED)\n")?;
    if let Ok(net_entries) = fs::read_dir("/sys/class/net") {
        for entry in net_entries.flatten() {
            let i = entry.file_name().into_string().unwrap_or_default();
            if i.is_empty() { continue; }

            let f = format!("{}/{}.md", iface_dir, i);
            overwrite_to(&f, &format!("# 🔌 INTERFACE: {}\n\n## 📡 IP CONFIG\n", i))?;
            write_to(&f, &run_cmd("ip", &["addr", "show", &i]))?;

            write_to(&f, "\n## ⚙️ STATE\n")?;
            if let Ok(operstate) = fs::read_to_string(entry.path().join("operstate")) {
                write_to(&f, &format!("{}\n", operstate.trim()))?;
            }

            if let Ok(speed) = fs::read_to_string(entry.path().join("speed")) {
                write_to(&f, &format!("\n## 🚀 LINK SPEED (Mbps)\n{}\n", speed.trim()))?;
            }

            if let Ok(duplex) = fs::read_to_string(entry.path().join("duplex")) {
                write_to(&f, &format!("\n## 🔗 DUPLEX\n{}\n", duplex.trim()))?;
            }

            let ethtool_out = run_cmd("ethtool", &[&i]);
            if !ethtool_out.is_empty() && !ethtool_out.contains("not found") {
                write_to(&f, &format!("\n## 🧠 ETH TOOL INFO\n{}", ethtool_out))?;
            }
        }
    }

    // 5. Routing Configuration Layer
    write_to(&base_f, "\n## 🧭 ROUTING TABLE\n")?;
    write_to(&base_f, &ip_route)?;

    overwrite_to(&routes_f, "# 🧭 ROUTES\n")?;
    write_to(&routes_f, &ip_route)?;

    // 6. DNS Resolver Configuration Hooks
    write_to(&base_f, "\n## 🌍 DNS / RESOLVER\n")?;
    if let Ok(resolv_content) = fs::read_to_string("/etc/resolv.conf") {
        write_to(&base_f, &resolv_content)?;
        overwrite_to(&resolv_f, &resolv_content)?;
    } else {
        write_to(&base_f, "no resolv.conf found\n")?;
    }

    let resolvectl_out = run_cmd("resolvectl", &["status"]);
    if !resolvectl_out.is_empty() && !resolvectl_out.contains("not found") {
        write_to(&base_f, "\n### SYSTEMD RESOLVED STATUS\n")?;
        write_to(&base_f, &resolvectl_out)?;
    }

    // 7. Active Sockets Output
    write_to(&base_f, "\n## 🔥 ACTIVE SOCKETS\n")?;
    write_to(&base_f, &ss_out)?;

    overwrite_to(&sockets_f, "# 🔥 SOCKETS\n")?;
    write_to(&sockets_f, &ss_out)?;

    // 8. Structural Hardware Snapshot
    write_to(&base_f, "\n## 🧩 NETWORK HARDWARE\n")?;
    write_to(&base_f, &run_cmd("lspci", &[]))?;
    write_to(&base_f, &run_cmd("lsusb", &[]))?;

    // 9. Kernel Driver Realtek Conflict Diagnostics
    write_to(&base_f, "\n## 🧠 NIC DRIVER STATUS\n")?;
    let lsmod_out = run_cmd("sh", &["-c", "lsmod | grep -E 'r8125|r8169'"]);
    if lsmod_out.contains("r8125") {
        write_to(&base_f, "✔ r8125 driver active (Realtek 2.5GbE)\n")?;
    }
    if lsmod_out.contains("r8169") {
        write_to(&base_f, "⚠ r8169 also loaded (possible conflict)\n")?;
    }

    // 10. Raw Stack Collection Layout Dump
    write_to(&base_f, "\n## 📦 RAW NETWORK DATA\n")?;
    write_to(&base_f, &ip_a)?;
    write_to(&base_f, &ip_link)?;
    write_to(&base_f, &ip_route)?;
    write_to(&base_f, &ss_out)?;

    overwrite_to(&raw_f, "# 📦 RAW NETWORK\n")?;
    write_to(&raw_f, &ip_a)?;
    write_to(&raw_f, &ip_route)?;

    // 11. Cache Export Engine Setup
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/network.md", cache_dir));
    }

    Ok(base_f)
}
