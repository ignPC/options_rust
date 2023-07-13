use rand::Rng;


pub fn market_open(stock: &mut crate::Stocks) {
    let mut rng = rand::thread_rng();
    let total_outstanding_shares = stock.total_stocks as f64;

    let mut change_percent: f64;

    // Generate change_percent with a 10% chance for -5 to 5.5 range
    if rng.gen_bool(0.1) {
        change_percent = rng.gen_range(-5.0..=5.5);
    } else {
        // Generate change_percent with a 90% chance for -1 to 1.2 range
        change_percent = rng.gen_range(-1.0..=1.2);
    }

    let price = stock.price * (1.0 + (change_percent / 100.0));

    stock.update_market_cap(price * total_outstanding_shares)
}


pub fn sell(stock: &mut crate::Stocks, shares_sold: u32) {
    let initial_stock_price = stock.price;
    let total_outstanding_shares = stock.total_stocks as f64;

    let stock_price_drop = (f64::from(shares_sold) / total_outstanding_shares) * initial_stock_price;
    let stock_price = stock.price - stock_price_drop;

    stock.update_market_cap(stock_price * total_outstanding_shares)
}

pub fn buy(stock: &mut crate::Stocks, shares_bought: u32) {
    let initial_stock_price = stock.price;
    let total_outstanding_shares = stock.total_stocks as f64;

    let stock_price_increase = (f64::from(shares_bought) / total_outstanding_shares) * initial_stock_price;
    let stock_price = stock.price + stock_price_increase;

    stock.update_market_cap(stock_price * total_outstanding_shares);
}
