use std::process::Command;

pub async fn execute_dashboard_module() -> std::io::Result<()> {
    // 1. Clear terminal screen
    let _ = Command::new("clear").status();

    let run_cmd = |cmd: &str, args: &[&str]| -> String {
        Command::new(cmd)
        .args(args)
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_default()
    };

    // 2. Banner Output
    println!("====================================");
    println!("     👁 CYBERDECK DASHBOARD");
    println!("====================================");
    println!();

    // 3. System Status
    println!("🧠 SYSTEM STATUS");
    print!("{}", run_cmd("uptime", &[]));
    println!();

    // 4. Processor Architecture Information
    println!("🔥 CPU");
    let lscpu_out = run_cmd("lscpu", &[]);
    if let Some(model_line) = lscpu_out.lines().find(|l| l.contains("Model name")) {
        println!("{}", model_line.trim());
    } else {
        println!("CPU Model Name tracking unavailable");
    }
    println!();

    // 5. Thermal Core Profiles
    println!("🌡 THERMALS");
    let sensors_out = run_cmd("sensors", &[]);
    if !sensors_out.is_empty() && !sensors_out.contains("not found") {
        for line in sensors_out.lines().take(10) {
            println!("{}", line);
        }
    } else {
        println!("lm-sensors interface missing");
    }
    println!();

    // 6. Block Storage Mapping
    println!("💾 DISKS");
    print!("{}", run_cmd("lsblk", &["-d", "-o", "NAME,SIZE,MODEL"]));
    println!();

    // 7. Network Stack Interface Status
    println!("🌐 NETWORK");
    print!("{}", run_cmd("ip", &["-br", "a"]));
    println!();

    // 8. Energy System Profiles
    println!("🔋 POWER");
    let upower_devices = run_cmd("upower", &["-e"]);
    if !upower_devices.is_empty() && !upower_devices.contains("not found") {
        if let Some(first_dev) = upower_devices.lines().next() {
            print!("{}", run_cmd("upower", &["-i", first_dev.trim()]));
        } else {
            println!("upower: no devices detected");
        }
    } else {
        println!("upower layer inactive");
    }

    // 9. Footer Sign-off Block
    println!();
    println!("====================================");
    println!("LIVE MODE ACTIVE");
    println!("====================================");

    Ok(())
}
