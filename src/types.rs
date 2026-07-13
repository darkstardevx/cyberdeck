use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CyberdeckModule {
    HardwareAudit,
    NetworkDiagnostics,
    StorageAnalysis,
    AudioDiagnostics,
    BiosDiagnostics,
    DisksDiagnostics,
    EthernetDiagnostics,
    FanDiagnostics,
    MemoryDiagnostics,
    PowerDiagnostics,
    StorageAiDiagnostics,
    ThermalDiagnostics,
    FullScan,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CyberdeckCommand {
    InitDisplay,
    SetStealth(bool),
    GenerateReport,

    // Ensure all these have (String) if you are passing data
    RunBatteryModule(String),
    RunCpuModule(String),
    RunMemoryModule(String),
    RunMotherboardModule(String),
    RunPowerModule(String),
    RunAudioModule(String),
    RunBiosModule(String),
    RunDisksModule(String),
    RunEthernetModule(String),
    RunFanModule(String),
    RunHardwareModule(String),
    RunNetworkModule(String),
    RunStorageAiModule(String),
    RunThermalModule(String),

    Unknown(String),
}

#[derive(Debug, Serialize, Clone)]
pub struct CyberdeckState {
    pub display_active: bool,
    pub active_modules: Vec<CyberdeckModule>,
    pub stealth_mode: bool,
    pub reports_generated: u32,
    pub execution_log: Vec<String>,
}

pub type SharedCyberdeckState = Arc<Mutex<CyberdeckState>>;
