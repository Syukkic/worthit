use anyhow::Result;
use clap::Parser;
use cli::Cli;

use crate::{cli::CommandHandler, model::Records, utils::setup_records_file};

mod cli;
mod display;
mod model;
mod utils;

fn main() -> Result<()> {
    // let record_path = "records.json".to_string();
    let record_path = setup_records_file()?;

    let cli = Cli::parse();
    let records = Records::load(&record_path)?;
    cli.command.execute(records, record_path)?;

    Ok(())
}
