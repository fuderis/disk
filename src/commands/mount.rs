use super::{section, success, warn};
use crate::{lsblk, prelude::*};

use std::{env, path::Path};
use tokio::process::Command;

pub async fn handle_mount(device: String, open: bool) -> Result<()> {
    let dev = lsblk::find(&device).await?;
    let dev_path = match dev.path.as_deref() {
        Some(path) => path,
        None => return Err(Error::Operational(str!("Device path is missing.")).into()),
    };

    section(&format!("Mounting {}", dev_path.blue()));

    if let Some(mount) = dev.mountpoint.as_deref() {
        success(&format!("Already mounted at {}", mount.blue()));

        if open {
            let _ = Command::new("xdg-open").arg(mount).spawn();
        }

        return Ok(());
    }

    let mount_name = dev
        .label
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(&dev.name);

    let user = env::var("USER").unwrap_or_else(|_| "user".into());
    let mount_path = format!("/run/media/{user}/{mount_name}");

    let status = Command::new("sudo")
        .args(["mkdir", "-p", &mount_path])
        .status()
        .await?;

    if !status.success() {
        return Err(Error::Operational(str!("Failed to create mount directory.")).into());
    }

    let status = Command::new("sudo")
        .args(["timeout", "15", "mount", dev_path, &mount_path])
        .status()
        .await?;

    if status.success() {
        success(&format!("Mounted at {}", mount_path.blue()));

        if open {
            let _ = Command::new("xdg-open").arg(&mount_path).spawn();
        }

        return Ok(());
    }

    if status.code() == Some(124) {
        return Err(Error::Operational(str!(
            "Mount timed out. The filesystem is probably locked."
        ))
        .into());
    }

    warn("Read-write mount failed. Trying read-only...");

    let status = Command::new("sudo")
        .args(["timeout", "10", "mount", "-o", "ro", dev_path, &mount_path])
        .status()
        .await?;

    if status.success() {
        success(&format!("Mounted read-only at {}", mount_path.blue()));

        if open {
            let _ = Command::new("xdg-open").arg(&mount_path).spawn();
        }

        return Ok(());
    }

    Err(Error::Operational(str!(
        "Failed to mount '{}'. Try 'disk fix {}'.",
        device,
        device
    ))
    .into())
}

pub async fn handle_unmount(device: String) -> Result<()> {
    let dev = lsblk::find(&device).await?;

    let mountpoint = match dev.mountpoint.clone() {
        Some(mp) => mp,
        None => {
            return Err(Error::Operational(format!("Device '{}' is not mounted.", device)).into());
        }
    };

    let dev_path = match dev.path.as_deref() {
        Some(path) => path,
        None => return Err(Error::Operational(str!("Device path is missing.")).into()),
    };

    section(&format!("Unmounting {}", dev_path.blue()));

    let output = Command::new("sudo")
        .args(["umount", dev_path])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        if stderr.contains("not mounted") {
            return Err(Error::Operational(format!("Device '{}' is not mounted.", device)).into());
        }

        return Err(Error::Operational(format!(
            "Failed to unmount '{}': {}",
            device,
            stderr.trim()
        ))
        .into());
    }

    if mountpoint.starts_with("/run/media/") && Path::new(&mountpoint).exists() {
        let _ = Command::new("sudo")
            .args(["rmdir", &mountpoint])
            .status()
            .await;
    }

    success("Unmounted");

    Ok(())
}
