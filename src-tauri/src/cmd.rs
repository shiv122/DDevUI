use std::process::Command;

/// Function to check if Docker is installed and running
#[tauri::command]
pub fn check_docker_status() -> Result<String, String> {
    // Define the docker command (to account for Windows compatibility)
    let docker_command = if cfg!(target_os = "windows") {
        "docker.exe" // Use docker.exe on Windows
    } else {
        "docker"
    };

    // Check if Docker is installed
    let output = Command::new(docker_command).arg("--version").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // Docker is installed; check if it's running
                let running_check = Command::new(docker_command).arg("info").output();

                match running_check {
                    Ok(info_output) => {
                        if info_output.status.success() {
                            Ok("Docker is installed and running".to_string())
                        } else {
                            Err("Docker is installed but not running".to_string())
                        }
                    }
                    Err(_) => Err("Failed to check if Docker is running".to_string()),
                }
            } else {
                Err("Docker is not installed".to_string())
            }
        }
        Err(_) => Err("Docker is not installed".to_string()),
    }
}
