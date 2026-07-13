use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_memory_module(dir: &str) -> std::io::Result<String> {
    // 1. Initialize Subdirectory Tree Structures
    let dimm_dir = format!("{}/dimm", dir);
    let usage_dir = format!("{}/usage", dir);
    let topo_dir = format!("{}/topology", dir);
    let raw_dir = format!("{}/raw", dir);

    fs::create_dir_all(&dimm_dir)?;
    fs::create_dir_all(&usage_dir)?;
    fs::create_dir_all(&topo_dir)?;
    fs::create_dir_all(&raw_dir)?;

    let base_f = format!("{}/memory.md", dir);
    let usage_f = format!("{}/usage.md", usage_dir);
    let dimm_raw_f = format!("{}/dimm_raw.md", dimm_dir);
    let topology_f = format!("{}/topology.md", topo_dir);
    let swap_f = format!("{}/swap.md", usage_dir);
    let raw_f = format!("{}/raw.md", raw_dir);

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
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>🧠 MEMORY INTELLIGENCE SYSTEM (RAM / DIMM / SWAP)</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n", timestamp))?;

    // 3. Collect Memory Allocation Usage Stats
    write_to(&base_f, "## 📊 MEMORY USAGE\n")?;
    let free_h = run_cmd("free", &["-h"]);
    let vmstat = run_cmd("vmstat", &["1", "1"]);

    write_to(&base_f, &free_h)?;
    write_to(&base_f, &vmstat)?;

    overwrite_to(&usage_f, "# 📊 MEMORY USAGE\n")?;
    write_to(&usage_f, &free_h)?;
    write_to(&usage_f, &vmstat)?;

    // 4. Physical DIMM Tracking (dmidecode fallback parsing)
    write_to(&base_f, "\n## 🧩 DIMM / PHYSICAL MEMORY\n")?;
    let dmi_out = run_cmd("sudo", &["dmidecode", "-t", "memory"]);
    if dmi_out.is_empty() || dmi_out.contains("not found") {
        write_to(&base_f, "dmidecode not available\n")?;
    } else {
        write_to(&base_f, &dmi_out)?;
        overwrite_to(&dimm_raw_f, &dmi_out)?;
    }

    // 5. Structure Allocation Map (NUMA Node Layout Lookup)
    write_to(&base_f, "\n## 🧬 MEMORY SLOT STRUCTURE\n")?;
    if let Ok(sys_nodes) = fs::read_dir("/sys/devices/system/node") {
        for entry in sys_nodes.flatten() {
            let path = entry.path();
            let filename = entry.file_name().into_string().unwrap_or_default();
            if filename.starts_with("node") {
                let meminfo_path = path.join("meminfo");
                if let Ok(node_info) = fs::read_to_string(&meminfo_path) {
                    write_to(&base_f, &format!("### NUMA NODE: {}\n", filename))?;
                    write_to(&base_f, &node_info)?;
                }
            }
        }
    }

    // 6. System Processing Topology Layer
    write_to(&base_f, "\n## 🧠 TOPOLOGY\n")?;
    let lscpu_out = run_cmd("lscpu", &[]);
    write_to(&base_f, &lscpu_out)?;

    overwrite_to(&topology_f, "# 🧠 MEMORY TOPOLOGY\n")?;
    write_to(&topology_f, &lscpu_out)?;

    // 7. Virtual Swap Profile Monitoring
    write_to(&base_f, "\n## 🔄 SWAP & PRESSURE\n")?;
    let swapon_out = run_cmd("swapon", &["--show"]);
    let proc_meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();

    write_to(&base_f, &swapon_out)?;
    write_to(&base_f, &proc_meminfo)?;

    overwrite_to(&swap_f, "# 🔄 SWAP INFO\n")?;
    write_to(&swap_f, &swapon_out)?;
    write_to(&swap_f, &proc_meminfo)?;

    // 8. Output Raw Bus Mappings
    write_to(&base_f, "\n## 📦 RAW MEMORY DATA\n")?;
    write_to(&base_f, &run_cmd("lspci", &[]))?;
    write_to(&base_f, &run_cmd("lsusb", &[]))?;

    overwrite_to(&raw_f, "# 📦 RAW MEMORY DUMP\n")?;
    write_to(&raw_f, &proc_meminfo)?;
    write_to(&raw_f, &free_h)?;

    // 9. Process Execution Metrics Summary
    write_to(&base_f, "\n## 🧠 MEMORY SUMMARY\n")?;
    let total_ram = free_h.lines()
    .find(|l| l.starts_with("Mem:"))
    .and_then(|l| l.split_whitespace().nth(1))
    .unwrap_or("Unknown");
    let used_ram = free_h.lines()
    .find(|l| l.starts_with("Mem:"))
    .and_then(|l| l.split_whitespace().nth(2))
    .unwrap_or("Unknown");

    write_to(&base_f, &format!("- Total RAM: {}\n- Used RAM: {}\n", total_ram, used_ram))?;

    // 10. Cache Replication Export Engine Hook
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/memory.md", cache_dir));
    }

    Ok(base_f)
}
