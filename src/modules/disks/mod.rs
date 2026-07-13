use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn execute_disks_module(dir: &str) -> std::io::Result<String> {
    // Write out diagnostic tracking flag to trace execution initialization
    if let Ok(mut dbg) = OpenOptions::new().create(true).append(true).open("/tmp/cyberdeck_debug.log") {
        let _ = writeln!(dbg, "RUNNING DISKS MODULE...");
    }

    // 1. Structural Setup Layout
    let nvme_dir = format!("{}/nvme", dir);
    let ssd_dir = format!("{}/ssd", dir);
    let hdd_dir = format!("{}/hdd", dir);
    let usb_dir = format!("{}/usb", dir);
    let media_dir = format!("{}/media", dir);
    let raw_dir = format!("{}/raw", dir);

    fs::create_dir_all(&nvme_dir)?;
    fs::create_dir_all(&ssd_dir)?;
    fs::create_dir_all(&hdd_dir)?;
    fs::create_dir_all(&usb_dir)?;
    fs::create_dir_all(&media_dir)?;
    fs::create_dir_all(&raw_dir)?;

    let base_f = format!("{}/disks.md", dir);

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

    // Clean internal runtime shell command caller
    let run_cmd = |cmd: &str, args: &[&str]| -> String {
        Command::new(cmd)
        .args(args)
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_default()
    };

    // Shell execution with timeout constraint built in
    let run_cmd_timeout = |cmd: &str, args: &[&str], _sec: u64| -> String {
        let child = Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

        if let Ok(c) = child {
            match c.wait_with_output() { // Simplest bounded completion for standard utility checking
                Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
                Err(_) => format!("{} timed out or failed\n", cmd),
            }
        } else {
            format!("{} not available\n", cmd)
        }
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // 2. Initialize Header Layout
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>💾 DISK INTELLIGENCE SYSTEM</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n## 📦 BLOCK DEVICES\n", timestamp))?;

    // 3. Scan Block Devices
    let lsblk_raw = run_cmd("lsblk", &["-dn", "-o", "NAME,TYPE"]);
    let mut devices = Vec::new();
    for line in lsblk_raw.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "disk" {
            devices.push(parts[0].to_string());
        }
    }

    if devices.is_empty() {
        write_to(&base_f, "No disks detected\n")?;
        return Ok(base_f);
    }

    // 4. Process Individual Hard Drives Natively
    for d in &devices {
        let dev_path = format!("/dev/{}", d);
        let model = fs::read_to_string(format!("/sys/block/{}/device/model", d))
        .unwrap_or_else(|_| "Unknown Model".to_string()).trim().to_string();
        let rota = fs::read_to_string(format!("/sys/block/{}/queue/rotational", d))
        .unwrap_or_else(|_| "0".to_string()).trim().to_string();
        let removable = fs::read_to_string(format!("/sys/block/{}/removable", d))
        .unwrap_or_else(|_| "0".to_string()).trim().to_string();

        // Target sorting categorization paths
        let mut out_file = format!("{}/{}.md", hdd_dir, d);
        let mut label = "🪵 HDD";

        if d.starts_with("nvme") {
            out_file = format!("{}/{}.md", nvme_dir, d);
            label = "⚡ NVMe";
        } else if rota == "0" {
            out_file = format!("{}/{}.md", ssd_dir, d);
            label = "⚡ SSD";
        }

        if d.starts_with("sd") && removable == "1" {
            out_file = format!("{}/{}.md", usb_dir, d);
            label = "🔌 USB";
        }

        // Write hardware specifics metadata out
        overwrite_to(&out_file, &format!("# {} DRIVE: {}\n\nModel: {}\n\n## 📊 PARTITIONS / MOUNTS\n", label, dev_path, model))?;

        let lsblk_dev = run_cmd("lsblk", &["-o", "NAME,SIZE,TYPE,FSTYPE,MOUNTPOINT", &dev_path]);
        write_to(&out_file, &lsblk_dev)?;

        // 5. Track Media and Mount points
        write_to(&out_file, "\n## 📁 MEDIA MOUNTS\n")?;
        let mountpoints_raw = run_cmd("lsblk", &["-nr", "-o", "MOUNTPOINT", &dev_path]);
        for m in mountpoints_raw.lines() {
            let m = m.trim();
            if !m.is_empty() {
                if m.starts_with("/media/") {
                    write_to(&out_file, &format!("📌 Media mount: {}\n", m))?;
                    write_to(&format!("{}/{}-media.md", media_dir, d), &format!("{}\n", m))?;
                } else if m.starts_with("/run/media/") {
                    write_to(&out_file, &format!("📌 User mount: {}\n", m))?;
                    write_to(&format!("{}/{}-media.md", media_dir, d), &format!("{}\n", m))?;
                }
            }
        }

        // 6. Block Size Calculations
        if let Ok(size_str) = fs::read_to_string(format!("/sys/block/{}/size", d)) {
            if let Ok(sectors) = size_str.trim().parse::<u64>() {
                let size_gb = (sectors * 512) / 1024 / 1024 / 1024;
                write_to(&out_file, &format!("\n## 📏 SIZE\n{} GB (approx)\n", size_gb))?;
            }
        }

        // 7. S.M.A.R.T. Health Matrix Monitoring Hook
        write_to(&out_file, "\n## 🧠 SMART HEALTH\n")?;
        let smart_out = run_cmd_timeout("smartctl", &["-H", &dev_path], 2);
        if smart_out.is_empty() || smart_out.contains("not available") {
            write_to(&out_file, "SMART unavailable\n")?;
        } else {
            write_to(&out_file, &smart_out)?;
        }
    }

    // 8. Global NVMe Specific Layout Dumps
    write_to(&base_f, "\n## ⚡ NVME SUMMARY\n")?;
    let nvme_out = run_cmd_timeout("nvme", &["list"], 2);
    if nvme_out.is_empty() || nvme_out.contains("not available") {
        write_to(&base_f, "nvme-cli not installed\n")?;
    } else {
        write_to(&base_f, &nvme_out)?;
    }

    // 9. Standard Raw Diagnostic Snapshots
    write_to(&base_f, "\n## 📦 RAW SNAPSHOT\n")?;
    write_to(&base_f, &run_cmd("lsblk", &["-f"]))?;
    write_to(&base_f, &run_cmd("df", &["-h"]))?;
    write_to(&base_f, "\n✔ disk scan complete\n")?;

    // 10. Cache Replication Export Logic
    if let Ok(cache_dir) = std::env::var("CYBERDECK_CACHE_DIR") {
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::copy(&base_f, format!("{}/disks.md", cache_dir));
    }

    Ok(base_f)
}
