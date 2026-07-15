//! # CYBERDECK Dispatcher
//! The command execution engine for the CYBERDECK system.

use crate::types::{SharedCyberdeckState, CyberdeckCommand};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command; // Added for shell execution

use crate::modules::{
    audio, battery, bios, cpu, dashboard, disks, fan, hardware,
    memory, motherboard, network, power, services, storage_ai, thermal_ai
};

/// Executes a given `CyberdeckCommand`.
pub async fn execute_cyberdeck_command(cmd: CyberdeckCommand, state: &SharedCyberdeckState) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    match cmd {
        CyberdeckCommand::InitDisplay => {
            let mut s = state.lock().await;
            s.display_active = true;
            s.execution_log.push(format!("[{}] Display system activated.", timestamp));
        }
        CyberdeckCommand::SetStealth(mode) => {
            let mut s = state.lock().await;
            s.stealth_mode = mode;
            s.execution_log.push(format!("[{}] Stealth configuration set to: {}", timestamp, mode));
        }
        CyberdeckCommand::GenerateReport => {
            let mut s = state.lock().await;
            s.reports_generated += 1;
            let current_report_num = s.reports_generated;
            s.execution_log.push(format!("[{}] System Report #{} generated.", timestamp, current_report_num));
        }

        // --- NEW ARCHIVE LOGIC ---
        CyberdeckCommand::ArchiveFiles(format) => {
            let fmt = format.to_lowercase();

            // 1. Ensure output directory exists
            if let Err(e) = std::fs::create_dir_all("output") {
                tracing::error!("Failed to create output directory: {}", e);
                return;
            }

            // 2. Generate date-stamped filename
            let now = chrono::Local::now();
            let date_str = now.format("%Y-%m-%d").to_string();

            let file_ext = if fmt == "gzip" { "tar.gz" } else { &fmt };
            let filename = format!("cyberdeck_archive_{}.{}", date_str, file_ext);
            let output_path = format!("output/{}", filename);

            let mut s = state.lock().await;
            tracing::info!("Archiving to: {}", output_path);

            // 3. Execute system command
            let result = match fmt.as_str() {
                "zip" => Command::new("zip").args(["-r", &output_path, "./diagnostics"]).status(),
                "tar" => Command::new("tar").args(["-cvf", &output_path, "./diagnostics"]).status(),
                "gzip" => Command::new("tar").args(["-czvf", &output_path, "./diagnostics"]).status(),
                "7z" => Command::new("7z").args(["a", &output_path, "./diagnostics"]).status(),
                _ => {
                    tracing::error!("Unsupported format requested: {}", fmt);
                    s.execution_log.push(format!("[{}] Error: Unsupported format '{}'", timestamp, fmt));
                    return;
                }
            };

            // 4. Log results
            match result {
                Ok(status) if status.success() => {
                    tracing::info!("Archive successful: {}", filename);
                    s.execution_log.push(format!("[{}] Archive created: {}", timestamp, filename));
                }
                _ => {
                    tracing::error!("Compression command failed.");
                    s.execution_log.push(format!("[{}] Critical: Compression failed.", timestamp));
                }
            }
        }

        // --- Consolidated Module Execution ---
        CyberdeckCommand::RunAudioModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { audio::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunBatteryModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { battery::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunBiosModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { bios::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunCpuModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { cpu::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunDashboardModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { dashboard::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunDisksModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { disks::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunFanModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { fan::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunHardwareModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { hardware::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunMemoryModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { memory::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunMotherboardModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { motherboard::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunNetworkModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { network::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunPowerModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { power::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunServicesModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { services::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunStorageAiModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { storage_ai::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        CyberdeckCommand::RunThermalModule(p) => run_module(state, &p, |params| {
            let params = params.to_string(); let state_arc = state.clone();
            async move { thermal_ai::execute(&*state_arc.lock().await, &params).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)) }
        }).await,

        _ => println!("Command not implemented"),
    }
}

/// Helper for standard module execution.
async fn run_module<F, Fut>(state: &SharedCyberdeckState, p: &str, exec_fn: F)
where F: Fn(&str) -> Fut, Fut: std::future::Future<Output = std::io::Result<String>> {
    match exec_fn(p).await {
        Ok(res) => state.lock().await.execution_log.push(res),
        Err(e) => state.lock().await.execution_log.push(format!("Error: {}", e)),
    }
}
