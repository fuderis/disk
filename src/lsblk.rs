use crate::prelude::*;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
pub struct Output {
    #[serde(rename = "blockdevices")]
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Device {
    pub name: String,
    pub path: Option<String>,
    pub label: Option<String>,
    pub uuid: Option<String>,
    pub fstype: Option<String>,
    pub size: Option<String>,
    pub mountpoint: Option<String>,
    #[serde(default)]
    pub children: Vec<Device>,
    pub fsused: Option<String>,
    #[serde(rename = "fsavail")]
    pub fsavail: Option<String>,
    #[serde(rename = "fsuse%")]
    pub fsuse_percent: Option<String>,
}

pub async fn list() -> Result<Vec<Device>> {
    let output = Command::new("lsblk")
        .args(["--json", "-O"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(str!("lsblk failed").into());
    }

    let output: Output = serde_json::from_slice(&output.stdout)?;

    Ok(output.devices)
}

pub async fn find(name: &str) -> Result<Device> {
    let devices = list().await?;

    find_recursive(&devices, name)
        .cloned()
        .ok_or(str!("Device '{name}' not found").into())
}

fn find_recursive<'a>(devices: &'a [Device], name: &str) -> Option<&'a Device> {
    for device in devices {
        if device.path.as_deref() == Some(name) || device.label.as_deref() == Some(name) {
            return Some(device);
        }

        if let Some(found) = find_recursive(&device.children, name) {
            return Some(found);
        }
    }

    None
}
