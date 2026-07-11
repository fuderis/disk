pub mod error;
pub mod lsblk;
pub mod prelude;
pub mod settings;

pub mod commands;

use clap::{Parser, Subcommand};
use prelude::*;

pub const APP_NAME: &str = "disk";
pub const APP_VERSION: &str = "0.1.0";

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
}

#[tokio::main]
async fn main() -> Result<()> {
    use commands as cmds;

    Settings::init(path!("$config$/settings.toml")).await?;

    let cli = Cli::parse();
    let result = match cli.command {
        Commands::List => cmds::list::handle_list().await,
        Commands::Uuid { device } => cmds::uuid::handle_uuid(device).await,
        Commands::Mount { open, device } => cmds::mount::handle_mount(device, open).await,
        Commands::Unmount { device } => cmds::mount::handle_unmount(device).await,
        Commands::Repair { device } => cmds::fix::handle_repair(device).await,
    };

    if let Err(e) = result {
        println!("{} {e}", " ->".red());
    }

    Ok(())
}
