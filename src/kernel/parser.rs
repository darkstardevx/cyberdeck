use crate::types::{CyberdeckCommand, CyberdeckModule};

pub fn parse_cyberdeck_script(script: &str) -> Vec<CyberdeckCommand> {
    let mut commands = Vec::new();

    for raw_cmd in script.split(';') {
        let trimmed = raw_cmd.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        match parts[0] {
            "init_display" => commands.push(CyberdeckCommand::InitDisplay),
            "generate_report" => commands.push(CyberdeckCommand::GenerateReport),
            "stealth_on" => commands.push(CyberdeckCommand::SetStealth(true)),
            "stealth_off" => commands.push(CyberdeckCommand::SetStealth(false)),

            // --- NATIVE DIRECT LOCATION MODULES ---
            "run_audio" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunAudioModule,
            ),
            "run_bios" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunBiosModule,
            ),
            "run_disks" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunDisksModule,
            ),
            "run_ethernet" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunEthernetModule,
            ),
            "run_fan" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunFanModule,
            ),
            "run_hardware" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunHardwareModule,
            ),
            "run_memory" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunMemoryModule,
            ),
            "run_network" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunNetworkModule,
            ),
            "run_power" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunPowerModule,
            ),
            "run_storage_ai" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunStorageAiModule,
            ),
            "run_thermal" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunThermalModule,
            ),
            "run_full_scan" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunFullScan,
            ),

            // --- NEW DIAGNOSTIC MODULES ---
            "run_battery" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunBatteryModule,
            ),
            "run_cpu" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunCpuModule,
            ),
            "run_motherboard" => parse_path_cmd(
                &parts,
                trimmed,
                &mut commands,
                CyberdeckCommand::RunMotherboardModule,
            ),

            "run_module" => {
                if let Some(&module_name) = parts.get(1) {
                    match module_name {
                        "hardware" => commands
                            .push(CyberdeckCommand::RunModule(CyberdeckModule::HardwareAudit)),
                        "network" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::NetworkDiagnostics,
                        )),
                        "storage" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::StorageAnalysis,
                        )),
                        "audio" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::AudioDiagnostics,
                        )),
                        "bios" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::BiosDiagnostics,
                        )),
                        "disks" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::DisksDiagnostics,
                        )),
                        "ethernet" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::EthernetDiagnostics,
                        )),
                        "fan" => commands
                            .push(CyberdeckCommand::RunModule(CyberdeckModule::FanDiagnostics)),
                        "memory" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::MemoryDiagnostics,
                        )),
                        "power" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::PowerDiagnostics,
                        )),
                        "storage_ai" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::StorageAiDiagnostics,
                        )),
                        "thermal" => commands.push(CyberdeckCommand::RunModule(
                            CyberdeckModule::ThermalDiagnostics,
                        )),
                        "full_scan" => {
                            commands.push(CyberdeckCommand::RunModule(CyberdeckModule::FullScan))
                        }
                        _ => commands.push(CyberdeckCommand::Unknown(trimmed.to_string())),
                    }
                } else {
                    commands.push(CyberdeckCommand::Unknown(trimmed.to_string()));
                }
            }
            _ => commands.push(CyberdeckCommand::Unknown(trimmed.to_string())),
        }
    }
    commands
}

fn parse_path_cmd<F>(parts: &[&str], raw: &str, commands: &mut Vec<CyberdeckCommand>, f: F)
where
    F: Fn(String) -> CyberdeckCommand,
{
    if let Some(&dir_path) = parts.get(1) {
        commands.push(f(dir_path.to_string()));
    } else {
        commands.push(CyberdeckCommand::Unknown(raw.to_string()));
    }
}
