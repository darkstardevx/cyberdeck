//! # CYBERDECK Dispatcher
//!
//! The command execution engine for the CYBERDECK system.
//! This module handles the interpretation and routing of system commands,
//! managing state updates and asynchronous diagnostic/module execution.

#![warn(missing_docs)]

use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::{SharedCyberdeckState, CyberdeckCommand};
use crate::modules::{audio, bios, disks, ethernet, fan, hardware, network, storage_ai, thermal_ai};
use crate::diagnostics::{battery, cpu, motherboard, memory, power};

/// Executes a given CyberdeckCommand by acquiring the shared state lock,
/// performing the requested operation, and updating the system execution log.
pub async fn execute_cyberdeck_command(cmd: CyberdeckCommand, state: &SharedCyberdeckState) {
    let mut s = state.lock().await;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let log_event = |log: &mut Vec<String>, msg: String| {
        log.push(format!("[{}] {}", timestamp, msg));
    };

    match cmd {
        CyberdeckCommand::InitDisplay => {
            s.display_active = true;
            log_event(&mut s.execution_log, "Display system activated.".to_string());
        }
        CyberdeckCommand::SetStealth(mode) => {
            s.stealth_mode = mode;
            log_event(&mut s.execution_log, format!("Stealth configuration set to: {}", mode));
        }
        CyberdeckCommand::GenerateReport => {
            s.reports_generated += 1;
            // 1. Extract the value into a local variable first (this is an immutable copy)
            let current_report_num = s.reports_generated;

            // 2. Now pass that local variable to the formatter.
            // s is now free to be borrowed mutably by log_event.
            log_event(
                &mut s.execution_log,
                format!("System Report #{} generated.", current_report_num)
            );
        }

        // --- Diagnostics ---
        CyberdeckCommand::RunBatteryModule(p) => {
            log_event(&mut s.execution_log, format!("Launching Battery: {}", p));
            drop(s);
            if let Ok(res) = battery::execute(&p).await {
                state.lock().await.execution_log.push(format!("[{}] Battery: {}", timestamp, res));
            }
        }
        CyberdeckCommand::RunCpuModule(p) => {
            log_event(&mut s.execution_log, format!("Launching CPU: {}", p));
            drop(s);
            if let Ok(res) = cpu::execute(&p).await {
                state.lock().await.execution_log.push(format!("[{}] CPU: {}", timestamp, res));
            }
        }
        CyberdeckCommand::RunMemoryModule(p) => {
            log_event(&mut s.execution_log, format!("Launching Memory: {}", p));
            drop(s);
            if let Ok(res) = memory::execute(&p).await {
                state.lock().await.execution_log.push(format!("[{}] Memory: {}", timestamp, res));
            }
        }
        CyberdeckCommand::RunMotherboardModule(p) => {
            log_event(&mut s.execution_log, format!("Launching Motherboard: {}", p));
            drop(s);
            if let Ok(res) = motherboard::execute(&p).await {
                state.lock().await.execution_log.push(format!("[{}] Motherboard: {}", timestamp, res));
            }
        }
        CyberdeckCommand::RunPowerModule(p) => {
            log_event(&mut s.execution_log, format!("Launching Power: {}", p));
            drop(s);
            if let Ok(res) = power::execute(&p).await {
                state.lock().await.execution_log.push(format!("[{}] Power: {}", timestamp, res));
            }
        }

        // --- Modules ---
        CyberdeckCommand::RunAudioModule(p) => {
            drop(s);
            if let Ok(res) = audio::execute_audio_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunBiosModule(p) => {
            drop(s);
            if let Ok(res) = bios::execute_bios_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunDisksModule(p) => {
            drop(s);
            if let Ok(res) = disks::execute_disks_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunEthernetModule(p) => {
            drop(s);
            if let Ok(res) = ethernet::execute_ethernet_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunFanModule(p) => {
            drop(s);
            if let Ok(res) = fan::execute_fan_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunHardwareModule(p) => {
            drop(s);
            if let Ok(res) = hardware::execute_hardware_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunNetworkModule(p) => {
            drop(s);
            if let Ok(res) = network::execute_network_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunStorageAiModule(p) => {
            drop(s);
            if let Ok(res) = storage_ai::execute_storage_ai_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }
        CyberdeckCommand::RunThermalModule(p) => {
            drop(s);
            if let Ok(res) = thermal_ai::execute_thermal_ai_module(&p).await {
                state.lock().await.execution_log.push(res);
            }
        }

        CyberdeckCommand::Unknown(u) => {
            log_event(&mut s.execution_log, format!("Unknown command: {}", u));
        }
        _ => {}
    }
}
