use rand::Rng;
use crate::{Buyable, option};

#[derive(Clone)]
pub struct Stocks {
    pub(crate) name: String,
    pub(crate) market_cap: f64,
    pub(crate) total_stocks: u32,
    pub(crate) price: f64,
    pub(crate) price_history: Vec<f64>,
}

impl Stocks {
    pub fn generate_new() -> Stocks {
        let name = generate_random_name();
        let market_cap = generate_random_market_cap();
        let total_stocks = generate_random_total_stocks();
        let price = calculate_price(market_cap, total_stocks);
        let price_history = Vec::new();

        Stocks {
            name,
            market_cap,
            total_stocks,
            price,
            price_history,
        }
    }

    pub fn update_market_cap(&mut self, market_cap: f64) {
        self.market_cap = market_cap;
        self.price = calculate_price(self.market_cap, self.total_stocks);
    }

    pub fn update_total_stocks(&mut self, total_stocks: u32) {
        self.total_stocks = total_stocks;
        self.price = calculate_price(self.market_cap, self.total_stocks);
    }

    pub fn update_price(&mut self, price: f64) {
        self.price = price
    }

    pub fn print_stock_information(&self){
        println!();
        println!("---STOCK---");
        println!(
            "Stock Name: {}",
            self.name
        );
        println!(
            "Market Cap: {}",
            format_number_with_commas(round_decimals(self.market_cap, 2))
        );
        println!(
            "Total Stocks: {}",
            format_number_with_commas(self.total_stocks as f64)
        );
        println!(
            "Price per Stock: {}",
            format_number_with_commas(round_decimals(self.price, 5))
        );
    }
}

fn generate_random_name() -> String {
    // Generate a random 3 or 4 length string
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(3..=4);
    let chars: Vec<char> = (0..length)
        .map(|_| rng.gen_range(b'A'..=b'Z') as char)
        .collect();
    chars.iter().collect()
}

fn generate_random_market_cap() -> f64 {
    let mut rng = rand::thread_rng();
    let market_cap = rng.gen_range(100_000_000.0..=100_000_000_000.0);
    market_cap
}

fn generate_random_total_stocks() -> u32 {
    // Generate a realistic total number of stocks
    let mut rng = rand::thread_rng();
    rng.gen_range(1_000_000..=100_000_000)
}

fn calculate_price(market_cap: f64, total_stocks: u32) -> f64 {
    // Calculate the price per stock based on market cap and total stocks
    let price = market_cap / f64::from(total_stocks);
    price
}

fn round_decimals(start: f64, decimals: i8) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    let end = (start * factor).round() / factor;
    end
}

fn format_number_with_commas(value: f64) -> String {
    let mut value_str = value.to_string();
    let mut result = String::new();
    let mut count = 0;
    let mut fraction_part = String::new();

    if let Some(dot_idx) = value_str.find('.') {
        fraction_part = value_str.split_off(dot_idx);
        value_str.retain(|c| c != '.');
    }

    for c in value_str.chars().rev() {
        if count != 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect::<String>() + &fraction_part
}



