//! # CYBERDECK: Orchestrator
//!
//! The gateway for the diagnostic suite.
//! Aggregates results from individual modules to generate
//! the master system intelligence report.
//!
//! ## Implementation Notes
//! - **Registry**: Acts as the single interface for importing all diagnostic sub-systems.
//! - **Execution**: Provides the `execute_all()` function to loop through registered modules.

// 1. Declare the utility first
pub mod utils;

// 2. Declare all your other modules so the Dispatcher can find them
pub mod audio;
pub mod battery;
pub mod bios;
pub mod cpu;
pub mod dashboard;
pub mod disks;
pub mod fan;
pub mod hardware;
pub mod memory;
pub mod motherboard;
pub mod network;
pub mod power;
pub mod services;
pub mod storage_ai;
pub mod thermal_ai;
