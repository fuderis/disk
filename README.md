# Disk CLI: Disk Management Utility

A lightweight command-line utility for managing block devices on Linux.
Disk CLI provides a simple interface for listing disks, mounting and unmounting filesystems, retrieving UUIDs,
and repairing common filesystem issues without remembering long shell commands.

## Key Features
  * **Device Discovery:** Browse all available block devices in a clean tree view.
  * **Filesystem Information:** Display labels, filesystem types, mount points, usage statistics, and UUIDs.
  * **Easy Mounting:** Mount devices by path or filesystem label.
  * **Safe Unmounting:** Unmount devices and automatically clean up temporary mount directories.
  * **Filesystem Repair:** Repair supported filesystems with automatic installation of required utilities.
  * **Human-Friendly Output:** Colored output with concise status messages inspired by classic Linux utilities.

## Commands & Usage

### List block devices
```bash
disk list
```

### Print filesystem UUID
```bash
disk uuid <DISK_LABEL>
```

### Mount a device
```bash
disk mount <DISK_LABEL>
```

### Mount and open in the file manager
```bash
disk mount --open <DISK_LABEL>
```

### Unmount a device
```bash
disk unmount <DISK_LABEL>
```

### Repair filesystem and mount afterwards
```bash
disk repair --open <DISK_LABEL>
```

### Backup a directory to target disks

```bash
# Backup current directory to a default disk defined in settings
disk backup

# Backup a specific folder to multiple target disks and manually exclude folders
disk backup /path/to/source --target disk1 --target disk2 --exclude "tmp" --exclude "cache"

# Backup and specify a custom destination path suffix on the target disk
disk backup /path/to/source custom_destination_dir
```

Devices can be specified either by their filesystem label or by their device path:

```bash
disk mount <DISK_LABEL>   # MyDisk
disk mount <DEVICE_PATH>  # /dev/sdb1
```

## Supported Filesystems

* ext2
* ext3
* ext4
* NTFS
* exFAT
* FAT32
* Most filesystems supported by the Linux kernel

### Automatic Repair

| Filesystem	| Utility     |
| :---        | :---        |
| ext4	      | e2fsck      |
| NTFS	      | ntfsfix     |
| exFAT	      | fsck.exfat  |

## Requirements

* Linux
  * `lsblk`
  * `mount`
  * `umount`
  * `sudo`

* Repair additionally requires
  * `e2fsprogs`
  * `ntfsprogs`
  * `exfatprogs`

* Rust toolchain (`cargo`)

Missing repair utilities are installed automatically using the system package manager when possible.

## Installation

Just run the following commands:

```bash
git clone https://github.com/fuderis/disk
cd disk
bash build.sh
```

## Configuration

The configuration file is located on:

```text
~/.config/disk/settings.toml
```

```toml
[backup]
default_disks = ["Backup"]
default_dir = "Backups"
```
 
## License & Feedback

> This software is distributed under the [GPL-3.0](LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
Contributions, bug reports, feature requests, and feedback are always welcome.
