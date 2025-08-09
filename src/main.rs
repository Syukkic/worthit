use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

use crate::{
    model::Records,
    utils::{add_handler, delete_handler, set_handler, setup_records_file, show_handler},
};

mod cli;
mod display;
mod model;
mod utils;

fn main() -> Result<()> {
    // let record_path = "records.json".to_string();
    let record_path = setup_records_file()?;

    let cli = Cli::parse();
    let records = Records::load(&record_path)?;

    match cli.command {
        Commands::Add {
            name,
            price,
            purchase_date,
        } => add_handler(name, price, purchase_date, records, record_path)?,
        Commands::Set {
            name,
            new_name,
            price,
            purchase_date,
            status,
            repair_count,
            repair_cost,
            sold_price,
            sold_date,
        } => set_handler(
            name,
            new_name,
            price,
            purchase_date,
            status,
            repair_count,
            repair_cost,
            sold_price,
            sold_date,
            records,
            record_path,
        )?,
        Commands::Show => {
            show_handler(records);
        }
        Commands::Delete { name } => delete_handler(name, records, record_path)?,
    }

    Ok(())
}
