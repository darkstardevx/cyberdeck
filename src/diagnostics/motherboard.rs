//! # Cyberdeck: Motherboard Module
//!
//! Handles DMI extraction for BIOS and baseboard identity.

use crate::diagnostics::run_cmd;
use std::fs;

pub async fn execute(dir: &str) -> std::io::Result<String> {
    let board_dir = format!("{}/motherboard", dir);
    fs::create_dir_all(&board_dir)?;

    let bios_report = format!("{}/bios.md", board_dir);
    let board_report = format!("{}/motherboard.md", board_dir);

    let dmi_bios = run_cmd("sudo", &["dmidecode", "-t", "bios"]);
    let dmi_board = run_cmd("sudo", &["dmidecode", "-t", "baseboard"]);

    fs::write(&bios_report, format!("# ⚙️ BIOS\n\n{}", dmi_bios))?;
    fs::write(&board_report, format!("# 🧩 MOTHERBOARD\n\n{}", dmi_board))?;

    Ok(board_report)
}
