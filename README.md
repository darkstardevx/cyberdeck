# 🧊 CYBERDECK

<img src="images/screenshot.png" alt="Cyberdeck Screenshot" width="800">

### The Ultimate Cyberpunk Linux System Intelligence Framework

**CYBERDECK** is a modular Linux hardware and operating system intelligence framework written entirely in **Rust**. 

It performs deep hardware inspection, driver analysis, storage intelligence, thermal monitoring, network discovery, BIOS auditing, power analysis, and produces organized Markdown reports designed for both humans and AI systems.

---

## 📦 Features

- **Modular architecture**
- **AI-oriented reports**
- **Cyberpunk themed interface**
- **Individual module execution**
- **Full system scan mode**
- **Hierarchical report generation**
- **Automatic cache generation**
- **Markdown output**
- **Safe timeout protection**
- **Designed for Arch Linux** (Compatible with most modern Linux distributions)

---

## 📜 Current Modules

| Module | Description |
| :--- | :--- |
| **Hardware** | CPU, motherboard, PCI devices, USB devices |
| **Disks** | Disk classification, partitions, SMART, NVMe, USB media |
| **Storage AI** | Deep storage intelligence and filesystem analysis |
| **Memory** | RAM configuration and utilization |
| **Thermal AI** | Thermal zones, lm-sensors integration, temperature summaries |
| **Power** | CPU governors, P-State, frequencies, power states |
| **Network** | Interfaces, sockets, routing, DNS, driver information |
| **Ethernet** | Realtek 2.5Gb analysis, driver diagnostics, interface statistics |
| **Audio** | PipeWire/PulseAudio inspection |
| **BIOS** | Firmware, DMI, UEFI information |
| **Fan** | Cooling subsystem and fan detection |

---

## 📕 Output Structure

Each module generates its own directory and Markdown report:

```bash
output/
├── hardware/
├── disks/
├── network/
├── full_scan/
│   └── 2026-06-29_13-40-55/
│       ├── FULL_REPORT.md
│       ├── hardware/
│       ├── disks/
│       └── ...
```

---

## 💾 Intelligence Engines

### Disk Intelligence
Classifies devices into `nvme/`, `ssd/`, `hdd/`, `usb/`, `media/`, `raw/`.

### Thermal Intelligence
Automatically detects thermal zones, parses temperatures, calculates maximums, and falls back to `lm-sensors`.

### Power Intelligence
Collects CPU topology, governors, scaling driver, AMD P-State, frequencies, power states, system load, and UPower information with timeout protection.

### Storage AI
Analyzes block devices, filesystem usage, SMART status, encryption, NVMe devices, and filesystem hierarchy.

---

## 🏗️ Installation & Deployment

### Dependencies
Ensure you have the following system utilities installed:

**Debian / Ubuntu / Mint / Pop!_OS**
```bash
sudo apt update && sudo apt install lshw pciutils usbutils lm-sensors iproute2
```

**Arch Linux (pacman, yay, or paru)**
```bash
sudo pacman -Syu lshw pciutils usbutils lm_sensors iproute2
```

### Build & Run
```bash
# Clone the repository
git clone https://github.com/darkstardevx/cyberdeck.git
cd cyberdeck

# Run the panel (Opens @ 127.0.0.1:8080)
cargo run
```

### Execution
To run every module and generate a `FULL_REPORT.md`:
```bash
cyberdeck scan --full
```

---

## 🧊 CYBERDECK Roadmap

### Phase 1: Stabilization & Foundation
- [x] Dependency Audit: Implemented pre-flight system checks.
- [x] I/O Optimization: Refactored synchronous I/O to `tokio::task::spawn_blocking`.
- [x] Error Handling Refactor: Standardizing error responses.
- [x] Dashboard UI: Enhancing CSS/Grid layouts.

### Phase 2: Extensibility
*Goal: Enable users to write their own hardware probes.*
- [ ] Lua 0.55 Runtime Initialization: Integrating engine within the Rust core.
- [ ] FFI Bindings: Exposing safe Rust hardware hooks to Lua.
- [ ] Script Sandboxing: Implementing restricted execution.
- [ ] Plugin Hot-Reloading: Adding diagnostic scripts without restarting.

### Phase 3: Network & Ecosystem
*Goal: Build the platform beyond the local machine.*
- [ ] Subgridsec Telemetry Sync: Pushing reports to subgridsec.org.
- [ ] Remote Command Control: Authenticated, encrypted remote execution.
- [ ] Plugin Marketplace: Community-contributed Lua scripts.

### Phase 4: Hardening & Security
- [ ] Audit Trail: Persistent encrypted logging for commands.
- [ ] Cross-Distro Compatibility: Support for BSD and non-systemd.
- [ ] Performance Benchmarking: Profiling the Lua execution layer.

---

## 🤝 Contributing
Interested in Lua Module Development, UI/UX Design, or Documentation? Contact us:

**Contact Matrix:** [cybercore.sh+cyberdeck@gmail.com](mailto:cybercore.sh+cyberdeck@gmail.com)

---

## ⚖️ Namespace & Legal Attribution

This project is an independent component of the **Cybercore Systems Framework** hosted canonically at [subgridsec.org](https://subgridsec.org).

**Copyright (c) 2026 Cybercore Tech (subgridsec.org)**

<details>
<summary><b>🛡️ Defensive Guardrail Statement</b></summary>

All software components, tools, prefixes, and configurations under the "Cyber" prefix within this ecosystem are developed completely independently as open-source utilities for specialized terminal environments. They maintain absolutely no affiliation, partnership, endorsement, sponsorship, or commercial connection with any external corporate cybersecurity providers, training collectives, or federal defense contractors. Prior art is formally registered and maintained immutably via active domain publication.
</details>

---

## ⚖️ License
MIT License
