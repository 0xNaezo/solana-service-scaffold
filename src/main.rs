use anyhow::{Ok, Result};
use solana_service_scaffold::configuration::Settings;

fn main() -> Result<()> {
    let _config = Settings::load()?;

    Ok(())
}
