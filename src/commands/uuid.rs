use crate::lsblk;
use crate::prelude::*;

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
