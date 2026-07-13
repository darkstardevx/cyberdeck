use std::process::Command;

#[derive(Debug, PartialEq)]
pub enum ServiceCategory {
    WebServer,
    Infrastructure,
    Database,
    Security,
    Unknown,
}

pub struct ServiceItem {
    pub name: String,
    pub category: ServiceCategory,
    pub is_active: bool,
}

impl ServiceItem {
    pub fn perform_action(&self, action: &str) -> Result<(), String> {
        let status = Command::new("systemctl")
            .arg(action)
            .arg(&self.name)
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Failed to {} service {}", action, self.name))
        }
    }
}

pub fn discover_services() -> Vec<ServiceItem> {
    let output = Command::new("systemctl")
        .args([
            "list-units",
            "--type=service",
            "--state=running",
            "--no-pager",
            "--no-legend",
        ])
        .output()
        .expect("Failed to execute systemctl. Is systemd installed?");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts.first().unwrap_or(&"unknown").to_string();

            ServiceItem {
                category: classify_service(&name),
                name,
                is_active: true,
            }
        })
        .collect()
}

fn classify_service(name: &str) -> ServiceCategory {
    if name.contains("nginx") || name.contains("httpd") || name.contains("caddy") {
        ServiceCategory::WebServer
    } else if name.contains("postgresql") || name.contains("redis") || name.contains("mariadb") {
        ServiceCategory::Database
    } else if name.contains("sshd") || name.contains("ufw") || name.contains("fail2ban") {
        ServiceCategory::Security
    } else if name.contains("traefik") || name.contains("bind") || name.contains("dnsmasq") {
        ServiceCategory::Infrastructure
    } else {
        ServiceCategory::Unknown
    }
}
