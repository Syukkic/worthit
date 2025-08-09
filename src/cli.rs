use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "記錄買過咩東西",
    long_about = "大額商品總是希望可以耐用不易壞，記錄買過啲貴嘢睇下可以用幾耐",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        #[arg(short = 'n', long, help = "佢個名")]
        name: String,

        #[arg(short = 'p', long, help = "幾钱")]
        price: f64,

        #[arg(long = "pd", help = "幾時買，例如2024-1-6")]
        purchase_date: String,
    },
    Set {
        #[arg(short = 'n', long = "current-name", help = "佢個名")]
        name: Option<String>,

        #[arg(long = "new-name", help = "起個新名")]
        new_name: Option<String>,

        #[arg(short = 'p', long, help = "幾钱")]
        price: Option<f64>,

        #[arg(long, help = "幾時買，例如2024-1-6")]
        purchase_date: Option<String>,

        #[arg(
            short,
            long,
            help = "0: 用緊, 1: 食塵, 2: 壞咗, 3: 賣咗",
            default_value = "0"
        )]
        status: Option<u32>,

        #[arg(long, help = "整咗幾次")]
        repair_count: Option<u32>,

        #[arg(long, help = "維修費")]
        repair_cost: Option<f64>,

        #[arg(long, help = "轉手價")]
        sold_price: Option<f64>,

        #[arg(long, help = "幾時賣，例如2025-1-6")]
        sold_date: Option<String>,
    },
    Show,
    Delete {
        #[arg(short, long, help = "要 delete 咩？（或者唔指定名稱開啓交互模式）")]
        name: Option<String>,
    },
}
