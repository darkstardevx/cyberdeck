//! # Cyberdeck: Memory Module
//!
//! Profiles system RAM utilization and physical DIMM status.

use crate::diagnostics::run_cmd;
use std::fs;

pub async fn execute(dir: &str) -> std::io::Result<String> {
    let mem_dir = format!("{}/memory", dir);
    fs::create_dir_all(&mem_dir)?;

    let mem_report = format!("{}/memory.md", mem_dir);

    let free_out = run_cmd("free", &["-h"]);
    let dmi_mem = run_cmd("sudo", &["dmidecode", "-t", "memory"]);

    let content = format!("# 🧠 MEMORY\n\n## Usage\n{}\n\n## DIMM Details\n{}", free_out, dmi_mem);
    fs::write(&mem_report, content)?;

    Ok(mem_report)
}
