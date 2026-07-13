use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_storage_ai_module(dir: &str) -> std::io::Result<String> {
    // 1. Initialize Subdirectory Tree Structures
    let nvme_dir = format!("{}/nvme", dir);
    let ssd_dir = format!("{}/ssd", dir);
    let hdd_dir = format!("{}/hdd", dir);
    let usb_dir = format!("{}/usb", dir);
    let raw_dir = format!("{}/raw", dir);
    let fs_dir = format!("{}/filesystems", dir);
    let smart_dir = format!("{}/smart", dir);

    fs::create_dir_all(&nvme_dir)?;
    fs::create_dir_all(&ssd_dir)?;
    fs::create_dir_all(&hdd_dir)?;
    fs::create_dir_all(&usb_dir)?;
    fs::create_dir_all(&raw_dir)?;
    fs::create_dir_all(&fs_dir)?;
    fs::create_dir_all(&smart_dir)?;

    let base_f = format!("{}/storage_ai.md", dir);
    let lsblk_json_f = format!("{}/lsblk.json", raw_dir);
    let df_md_f = format!("{}/df.md", fs_dir);
    let nvme_list_f = format!("{}/nvme_list.md", nvme_dir);
    let lsblk_md_f = format!("{}/lsblk.md", raw_dir);

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

    let run_cmd = |cmd: &str, args: &[&str]| -> String {
        Command::new(cmd)
        .args(args)
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_default()
    };

    let run_cmd_timeout = |cmd: &str, args: &[&str]| -> String {
        let child = Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

        if let Ok(c) = child {
            match c.wait_with_output() {
                Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
                Err(_) => format!("{} execution timeout occurred\n", cmd),
            }
        } else {
            format!("{} not available\n", cmd)
        }
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // 2. Base Header Configuration
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>💾 STORAGE INTELLIGENCE SYSTEM (AI MODE)</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n", timestamp))?;

    // 3. Block Device Mapping & JSON Export
    write_to(&base_f, "## 🧩 BLOCK DEVICE MAP\n")?;
    let lsblk_map = run_cmd("lsblk", &["-o", "NAME,SIZE,MODEL,TYPE,MOUNTPOINT,FSTYPE"]);
    write_to(&base_f, &lsblk_map)?;

    let lsblk_json = run_cmd("lsblk", &["-J"]);
    overwrite_to(&lsblk_json_f, &lsblk_json)?;

    // 4. File System Allocation
    write_to(&base_f, "\n## 📊 FILESYSTEM USAGE\n")?;
    let df_h = run_cmd("df", &["-h"]);
    write_to(&base_f, &df_h)?;
    overwrite_to(&df_md_f, &df_h)?;

    // 5. Safe Device Enumeration & S.M.A.R.T. Hooks
    write_to(&base_f, "\n## 🧠 DEVICE CLASSIFICATION\n")?;
    if let Ok(sys_block_entries) = fs::read_dir("/sys/block") {
        for entry in sys_block_entries.flatten() {
            let d = entry.file_name().into_string().unwrap_or_default();
            if d.is_empty() { continue; }

            let dev_path = format!("/dev/{}", d);
            let model = fs::read_to_string(entry.path().join("device/model"))
            .unwrap_or_else(|_| "Unknown Model".to_string()).trim().to_string();
            let rota = fs::read_to_string(entry.path().join("queue/rotational"))
            .unwrap_or_else(|_| "-1".to_string()).trim().to_string();

            let out_file: String;
            if rota == "0" {
                out_file = format!("{}/{}.md", ssd_dir, d);
                write_to(&base_f, &format!("⚡ SSD/NVMe: {}\n", d))?;
            } else if rota == "1" {
                out_file = format!("{}/{}.md", hdd_dir, d);
                write_to(&base_f, &format!("🪵 HDD: {}\n", d))?;
            } else {
                out_file = format!("{}/{}.md", usb_dir, d);
                write_to(&base_f, &format!("🔌 UNKNOWN: {}\n", d))?;
            }

            overwrite_to(&out_file, &format!("# 💽 DRIVE: {}\n\nMODEL: {}\n\n", d, model))?;
            let lsblk_dev = run_cmd("lsblk", &["-o", "NAME,SIZE,TYPE,FSTYPE,MOUNTPOINT", &dev_path]);
            write_to(&out_file, &lsblk_dev)?;

            write_to(&out_file, "\n## 🧠 SMART DATA\n")?;
            let smartctl_out = run_cmd_timeout("smartctl", &["-a", &dev_path]);
            if smartctl_out.is_empty() || smartctl_out.contains("not available") {
                write_to(&out_file, "smartctl not installed\n")?;
            } else {
                write_to(&out_file, &smartctl_out)?;
            }
        }
    }

    // 6. NVMe Specific Infrastructure Probe
    write_to(&base_f, "\n## ⚡ NVMe INTELLIGENCE\n")?;
    let nvme_out = run_cmd_timeout("nvme", &["list"]);
    if nvme_out.is_empty() || nvme_out.contains("not available") {
        write_to(&base_f, "nvme-cli not installed\n")?;
    } else {
        write_to(&base_f, &nvme_out)?;
        overwrite_to(&nvme_list_f, &nvme_out)?;
    }

    // 7. Security Cryptographic Mapping
    write_to(&base_f, "\n## 🔐 ENCRYPTION LAYER (LUKS)\n")?;
    let lsblk_f = run_cmd("lsblk", &["-f"]);
    write_to(&base_f, &lsblk_f)?;

    if let Ok(mapper_entries) = fs::read_dir("/dev/mapper") {
        for entry in mapper_entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                write_to(&base_f, &format!("{}\n", name))?;
            }
        }
    }

    // 8. Raw Subsystem Matrix Architecture
    write_to(&base_f, "\n## 📦 RAW STORAGE DATA\n")?;
    write_to(&base_f, &run_cmd("lspci", &[]))?;
    write_to(&base_f, &run_cmd("lsusb", &[]))?;
    write_to(&base_f, &lsblk_f)?;
    overwrite_to(&lsblk_md_f, &lsblk_f)?;

    // 9. Aggregation Telemetry Parsing Summary
    write_to(&base_f, "\n## 🧠 STORAGE SUMMARY\n")?;
    let df_total_out = run_cmd("df", &["-h", "--total"]);
    if let Some(total_line) = df_total_out.lines().last() {
        let tokens: Vec<&str> = total_line.split_whitespace().collect();
        if tokens.len() >= 4 && total_line.starts_with("total") {
            write_to(&base_f, &format!("- Total Storage: {}\n- Used Storage: {}\n", tokens[1], tokens[2]))?;
        }
    }

    // 10. Cache Management Exporter Layer
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/storage_ai.md", cache_dir));
    }

    Ok(base_f)
}
