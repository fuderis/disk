use super::section;
use crate::{
    lsblk::{self, Device},
    prelude::*,
};

struct DisplayDevice<'a> {
    name: &'a str,
    label: Option<&'a str>,
    fstype: Option<&'a str>,
    size: Option<&'a str>,
    used: Option<String>,
    free: Option<String>,
    mount: Option<&'a str>,
    children: &'a [Device],
}

pub async fn handle_list() -> Result<()> {
    let devices = lsblk::list().await?;

    section("Available Devices");
    println!();
    println!(
        "{:<16} {:<14} {:<8} {:<8} {:<14} {:<16} {}",
        "NAME", "LABEL", "FS", "SIZE", "USED", "FREE", "MOUNT",
    );

    println!("{}", "─".repeat(110));

    for (i, dev) in devices.iter().enumerate() {
        println!("{}", dev.name);

        print_children(&dev.children, "");

        if i + 1 != devices.len() {
            println!();
        }
    }

    Ok(())
}

fn print_children(devices: &[Device], prefix: &str) {
    for (i, dev) in devices.iter().enumerate() {
        let last = i + 1 == devices.len();

        let d = display_device(dev);

        println!(
            "{}{}{:<14} {:<14} {:<8} {:<8} {:<14} {:<16} {}",
            prefix,
            if last { "└ " } else { "├ " },
            d.name,
            d.label.unwrap_or("—"),
            d.fstype.unwrap_or("—"),
            d.size.unwrap_or("—"),
            d.used.as_deref().unwrap_or("—"),
            d.free.as_deref().unwrap_or("—"),
            d.mount.unwrap_or("—"),
        );

        let next_prefix = if last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };

        print_children(d.children, &next_prefix);
    }
}

fn display_device(dev: &Device) -> DisplayDevice<'_> {
    fn map_fstype(fstype: Option<&str>) -> Option<&str> {
        match fstype {
            Some("crypto_LUKS") => Some("luks"),
            other => other,
        }
    }

    // hide dm-crypt/luks-* layer
    if dev.fstype.as_deref() == Some("crypto_LUKS") && dev.children.len() == 1 {
        let child = &dev.children[0];

        return DisplayDevice {
            name: &dev.name,
            label: child.label.as_deref(),
            fstype: map_fstype(child.fstype.as_deref()),
            size: child.size.as_deref(),
            used: used(child),
            free: free(child),
            mount: child.mountpoint.as_deref(),
            children: &[],
        };
    }

    DisplayDevice {
        name: &dev.name,
        label: dev.label.as_deref(),
        fstype: map_fstype(dev.fstype.as_deref()),
        size: dev.size.as_deref(),
        used: used(dev),
        free: free(dev),
        mount: dev.mountpoint.as_deref(),
        children: &dev.children,
    }
}

fn used(dev: &Device) -> Option<String> {
    match (&dev.fsused, &dev.fsuse_percent) {
        (Some(used), Some(percent)) => Some(format!("{used} ({percent})")),
        (Some(used), None) => Some(used.clone()),
        _ => None,
    }
}

fn free(dev: &Device) -> Option<String> {
    match (&dev.fsavail, &dev.fsuse_percent) {
        (Some(free), Some(percent)) => {
            let p = percent.trim_end_matches('%');

            if let Ok(v) = p.parse::<u8>() {
                Some(format!("{free} ({}%)", 100 - v))
            } else {
                Some(free.clone())
            }
        }

        (Some(free), None) => Some(free.clone()),

        _ => None,
    }
}

pub async fn handle_uuid(device: String) -> Result<()> {
    let dev = lsblk::find(&device).await?;

    match dev.uuid {
        Some(uuid) => {
            println!("{uuid}");
            Ok(())
        }
        None => Err(Error::Operational(str!("Device '{}' has no UUID.", device)).into()),
    }
}
