use std::path::Path;
use std::process::Command;

/// Various init daemons
pub enum Daemon {
    /// Most systems are using systemd these days.  Those include:
    /// Arch, Debian, Fedora, Gentoo, Mageia, OpenSUSE, Ubuntu and probably others
    Systemd,
    /// Older versions of Ubuntu
    Upstart,
    /// Any daemon that we don't know how to detect yet
    Unknown,
}

pub fn detect_daemon() -> Result<Daemon, String> {
    // If this folder exists it is likely a systemd system
    if Path::new("/run/systemd/system").exists() {
        return Ok(Daemon::Systemd);
    }
    if Path::new("/sbin/initctl").exists() {
        // Check for upstart
        let mut cmd = Command::new("/sbin/initctl");
        cmd.arg("--version");
        let output = cmd.output().map_err(|e| e.to_string())?;
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout).into_owned();
            if output_str.contains("upstart") {
                return Ok(Daemon::Upstart);
            }
        }
    }
    return Ok(Daemon::Unknown);
}
