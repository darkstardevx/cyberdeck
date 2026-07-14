//! # CYBERDECK Dispatcher
//!
//! The command execution engine for the CYBERDECK system.

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::types::CyberdeckState;

use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::{SharedCyberdeckState, CyberdeckCommand};
use crate::modules::{
    audio, battery, bios, cpu, dashboard, disks, fan, hardware,
    memory, motherboard, network, power, services, storage_ai, thermal_ai
};

/// Executes a given `CyberdeckCommand`.
/// Note: This function no longer locks the state globally, preventing deadlocks.
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

        // --- Consolidated Module Execution ---

        CyberdeckCommand::RunAudioModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    audio::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunBatteryModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    battery::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunBiosModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    bios::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunCpuModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    cpu::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunDashboardModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    dashboard::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunDisksModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    disks::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunFanModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    fan::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunHardwareModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    hardware::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunMemoryModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    memory::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunMotherboardModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    motherboard::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunNetworkModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    network::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunPowerModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    power::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunServicesModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    services::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunStorageAiModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    storage_ai::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

        CyberdeckCommand::RunThermalModule(p) => {
            run_module(state, &p, |params| {
                let params = params.to_string();
                let state_arc = state.clone();
                async move {
                    let locked_state = state_arc.lock().await;
                    thermal_ai::execute(&*locked_state, &params).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }).await
        },

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
