use super::{error, info, section, success, warn};
use crate::{lsblk, prelude::*};

use chrono::Local;
use std::collections::HashSet;
use tokio::process::Command;

pub async fn handle_backup(
    source_path: Option<String>,
    destination_path: Option<String>,
    mut target_disks: Vec<String>,
    user_excludes: Vec<String>,
    open_after: bool,
) -> Result<()> {
    let settings = Settings::get();

    section("Backup Files");

    let source = source_path.unwrap_or_else(|| str!("."));
    let src_path = PathBuf::from(&source);
    if !src_path.exists() {
        return Err(Error::Operational(str!("Source path '{}' does not exist.", source)).into());
    }

    let full_source = src_path
        .canonicalize()
        .map_err(|e| str!("Failed to canonicalize path: {e}"))?;

    let src_base_name = full_source
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(Error::Operational(str!("Invalid source name").into()))?;

    let dest_suffix = destination_path.unwrap_or_else(|| str!(src_base_name));

    if target_disks.is_empty() {
        target_disks = settings.backup.default_disks.clone();
        if target_disks.is_empty() {
            return Err(Error::Operational(str!(
                "No target disks specified via -t and 'default_disks' in settings is empty."
            ))
            .into());
        }
    }

    let mut all_excludes = HashSet::new();

    if full_source.is_dir() {
        for ext in detect_automatic_excludes(&full_source) {
            all_excludes.insert(ext);
        }
    }

    for ext in user_excludes {
        all_excludes.insert(ext);
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let final_folder = format!("{src_base_name}__{timestamp}");

    for disk_identifier in &target_disks {
        info("Target   ", &disk_identifier.blue().to_string());

        let mut device = match lsblk::find(disk_identifier).await {
            Ok(dev) => dev,
            Err(_) => {
                println!(
                    "{} Drive '{}' not found in the system.",
                    " ->".yellow(),
                    disk_identifier
                );
                continue;
            }
        };

        if device.mountpoint.is_none() {
            warn("Drive is not mounted. Attempting to mount...");

            let mount_target = device
                .path
                .clone()
                .or_else(|| device.label.clone())
                .unwrap_or_else(|| device.name.clone());

            if let Err(e) = crate::commands::mount::handle_mount(mount_target, false).await {
                error(format!("Could not mount: {e}").into());
                continue;
            }

            if let Ok(updated_device) = lsblk::find(disk_identifier).await {
                device = updated_device;
            }
        }

        let mount_point = match device.mountpoint {
            Some(mp) => mp,
            None => {
                error("Drive found but mountpoint is still empty.".into());
                continue;
            }
        };

        let full_dest_path = PathBuf::from(&mount_point)
            .join(&settings.backup.default_dir)
            .join(&dest_suffix)
            .join(&final_folder);

        if let Err(e) = tokio::fs::create_dir_all(&full_dest_path).await {
            error(
                str!(
                    "Failed to create directory {}: {e}",
                    full_dest_path.display()
                )
                .into(),
            );
            continue;
        }

        info(
            "Destiny  ",
            &str!("{}", full_dest_path.to_string_lossy().blue()),
        );

        let mut rsync = Command::new("rsync");
        rsync.arg("-avz");

        if !all_excludes.is_empty() {
            info(
                "Excluding",
                &all_excludes
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            for exclude_item in &all_excludes {
                rsync.arg("--exclude").arg(exclude_item);
            }
        }

        let mut src_str = full_source.to_string_lossy().into_owned();
        if full_source.is_dir() && !src_str.ends_with('/') {
            src_str.push('/');
        }

        rsync.arg(src_str);
        rsync.arg(format!("{}/", full_dest_path.display()));

        println!();
        let status = rsync.status().await;

        match status {
            Ok(s) if s.success() => {
                println!();
                success(&str!(
                    "Mirrored to {}",
                    full_dest_path.to_string_lossy().blue()
                ));

                if open_after {
                    let _ = Command::new("xdg-open").arg(&full_dest_path).spawn();
                }
            }
            _ => {
                error(str!("rsync failed for drive '{}'", disk_identifier).into());
            }
        }
    }

    Ok(())
}

fn detect_automatic_excludes(source_dir: &Path) -> Vec<String> {
    let mut auto_excludes = Vec::new();

    // Rust / Cargo
    if source_dir.join("Cargo.toml").exists() {
        auto_excludes.push(str!("target"));
    }

    // Node.js
    if source_dir.join("package.json").exists() {
        auto_excludes.push(str!("node_modules"));
    }

    // Python (venv)
    if source_dir.join("venv").exists() {
        auto_excludes.push(str!("venv"));
    } else if source_dir.join(".venv").exists() {
        auto_excludes.push(str!(".venv"));
    }

    auto_excludes
}
