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
#[serde(tag = "type", content = "args")]
pub enum CyberdeckCommand {
    // ... (Other variants)
    InitDisplay,
    SetStealth(bool),
    GenerateReport,
    Unknown(String),

    // Diagnostic Modules
    RunAudioModule(String),
    RunBiosModule(String),
    RunDisksModule(String),
    RunEthernetModule(String),
    RunFanModule(String),
    RunHardwareModule(String),
    RunNetworkModule(String),
    RunStorageAiModule(String),
    RunThermalModule(String),
    RunFullScan(String),

    // --- ADD THESE MISSING VARIANTS ---
    RunBatteryModule(String),
    RunCpuModule(String),
    RunMotherboardModule(String),
    RunMemoryModule(String),
    RunPowerModule(String),

    RunModule(CyberdeckModule),
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
