//! # CYBERDECK Dispatcher
//!
//! The command execution engine for the CYBERDECK system.
//!
//! This module acts as the central router for system operations, bridging the
//! gap between the `CyberdeckCommand` enum and specific hardware/diagnostic modules.
//! It manages asynchronous execution, thread-safe state mutation, and uniform
//! logging of system events.

use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::{SharedCyberdeckState, CyberdeckCommand};
// Consolidated imports: All modules are now under crate::modules
use crate::modules::{
    audio, battery, bios, cpu, dashboard, disks, fan, hardware,
    memory, motherboard, network, power, services, storage_ai, thermal_ai
};

/// Executes a given `CyberdeckCommand` by acquiring the shared state lock,
/// performing the requested operation via appropriate modules, and logging the outcome.
///
/// This function handles the lifecycle of command execution:
/// 1. Locks the `SharedCyberdeckState`.
/// 2. Records the initiation of the command in the execution log.
/// 3. Drops the lock to allow asynchronous execution of the module.
/// 4. Re-acquires the lock to log the final result of the operation.
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
            let current_report_num = s.reports_generated;
            log_event(&mut s.execution_log, format!("System Report #{} generated.", current_report_num));
        }

        CyberdeckCommand::RunAudioModule(p) => run_module(state, &p, |params| {
            // We clone the Arc to move it into the async block
            let state_arc = state.clone();
            async move {
                // Lock the state to get the underlying AppState/CyberdeckState
                let locked_state = state_arc.lock().await;

                // Pass the reference to the inner state
                audio::execute(&locked_state.app_state, params)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            }
        }).await,

        // --- Consolidated Module Execution ---

        // Standard Modules
        CyberdeckCommand::RunAudioModule(p) => run_module(state, &p, |params| audio::execute(state, params)).await,
        CyberdeckCommand::RunBatteryModule(p) => run_module(state, &p, |params| battery::execute(state, params)).await,
        CyberdeckCommand::RunBiosModule(p) => run_module(state, &p, |params| bios::execute(state, params)).await,
        CyberdeckCommand::RunCpuModule(p) => run_module(state, &p, |params| cpu::execute(state, params)).await,
        CyberdeckCommand::RunDashboardModule(p) => run_module(state, &p, |params| dashboard::execute(state, params)).await,
        CyberdeckCommand::RunDisksModule(p) => run_module(state, &p, |params| disks::execute(state, params)).await,
        CyberdeckCommand::RunFanModule(p) => run_module(state, &p, |params| fan::execute(state, params)).await,
        CyberdeckCommand::RunHardwareModule(p) => run_module(state, &p, |params| hardware::execute(state, params)).await,
        CyberdeckCommand::RunMemoryModule(p) => run_module(state, &p, |params| memory::execute(state, params)).await,
        CyberdeckCommand::RunMotherboardModule(p) => run_module(state, &p, |params| motherboard::execute(state, params)).await,
        CyberdeckCommand::RunNetworkModule(p) => run_module(state, &p, |params| network::execute(state, params)).await,
        CyberdeckCommand::RunPowerModule(p) => run_module(state, &p, |params| power::execute(state, params)).await,
        CyberdeckCommand::RunServicesModule(p) => run_module(state, &p, |params| services::execute(state, params)).await,
        CyberdeckCommand::RunStorageAiModule(p) => run_module(state, &p, |params| storage_ai::execute(state, params)).await,
        CyberdeckCommand::RunThermalModule(p) => run_module(state, &p, |params| thermal_ai::execute(state, params)).await,

        _ => println!("Command not implemented"),

        CyberdeckCommand::Unknown(u) => {
            log_event(&mut s.execution_log, format!("Unknown command: {}", u));

        }
    }
}

/// Helper that handles diagnostic module execution, logs the start, and captures output.
///
/// This function is specifically for modules that return `Result<String, std::io::Error>`.
/// It manages the drop/lock cycle to prevent holding the Mutex during long I/O operations.
async fn execute_and_log<F, Fut>(
    s: &mut crate::types::CyberdeckState,
    state: &SharedCyberdeckState,
    p: &str,
    name: &str,
    ts: u64,
    exec_fn: F
) where F: Fn(&str) -> Fut, Fut: std::future::Future<Output = std::io::Result<String>> {
    s.execution_log.push(format!("[{}] Launching {}: {}", ts, name, p));
    drop(s);

    let log_msg = match exec_fn(p).await {
        Ok(res) => format!("[{}] {}: {}", ts, name, res),
        Err(e) => format!("[{}] {}: Error: {}", ts, name, e),
    };

    state.lock().await.execution_log.push(log_msg);
}

/// Helper for standard module execution where the log message is provided by the module.
async fn run_module<F, Fut>(state: &SharedCyberdeckState, p: &str, exec_fn: F)
where F: Fn(&str) -> Fut, Fut: std::future::Future<Output = std::io::Result<String>> {
    match exec_fn(p).await {
        Ok(res) => state.lock().await.execution_log.push(res),
        Err(e) => state.lock().await.execution_log.push(format!("Error: {}", e)),
    }
}
