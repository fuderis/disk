use super::{info, section, success, warn};
use crate::{lsblk, prelude::*};

use std::process::ExitStatus;
use tokio::process::Command;

struct FsRepair {
    tool: &'static str,
    package: &'static str,
}

pub async fn handle_repair(device: String) -> Result<()> {
    let dev = lsblk::find(&device).await?;
    let dev_path = match dev.path.as_deref() {
        Some(path) => path,
        None => return Err(Error::Operational(str!("Device path is missing.")).into()),
    };

    section(&str!("Repairing {}", dev_path.blue().to_string()));

    let repair = match dev.fstype.as_deref() {
        Some("ntfs") => FsRepair {
            tool: "ntfsfix",
            package: "ntfsprogs",
        },
        Some("ext4") => FsRepair {
            tool: "e2fsck",
            package: "e2fsprogs",
        },
        Some("exfat") => FsRepair {
            tool: "fsck.exfat",
            package: "exfatprogs",
        },
        Some(fs) => {
            return Err(Error::Operational(str!(
                "Automatic repair for '{}' is not supported.",
                fs
            ))
            .into());
        }
        None => {
            return Err(Error::Operational(str!("Could not detect filesystem type.")).into());
        }
    };

    info(
        "Filesystem",
        &dev.fstype.as_ref().unwrap().blue().to_string(),
    );
    println!();

    ensure_tool(&repair).await?;

    let status = repair_fs(repair.tool, dev_path).await?;
    let mut code = status.code().unwrap_or(1);

    // e2fsck returns 1 when errors were fixed successfully.
    if repair.tool == "e2fsck" && code == 1 {
        code = 0;
    }

    if code != 0 {
        return Err(Error::Operational(str!("Repair utility exited with code {}.", code)).into());
    }

    println!();
    success("Filesystem repaired.");

    Ok(())
}

async fn ensure_tool(repair: &FsRepair) -> Result<()> {
    let status = Command::new("sh")
        .args(["-c", &format!("command -v {}", repair.tool)])
        .status()
        .await?;

    if status.success() {
        return Ok(());
    }

    warn(&str!("Required utility '{}' is missing.", repair.tool));
    info("Installing", &str!("{}...", repair.package.blue()));

    install_package(repair.package).await?;

    let status = Command::new("sh")
        .args(["-c", &format!("command -v {}", repair.tool)])
        .status()
        .await?;

    if !status.success() {
        return Err(Error::Operational(str!("Failed to install '{}'.", repair.tool)).into());
    }

    Ok(())
}

async fn install_package(package: &str) -> Result<()> {
    let managers = [
        (
            "pacman",
            vec!["pacman", "-Sy", "--needed", "--noconfirm", package],
        ),
        ("apt", vec!["apt", "install", "-y", package]),
        ("dnf", vec!["dnf", "install", "-y", package]),
        (
            "zypper",
            vec!["zypper", "--non-interactive", "install", package],
        ),
    ];

    for (manager, args) in managers {
        let exists = Command::new("sh")
            .args(["-c", &format!("command -v {manager}")])
            .status()
            .await?;

        if !exists.success() {
            continue;
        }

        if manager == "apt" {
            let _ = Command::new("sudo")
                .args(["apt", "update"])
                .status()
                .await?;
        }

        let status = Command::new("sudo").args(args).status().await?;

        if status.success() {
            return Ok(());
        }

        return Err(Error::Operational(str!("Failed to install '{}'.", package)).into());
    }

    Err(Error::Operational(str!("Unsupported package manager.")).into())
}

async fn repair_fs(tool: &str, dev: &str) -> Result<ExitStatus> {
    let mut cmd = Command::new("sudo");

    match tool {
        "ntfsfix" => {
            cmd.args(["ntfsfix", "-b", "-d", dev]);
        }
        "e2fsck" => {
            cmd.args(["e2fsck", "-p", dev]);
        }
        "fsck.exfat" => {
            cmd.args(["fsck.exfat", dev]);
        }
        _ => {
            return Err(Error::Operational(str!("Unsupported repair utility '{}'.", tool)).into());
        }
    }

    Ok(cmd.status().await?)
}
