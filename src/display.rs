use crate::model::{Product, Status};
use chrono::Local;
use tabled::Tabled;

#[derive(Tabled)]
pub struct ProductTable {
    #[tabled(rename = "名稱")]
    name: String,

    #[tabled(rename = "價格")]
    price: String,

    #[tabled(rename = "購買日期")]
    purchase_date: String,

    #[tabled(rename = "狀態")]
    status: Status,

    // #[tabled(rename = "使用次數")]
    // usage_count: u32,
    #[tabled(rename = "維修次數")]
    repair_count: String,

    #[tabled(rename = "維修費")]
    repair_cost: String,

    #[tabled(rename = "轉手價")]
    sold_price: String,

    #[tabled(rename = "出售日期")]
    sold_date: String,

    #[tabled(rename = "持有天數")]
    days_held: i64,

    #[tabled(rename = "日均成本")]
    daily_cost: String,
}

impl ProductTable {
    pub fn from_product(product: &Product) -> Self {
        let today = Local::now().date_naive();
        let days_held = match product.sold_date {
            Some(sold_date) => (sold_date - product.purchase_date).num_days(),
            None => (today - product.purchase_date).num_days(),
        };

        let daily_cost = if days_held > 0 {
            product
                .sold_price
                .map_or(product.price, |sp| product.price - sp)
                / days_held as f64
        } else {
            0.0
        };

        ProductTable {
            name: product.name.clone(),
            price: format!("¥{:.2}", product.price),
            purchase_date: product.purchase_date.format("%Y-%m-%d").to_string(),
            status: product.status.clone().unwrap_or(Status::Active),
            // usage_count: product.usage_count.map_or(0, |v| v),
            repair_count: product
                .repair_count
                .map_or("0".to_string(), |v| v.to_string()),
            repair_cost: product
                .repair_cost
                .map_or("0".to_string(), |v| format!("¥{:.2}", v)),
            sold_price: product
                .sold_price
                .map_or("0".to_string(), |v| format!("¥{:.2}", v)),
            sold_date: product
                .sold_date
                .map(|sd| sd.format("%Y-%m-%d").to_string())
                .unwrap_or("未賣".to_string()),
            days_held,
            daily_cost: format!("¥{:.2}", daily_cost),
        }
    }
}
