pub mod error;
pub mod lsblk;
pub mod prelude;
pub mod settings;

pub mod commands;

use clap::{Parser, Subcommand};
use prelude::*;

pub const APP_NAME: &str = "disk";
pub const APP_VERSION: &str = "0.2.0";

#[derive(Parser, Debug)]
#[command(
    name = APP_NAME,
    version = APP_VERSION,
    about = "Linux disk management utility"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List block devices
    List,

    /// Print UUID of device
    Uuid {
        /// Device path or filesystem label
        device: String,
    },

    /// Mount device
    Mount {
        /// Open mount directory after mounting
        #[arg(short, long)]
        open: bool,

        /// Device path or filesystem label
        device: String,
    },

    /// Unmount device
    Unmount {
        /// Device path or filesystem label
        device: String,
    },

    /// Repair filesystem
    Repair {
        /// Device path or filesystem label
        device: String,
    },

    /// Backup source directory to target disk(s)
    Backup {
        /// Source path to backup (defaults to current directory)
        source_path: Option<String>,

        /// Subdirectory name on target drive (defaults to settings.default_label)
        destination_path: Option<String>,

        /// Target disk(s) identifier, label, or UUID. Can be specified multiple times
        /// (if not provided, uses 'backup_disks' from settings)
        #[arg(short, long)]
        target: Vec<String>,

        /// Folders to exclude from backup (can be specified multiple times)
        #[arg(short, long)]
        exclude: Vec<String>,

        /// Open destination folder after backup
        #[arg(short, long)]
        open: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    use commands as cmds;

    Settings::init(path!("$config$/settings.toml")).await?;

    let cli = Cli::parse();
    let result = match cli.command {
        Commands::List => cmds::uuid::handle_list().await,
        Commands::Uuid { device } => cmds::uuid::handle_uuid(device).await,
        Commands::Mount { open, device } => cmds::mount::handle_mount(device, open).await,
        Commands::Unmount { device } => cmds::mount::handle_unmount(device).await,
        Commands::Repair { device } => cmds::fix::handle_repair(device).await,
        Commands::Backup {
            source_path,
            destination_path,
            target,
            exclude,
            open,
        } => {
            cmds::backup::handle_backup(source_path, destination_path, target, exclude, open).await
        }
    };

    if let Err(e) = result {
        cmds::error(e);
    }

    Ok(())
}
