use crate::buyable::Buyable;
use crate::option::Options;
use crate::stock::Stocks;
use crate::stock_market::market_open;
use crate::portfolio::{BuyableItem, Portfolio};

mod stock;
mod stock_market;
mod portfolio;
mod graph;
mod option;
mod buyable;

fn main() {
    println!("\nSETUP");
    let mut stock = Stocks::generate_new();     // Setup environment
    let mut portfolio = Portfolio::new(0.0);
    let mut option = option::generate_random_option(&stock);

    option.print_option_information();

    println!("\nMARKET");
    portfolio.buy_value(BuyableItem::Option(option.clone()), 10_000.0);

    let mut days = 0;

    while days < 100 {
        days += 1;
        market_open(&mut stock);
        option.update_option_price(stock.clone(), 1 as f64);

        stock.price_history.push(stock.price.clone());
        option.price_history.push(option.price.clone());

        portfolio.update_portfolio(BuyableItem::Stock(stock.clone()));
        portfolio.update_portfolio(BuyableItem::Option(option.clone()));
    }

    println!("\nAFTERMARKET");

    graph::plot_price_history(stock.price_history.clone(), "l_Stock_Graph");
    graph::plot_price_history(option.price_history.clone(), "l_Option_Graph");

    option.print_option_information();
    stock.print_stock_information();
    portfolio.print_portfolio_information();
}




