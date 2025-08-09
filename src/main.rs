use anyhow::{Context, Result, anyhow};
use chrono::NaiveDate;
use clap::Parser;
use cli::{Cli, Commands};
use tabled::Table;

use crate::{
    display::ProductTable,
    model::{Product, Records, Status},
    utils::setup_records_file,
};

mod cli;
mod display;
mod model;
mod utils;

fn main() -> Result<()> {
    // let record_path = "records.json";
    let record_path = setup_records_file()?;

    let cli = Cli::parse();
    let mut records = Records::load(&record_path)?;

    match cli.command {
        Commands::Add {
            name,
            price,
            purchase_date,
        } => {
            if name.is_empty() {
                anyhow::bail!("product name can't not be empty.")
            }
            if price <= 0.0 {
                anyhow::bail!("price must be greater than 0.")
            }
            let purchase_date = NaiveDate::parse_from_str(&purchase_date, "%Y-%m-%d")
                .with_context(|| {
                    format!(
                        "Invalid date format. Please use YYYY-M-D, e.g. 2023-1-9. You entered: {}",
                        purchase_date
                    )
                })?;
            let product = Product::new(name, price, purchase_date);

            records.add_product(product)?;
            records.save(&record_path)?;
        }
        Commands::Set {
            name,
            price,
            purchase_date,
            status,
            repair_count,
            repair_cost,
            sold_price,
            sold_date,
        } => {
            let product_name = name.as_ref().ok_or_else(|| anyhow!("Name is required"))?;
            let product = records.get_product_mut(product_name)?;

            let validated_price = price
                .map(|price| {
                    (price > 0.0)
                        .then_some(price)
                        .ok_or_else(|| anyhow!("price must be greater than 0"))
                })
                .transpose()?;

            let parsed_purchase_date = purchase_date
                    .map(|date_str| {
                        NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").with_context(|| anyhow!(
                            "Invalid date format. Please use YYYY-M-D, e.g. 2023-1-9. You entered: {}",
                            date_str
                        ))
                    })
                    .transpose()?;

            let parsed_status = status.map(Status::from_u32).transpose()?;

            let validated_repair_cost = repair_cost
                .map(|cost| {
                    (cost > 0.0)
                        .then_some(cost)
                        .ok_or_else(|| anyhow!("cost price must be greater than 0"))
                })
                .transpose()?;

            // Begin ===============================================
            let sold_price_result = sold_price.map(|price| {
                (price > 0.0)
                    .then_some(price)
                    .ok_or_else(|| anyhow!("sold price can't be negative"))
            });
            let sold_date_result = sold_date.map(|date_str| {
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").with_context(|| {
                    anyhow!(
                        "Invalid date format. Please use YYYY-M-D, e.g. 2023-1-9. You entered: {}",
                        date_str
                    )
                })
            });
            match (&sold_date_result, &sold_price_result) {
                // sold_date_result 有值，但 sold_price_result 是 None
                // 或是 sold_price_result 有值，但 sold_date_result 是 None
                (Some(_), None) | (None, Some(_)) => {
                    if product.sold_price.is_none() && product.sold_date.is_none() {
                        anyhow::bail!(
                            "Both `sold-price` and `sold-date` must be provided together when setting for the first time"
                        );
                    }
                }
                _ => {}
            }
            let validated_sold_price = sold_price_result.transpose()?;
            let parsed_sold_date = sold_date_result.transpose()?;
            // =============================================== End

            product.update(
                validated_price,
                parsed_purchase_date,
                parsed_status,
                // usage_count,
                repair_count,
                validated_repair_cost,
                validated_sold_price,
                parsed_sold_date,
            );
            records.save(&record_path)?;
        }
        Commands::Show => {
            let products: Vec<ProductTable> = records
                .list_products()
                .iter()
                .map(|p| ProductTable::from_product(p))
                .collect();

            if products.is_empty() {
                println!("No products found");
            } else {
                println!("{}", Table::new(products))
            }
        }
        Commands::Delete { name } => {
            if let Some(name) = name {
                records.remove_product(&name)?;
                records.save(&record_path)?;
            } else {
                let product_names: Vec<String> = records.products.keys().cloned().collect();
                if product_names.is_empty() {
                    println!("No product to delete.");
                    return Ok(());
                }

                let target = dialoguer::Select::new()
                    .with_prompt("Choose product to delete")
                    .items(&product_names)
                    .interact()?;

                let selected_name = &product_names[target];
                records.remove_product(selected_name)?;
                let _ = records.save(&record_path);
                println!("Deleted product: {}", selected_name);
            }
        }
    }

    Ok(())
}
