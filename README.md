# 👁 CYBERDECK

### The Ultimate Cyberpunk Linux System Intelligence Framework

> **CYBERDECK** is a modular Linux hardware and operating system intelligence framework written entirely in **Fish Shell**.
> 
> It performs deep hardware inspection, driver analysis, storage intelligence, thermal monitoring, network discovery, BIOS auditing, power analysis, and produces organized Markdown reports designed for both humans and AI systems.

---

# Features

- Modular architecture
- AI-oriented reports
- Cyberpunk themed interface
- Individual module execution
- Full system scan mode
- Hierarchical report generation
- Automatic cache generation
- Markdown output
- Safe timeout protection
- Designed for Arch Linux but compatible with most modern Linux distributions

---

# Current Modules

| Module     | Description                                                      |
| ---------- | ---------------------------------------------------------------- |
| Hardware   | CPU, motherboard, PCI devices, USB devices                       |
| Disks      | Disk classification, partitions, SMART, NVMe, USB media          |
| Storage AI | Deep storage intelligence and filesystem analysis                |
| Memory     | RAM configuration and utilization                                |
| Thermal AI | Thermal zones, lm-sensors integration, temperature summaries     |
| Power      | CPU governors, P-State, frequencies, power states                |
| Network    | Interfaces, sockets, routing, DNS, driver information            |
| Ethernet   | Realtek 2.5Gb analysis, driver diagnostics, interface statistics |
| Audio      | PipeWire/PulseAudio inspection                                   |
| BIOS       | Firmware, DMI, UEFI information                                  |
| Fan        | Cooling subsystem and fan detection                              |

---

# Output Structure

```
output/

    hardware/
    disks/
    network/
    ethernet/
    power/
    thermal_ai/
    storage_ai/
    audio/
    memory/
    bios/
    fan/

    full_scan/

        2026-06-29_13-40-55/

            FULL_REPORT.md

            hardware/
            disks/
            network/
            ethernet/
            power/
            thermal_ai/
            storage_ai/
            memory/
            bios/
            fan/
            audio/
```

Each module generates its own directory and Markdown report.

---

# Disk Intelligence

The disk engine automatically classifies devices into

```
nvme/
ssd/
hdd/
usb/
media/
raw/
```

Each detected drive receives its own report.

Example:

```
nvme/

    nvme0n1.md

ssd/

    sda.md

usb/

    sdb.md
```

Media mounted under

```
/media
```

or

```
/run/media
```

are automatically identified.

---

# Thermal Intelligence

Thermal AI automatically

- detects thermal zones
- parses temperatures
- calculates maximum temperature
- falls back to lm-sensors
- creates parsed summaries
- creates raw sensor dumps

Output:

```
thermal_ai/

    thermal_ai.md

    raw/

    parsed/
```

---

# Power Intelligence

The Power module collects

- CPU topology
- governors
- scaling driver
- AMD P-State
- CPU frequencies
- power states
- system load
- UPower information
- raw power data

Timeout protection prevents hanging commands.

---

# Storage AI

Storage AI analyzes

- block devices
- filesystem usage
- SMART status
- encryption
- NVMe devices
- removable storage
- filesystem hierarchy

---

# Network Intelligence

Reports include

- interfaces
- routing
- DNS
- sockets
- driver information
- link speed
- duplex
- interface statistics

---

# Ethernet Intelligence

Designed specifically around modern Realtek 2.5Gb controllers including

- r8125
- r8169 conflict detection
- link speed
- driver analysis
- statistics
- ethtool integration

---

# Full Scan

Run every module

```
oms
```

Select

```
full_scan
```

The framework generates

```
FULL_REPORT.md
```

containing links to every module report.

---

# Cache System

CYBERDECK now supports a centralized cache directory.

```
cache/

    hardware.md
    disks.md
    network.md
    power.md
    thermal_ai.md
```

The cache provides quick access to the latest module output without navigating timestamped folders.

Environment variables

```
CYBERDECK_ROOT
CYBERDECK_CACHE_DIR
```

---

# Core Framework

```
core/

    env.fish
    init.fish
    output_writer.fish
    hud.fish
```

The Core initializes

- environment variables
- cache locations
- shared helper functions
- output rendering
- future dashboard components

---

# Safe Execution

Modules are executed with timeout protection.

Features

- prevents hanging modules
- debug logging
- safe execution wrappers
- isolated module failures

---

# Report Format

All reports are now generated as

```
Markdown (.md)
```

No legacy `.txt` reports remain.

Reports are organized into sections with:

- headings
- summaries
- parsed data
- raw data
- AI-friendly formatting

---

# Dashboard (In Development)

The next major milestone is Dashboard Mode.

Planned features include

- Live system overview
- CPU graphs
- Memory graphs
- Thermal gauges
- Disk utilization
- Network throughput
- Alert notifications
- Hardware health scoring
- AI-generated recommendations
- Terminal dashboard
- Web dashboard
- Rich HTML reports
- Historical scan comparisons

---

# Design Goals

- Modular
- Fast
- AI-friendly
- Human-readable
- Markdown-native
- Easy to extend
- Linux-first
- Cyberpunk aesthetic

---

# Planned Modules

- GPU Intelligence
- Security AI
- Kernel AI
- Services
- Processes
- Packages
- Containers
- Virtual Machines
- Wireless
- Bluetooth
- USB Intelligence
- Filesystem Health
- Battery Intelligence
- Performance AI
- Scheduler Analysis
- Boot Analyzer
- Cloud Detection
- Docker
- Kubernetes
- Virtualization
- Benchmark Suite

---

# Long-Term Vision

CYBERDECK is evolving into a complete Linux intelligence platform rather than a collection of shell scripts.

Future releases aim to provide:

- Live dashboard mode
- Historical scan database
- Hardware health scoring
- AI-generated diagnostics
- Trend analysis
- Automated report generation
- HTML report rendering
- REST API
- Plugin architecture
- Theme engine
- Multi-host inventory
- Remote scanning
- Fleet management

---

# License

MIT License

---

# Built With

- Fish Shell
- Linux sysfs
- procfs
- lsblk
- lspci
- lsusb
- smartctl
- nvme-cli
- lm-sensors
- upower
- PipeWire
- ethtool
- systemd utilities

---

**CYBERDECK** isn't just a system information tool—it's evolving into a modular Linux hardware intelligence platform with AI-friendly reporting, designed for power users, system administrators, and hardware enthusiasts.
