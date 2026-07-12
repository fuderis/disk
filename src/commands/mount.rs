use crate::lsblk;
use crate::prelude::*;

use std::{env, path::Path};
use tokio::process::Command;

pub async fn handle_mount(device: String, open: bool) -> Result<()> {
    let dev = lsblk::find(&device).await?;

    if let Some(mount) = dev.mountpoint.as_deref() {
        println!("{} Already mounted at {}", " ->".green(), mount.blue());

        if open {
            let _ = Command::new("xdg-open").arg(mount).spawn();
        }

        return Ok(());
    }

    let dev_path = match dev.path.as_deref() {
        Some(path) => path,
        None => return Err(Error::Operational(str!("Device path is missing.")).into()),
    };

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

    println!("{} {} {}", "==>".blue(), "Mounting".bold(), dev_path.blue());

    let status = Command::new("sudo")
        .args(["timeout", "15", "mount", dev_path, &mount_path])
        .status()
        .await?;

    if status.success() {
        println!("{} Mounted at {}", " ->".green(), mount_path.blue(),);

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

    println!(
        "{} Read-write mount failed. Trying read-only...",
        " ->".yellow(),
    );

    let status = Command::new("sudo")
        .args(["timeout", "10", "mount", "-o", "ro", dev_path, &mount_path])
        .status()
        .await?;

    if status.success() {
        println!(
            "{} Mounted read-only at {}",
            " ->".green(),
            mount_path.blue(),
        );

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

    let dev_path = match dev.path.as_deref() {
        Some(path) => path,
        None => return Err(Error::Operational(str!("Device path is missing.")).into()),
    };

    let mountpoint = dev.mountpoint.clone();

    let status = Command::new("sudo")
        .args(["umount", dev_path])
        .status()
        .await?;

    if !status.success() {
        return Err(Error::Operational(str!("Failed to unmount '{}'.", device)).into());
    }

    if let Some(path) = mountpoint {
        if path.starts_with("/run/media/") && Path::new(&path).exists() {
            let _ = Command::new("sudo").args(["rmdir", &path]).status().await;
        }
    }

    println!("{} Unmounted {}", " ->".green(), dev_path.blue(),);

    Ok(())
}
