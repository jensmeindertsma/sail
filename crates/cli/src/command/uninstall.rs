use nix::unistd::Uid;
use std::{
    fs,
    path::Path,
    process::{Command, ExitCode},
};

pub fn uninstall() -> Result<(), UninstallError> {
    println!("Uninstalling....");

    if Uid::effective().is_root() {
        println!("Running as root.");
    } else {
        eprintln!("error: this program must be run as root.");
        return ExitCode::FAILURE;
    }

    let config_path = Path::new("/etc/sail");
    let var_path = Path::new("/var/lib/sail");
    let cli_path = Path::new("/usr/local/bin/sail");
    let daemon_path = Path::new("/usr/local/bin/saild");
    let systemd_service_path = Path::new("/etc/systemd/system/sail.service");
    let systemd_socket_path = Path::new("/etc/systemd/system/sail.socket");

    // Delete config files
    if config_path.exists() {
        fs::remove_dir_all(config_path).unwrap();
        println!("Deleted config at {}", config_path.display());
    }

    // Delete version file
    if var_path.exists() {
        fs::remove_dir_all(var_path).unwrap();
        println!("Deleted variables at {}", var_path.display());
    }

    // Delete binary files
    if cli_path.exists() {
        fs::remove_file(cli_path).unwrap();
        println!("Deleted binary at {}", cli_path.display());
    }

    if daemon_path.exists() {
        fs::remove_file(daemon_path).unwrap();
        println!("Deleted binary at {}", daemon_path.display());
    }

    // Delete systemd service file
    if systemd_service_path.exists() {
        fs::remove_file(systemd_service_path).unwrap();
        println!(
            "Deleted systemd service at {}",
            systemd_service_path.display()
        );
    }

    // Delete systemd socket file
    if systemd_socket_path.exists() {
        fs::remove_file(systemd_socket_path).unwrap();
        println!(
            "Deleted systemd socket at {}",
            systemd_socket_path.display()
        );
    }

    let output = Command::new("systemctl")
        .arg("stop")
        .arg("sail")
        .output()
        .unwrap();

    if output.status.success() {
        println!("Service stopped successfully.");
    } else {
        eprintln!(
            "Failed to stop service : {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = Command::new("systemctl")
        .arg("daemon-reload")
        .output()
        .unwrap();
    if output.status.success() {
        println!("Reloaded systemd successfully.");
    } else {
        eprintln!(
            "Failed to reload systemd: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = Command::new("systemctl")
        .arg("reset-failed")
        .arg("sail")
        .output()
        .unwrap();

    if output.status.success() {
        println!("Reset systemd successfully.");
    } else {
        eprintln!(
            "Failed to reset systemd: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub enum UninstallError {}
