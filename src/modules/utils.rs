// modules/utils.rs
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

// Your existing header function
pub fn write_header(file: &mut File, status: &str) -> std::io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let header = format!(
        r#"
        /* ========================================================== */
        /* CYBERDECK INTERNAL DIAGNOSTIC SYSTEM                       */
        /* STATUS: {} | SYS_ID: {:X}                                  */
        /* ========================================================== */
        "#,
        status, timestamp
    );
    writeln!(file, "{}", header)
}

// THE NEW HELPER: Standardizes initialization
pub fn init_diagnostic_file(dir: &str, filename: &str) -> Result<File, String> {
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let path = format!("{}/{}", dir, filename);
    let mut file = File::create(&path).map_err(|e| e.to_string())?;

    // Auto-inject header
    write_header(&mut file, "ACTIVE").map_err(|e| e.to_string())?;

    Ok(file)
}
