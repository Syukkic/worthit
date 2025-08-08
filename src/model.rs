use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result, anyhow};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Active,
    Idle,
    Broken,
    Sold,
}

impl Status {
    pub fn to_cantonees(&self) -> &str {
        match self {
            Self::Active => "用緊",
            Self::Idle => "食塵",
            Self::Broken => "壞咗",
            Self::Sold => "賣咗",
        }
    }

    pub fn from_u32(id: u32) -> Result<Self> {
        match id {
            0 => Ok(Self::Active),
            1 => Ok(Self::Idle),
            2 => Ok(Self::Broken),
            3 => Ok(Self::Sold),
            _ => Err(anyhow!("Invalid status id: {id}. Use 0, 1, 2 or 3")),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_cantonees())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub purchase_date: NaiveDate,
    pub status: Option<Status>,
    // pub usage_count: Option<u32>,
    pub repair_count: Option<u32>,
    pub repair_cost: Option<f64>,
    pub sold_price: Option<f64>,
    pub sold_date: Option<NaiveDate>,
}

impl Product {
    pub fn new(name: String, price: f64, purchase_date: NaiveDate) -> Self {
        Self {
            name,
            price,
            purchase_date,
            status: Some(Status::Active),
            // usage_count: None,
            repair_count: None,
            repair_cost: None,
            sold_price: None,
            sold_date: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        price: Option<f64>,
        purchase_date: Option<NaiveDate>,
        status: Option<Status>,
        // usage_count: Option<u32>,
        repair_count: Option<u32>,
        repair_cost: Option<f64>,
        sold_price: Option<f64>,
        sold_date: Option<NaiveDate>,
    ) {
        if let Some(p) = price {
            self.price = p;
        }
        if let Some(pd) = purchase_date {
            self.purchase_date = pd;
        }
        if let Some(s) = status {
            self.status = Some(s)
        }

        // self.usage_count = usage_count.or(self.usage_count);
        self.repair_count = repair_count.or(self.repair_count);
        self.repair_cost = repair_cost.or(self.repair_cost);
        self.sold_price = sold_price.or(self.sold_price);
        self.sold_date = sold_date.or(self.sold_date);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Records {
    pub products: HashMap<String, Product>,
}

impl Records {
    pub fn load(file_path: &str) -> Result<Self> {
        if !Path::new(file_path).exists() {
            return Ok(Self::new());
        }
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read `record.json` file: {}", file_path))?;
        let records: Records = serde_json::from_str(&content)?;

        Ok(records)
    }

    fn new() -> Self {
        Self {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) -> Result<()> {
        if self.products.contains_key(&product.name) {
            return Err(anyhow!(
                "product name `{}` already exists. choose another one",
                product.name
            ));
        }
        self.products.insert(product.name.clone(), product);
        Ok(())
    }

    pub fn save(&self, file_path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self).context("Failed to serialize")?;

        fs::write(file_path, content)
            .with_context(|| format!("Failed to write record(s) to: {}", file_path))
    }

    pub fn get_product_mut(&mut self, name: &str) -> Result<&mut Product> {
        self.products
            .get_mut(name)
            .ok_or_else(|| anyhow!("Product `{}` not found", name))
    }

    pub fn remove_product(&mut self, name: &str) -> Result<Product> {
        self.products
            .remove(name)
            .ok_or_else(|| anyhow!("Product `{}` not found", name))
    }

    pub fn list_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }
}
