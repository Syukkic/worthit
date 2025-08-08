use anyhow::{Context, Result};
use dirs::data_local_dir;
use std::path::PathBuf;

pub fn setup_records_file() -> Result<PathBuf> {
    let local_path = data_local_dir().context("Unable to determine the local directory")?;
    let worthit_dir = local_path.join("worthit");
    let records_file_path = worthit_dir.join("records.json");
    Ok(records_file_path)
}
