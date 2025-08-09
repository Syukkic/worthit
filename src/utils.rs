use anyhow::{Context, Result};
use dirs::data_local_dir;

pub fn setup_records_file() -> Result<String> {
    let local_path = data_local_dir().context("Unable to determine the local directory")?;
    let worthit_dir = local_path.join("worthit");
    let records_file_path = worthit_dir.join("records.json");
    let record_path = records_file_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in record file path"))?;
    Ok(record_path.into())
}
