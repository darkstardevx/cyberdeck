//! # Cyberdeck: System Intelligence Module
//!
//! This module serves as the primary diagnostic engine for low-level system
//! identification and hardware state reporting. It probes motherboard firmware,
//! CPU states, memory, and power management interfaces.
//!
//! ## Implementation Details
//! - **DMI/Sysfs Integration**: Leverages `dmidecode` (requires root privileges) and
//!   `/sys/` interfaces to bypass high-level abstractions for raw hardware data.
//! - **Diagnostic Scoping**: Organizes outputs into modular directories (`cpu/`, `memory/`,
//!   `chipset/`, etc.) for granular subsystem analysis.
//! - **Frequency Snapshots**: Iterates through sysfs entries to calculate real-time
//!   core frequencies.
//!
//! ## Diagnostic Artifacts
//! The module generates a primary `bios.md` file and individual subsystem reports:
//! - `cpu/cpu.md`: Detailed processor architecture and scaling data.
//! - `motherboard/bios.md`: Firmware-level identification.
//! - `power/power.md`: CPU frequency governors and driver state.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Executes the full-stack system diagnostic suite.
///
/// Initializes the filesystem workspace, executes hardware-level commands,
/// and compiles both a primary system report and individual subsystem profiles.
///
/// # Arguments
/// * `dir` - The base path where the diagnostic directory structure will be initialized.
///
/// # Returns
/// * `Ok(String)` - The path to the primary `bios.md` diagnostic file.
/// * `Err(std::io::Error)` - Propagates filesystem creation or command execution errors.
pub async fn execute_bios_module(dir: &str) -> std::io::Result<String> {
    // 1. Initialize Subdirectory Tree Structure
    let cpu_dir = format!("{}/cpu", dir);
    let chipset_dir = format!("{}/chipset", dir);
    let board_dir = format!("{}/motherboard", dir);
    let mem_dir = format!("{}/memory", dir);
    let kernel_dir = format!("{}/kernel", dir);
    let power_dir = format!("{}/power", dir);

    fs::create_dir_all(&cpu_dir)?;
    fs::create_dir_all(&chipset_dir)?;
    fs::create_dir_all(&board_dir)?;
    fs::create_dir_all(&mem_dir)?;
    fs::create_dir_all(&kernel_dir)?;
    fs::create_dir_all(&power_dir)?;

    let base_f = format!("{}/bios.md", dir);
    let sub_bios_f = format!("{}/bios.md", board_dir);
    let sub_board_f = format!("{}/motherboard.md", board_dir);
    let sub_cpu_f = format!("{}/cpu.md", cpu_dir);
    let sub_chipset_f = format!("{}/chipset.md", chipset_dir);
    let sub_mem_f = format!("{}/memory.md", mem_dir);
    let sub_kernel_f = format!("{}/kernel.md", kernel_dir);
    let sub_power_f = format!("{}/power.md", power_dir);

    // Helper: Appends content to a file (creates if missing).
    let write_to = |path: &str, content: &str| -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    };

    // Helper: Overwrites a file (truncates existing content).
    let overwrite_to = |path: &str, content: &str| -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    };

    // Helper: Executes shell commands.
    let run_cmd = |cmd: &str, args: &[&str]| -> String {
        Command::new(cmd)
        .args(args)
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_else(|_| format!("{} not available\n", cmd))
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // 2. Base Header Configuration
    overwrite_to(&base_f, "<div style='background:#6a0dad;color:white;padding:6px;'>⚙️ BIOS & SYSTEM INTELLIGENCE</div>\n\n")?;
    write_to(&base_f, &format!("Timestamp: {}\n\n## 🧠 SYSTEM OVERVIEW\n", timestamp))?;

    let uname_all = run_cmd("uname", &["-a"]);
    write_to(&base_f, &uname_all)?;

    // 3. BIOS Records Extraction (DMI Decode via Sudo)
    write_to(&base_f, "\n## ⚙️ BIOS\n")?;
    let dmi_bios = run_cmd("sudo", &["dmidecode", "-t", "bios"]);
    write_to(&base_f, &dmi_bios)?;

    overwrite_to(&sub_bios_f, "# ⚙️ BIOS\n")?;
    write_to(&sub_bios_f, &dmi_bios)?;

    // 4. Motherboard Configuration Profiles
    write_to(&base_f, "\n## 🧩 MOTHERBOARD (ASUS PRIME B450M-A II)\n")?;
    let dmi_board = run_cmd("sudo", &["dmidecode", "-t", "baseboard"]);
    write_to(&base_f, &dmi_board)?;

    overwrite_to(&sub_board_f, "# 🧩 MOTHERBOARD\n")?;
    write_to(&sub_board_f, &dmi_board)?;

    // 5. Processor Diagnostics
    write_to(&base_f, "\n## 🧠 CPU\n")?;
    let lscpu_out = run_cmd("lscpu", &[]);
    write_to(&base_f, &lscpu_out)?;

    overwrite_to(&sub_cpu_f, "# 🧠 CPU\n")?;
    write_to(&sub_cpu_f, &lscpu_out)?;
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        write_to(&sub_cpu_f, &cpuinfo)?;
    }

    // 6. Chipset Hardware Components
    write_to(&base_f, "\n## 🔌 CHIPSET / PCI\n")?;
    let lspci_out = run_cmd("lspci", &[]);
    write_to(&base_f, &lspci_out)?;

    overwrite_to(&sub_chipset_f, "# 🔌 CHIPSET\n")?;
    write_to(&sub_chipset_f, &lspci_out)?;

    // 7. System Allocation Memory Profile
    write_to(&base_f, "\n## 🧠 MEMORY\n")?;
    let free_out = run_cmd("free", &["-h"]);
    write_to(&base_f, &free_out)?;

    overwrite_to(&sub_mem_f, "# 🧠 MEMORY\n")?;
    write_to(&sub_mem_f, &free_out)?;
    let dmi_mem = run_cmd("sudo", &["dmidecode", "-t", "memory"]);
    write_to(&sub_mem_f, &dmi_mem)?;

    // 8. Active Core Boot Parameters
    write_to(&base_f, "\n## 🧬 KERNEL\n")?;
    let uname_release = run_cmd("uname", &["-r"]);
    write_to(&base_f, &uname_release)?;
    let cmdline = fs::read_to_string("/proc/cmdline").unwrap_or_default();
    write_to(&base_f, &format!("{}\n", cmdline))?;

    overwrite_to(&sub_kernel_f, "# 🧬 KERNEL\n")?;
    write_to(&sub_kernel_f, &uname_all)?;
    write_to(&sub_kernel_f, &format!("{}\n", cmdline))?;

    // 9. Scaling Driver and Power Profile Mapping
    write_to(&base_f, "\n## 🔋 POWER & CPU CONTROL\n")?;
    overwrite_to(&sub_power_f, "# 🔋 POWER PROFILE\n")?;

    let cpupower_info = run_cmd("cpupower", &["frequency-info"]);
    if cpupower_info.contains("not available") {
        write_to(&base_f, "cpupower not installed\n")?;
    } else {
        write_to(&base_f, "### ⚙️ CPUPOWER INFO\n")?;
        write_to(&base_f, &cpupower_info)?;
        write_to(&sub_power_f, &cpupower_info)?;
    }

    write_to(&base_f, "\n### 🧠 AMD PSTATE\n")?;
    if let Ok(pstate_status) = fs::read_to_string("/sys/devices/system/cpu/amd_pstate/status") {
        write_to(&base_f, &pstate_status)?;
        write_to(&sub_power_f, &pstate_status)?;
    } else {
        write_to(&base_f, "amd_pstate not active\n")?;
    }

    if let Ok(driver) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_driver") {
        write_to(&base_f, &format!("\n### ⚡ SCALING DRIVER\n{}", driver))?;
    }
    if let Ok(governor) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor") {
        write_to(&base_f, &format!("\n### 🎯 ACTIVE GOVERNOR\n{}", governor))?;
    }
    if let Ok(avail_govs) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors") {
        write_to(&base_f, &format!("\n### 🎛️ AVAILABLE GOVERNORS\n{}", avail_govs))?;
    }

    // 10. Native Core Frequency Live Matrix Calculation Loop
    write_to(&base_f, "\n### 📊 CPU FREQUENCY SNAPSHOT\n")?;
    if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu") {
        for entry in entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit()) {
                let core_num = &name[3..];
                let freq_path = format!("/sys/devices/system/cpu/{}/cpufreq/scaling_cur_freq", name);
                if let Ok(raw_freq_str) = fs::read_to_string(freq_path) {
                    if let Ok(raw_freq) = raw_freq_str.trim().parse::<f64>() {
                        let mhz = raw_freq / 1000.0;
                        write_to(&base_f, &format!("Core {}: {:.2} MHz\n", core_num, mhz))?;
                    }
                }
            }
        }
    }

    Ok(base_f)
}
