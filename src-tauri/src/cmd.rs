use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;
use sysinfo::{System, SystemExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    os: String,
    architecture: String,
    docker_installed: bool,
    docker_running: bool,
    ddev_installed: bool,
}

#[tauri::command]
pub fn get_system_info() -> Result<String, String> {
    let mut system = System::new_all();
    system.refresh_all();

    // Determine OS and Architecture
    let os = std::env::consts::OS.to_string();

    let architecture = std::env::consts::ARCH.to_string();

    // Check Docker Installation
    let docker_installed = is_command_available("docker");

    // Check if Docker is running
    let docker_running = check_docker_running();

    // Check DDEV Installation
    let ddev_installed = is_command_available("ddev");

    // Create system info struct
    let system_info = SystemInfo {
        os,
        architecture,
        docker_installed,
        docker_running,
        ddev_installed,
    };

    // Convert to JSON string
    serde_json::to_string(&system_info)
        .map_err(|e| format!("Failed to serialize system info: {}", e))
}

// Helper function to check if a command is available
fn is_command_available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Helper function to check if Docker is running
fn check_docker_running() -> bool {
    Command::new("docker")
        .arg("info")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
