//! # CYBERDECK: Orchestrator
//!
//! The gateway for the diagnostic suite.
//! Aggregates results from individual modules to generate
//! the master system intelligence report.
//!
//! ## Implementation Notes
//! - **Registry**: Acts as the single interface for importing all diagnostic sub-systems.
//! - **Execution**: Provides the `execute_all()` function to loop through registered modules.

pub mod audio;
pub mod battery; // Added
pub mod bios;
pub mod cpu; // Added
pub mod dashboard;
pub mod disks;
pub mod fan;
pub mod hardware;
pub mod memory;
pub mod motherboard; // Added
pub mod network;
pub mod power;
pub mod services;
pub mod storage_ai;
pub mod thermal_ai;
