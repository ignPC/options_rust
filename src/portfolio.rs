use crate::buyable::Buyable;
use crate::stock::Stocks;
use crate::option::Options;
use crate::stock_market;

#[derive(Clone)]
pub enum BuyableItem {
    Stock(Stocks),
    Option(Options),
}

pub struct Portfolio {
    pub initial_value: f64,
    pub current_value: f64,
    pub items: Vec<BuyableItem>,
}

impl Portfolio {
    pub fn new(initial_value: f64) -> Self {
        Portfolio {
            initial_value,
            current_value: initial_value,
            items: Vec::new(),
        }
    }

    pub fn buy(&mut self, item: BuyableItem, amount: i32) {
        for _ in 0..amount {
            let price = match &item {
                BuyableItem::Stock(stock) => {
                    stock.get_price()
                },
                BuyableItem::Option(option) => option.get_price(),
            };

            self.initial_value += price;
            self.current_value += price;

            self.items.push(item.clone());
        }
    }

    pub fn sell(&mut self, item: &BuyableItem, amount: i32) {
        let mut remaining_amount = amount;
        let mut remove_indices = Vec::new();

        for (i, portfolio_item) in self.items.iter().enumerate() {
            if portfolio_item_matches(portfolio_item, item) {
                if remaining_amount >= 1 {
                    let price = match item {
                        BuyableItem::Stock(stock) => {
                            stock.get_price()
                        },
                        BuyableItem::Option(option) => option.get_price(),
                    };

                    self.initial_value -= price;
                    self.current_value -= price;
                    remaining_amount -= 1;
                    remove_indices.push(i);
                } else {
                    break;
                }
            }
        }

        for &index in remove_indices.iter().rev() {
            self.items.remove(index);
        }
    }

    pub fn buy_value(&mut self, item: BuyableItem, value: f64) {
        let price = match item.clone() {
            BuyableItem::Stock(stock) => stock.get_price(),
            BuyableItem::Option(option) => option.get_price(),
        };

        let amount: i32 = if price > 0.001 {
            (value / (price)) as i32
        } else {
           100_000
        };

        self.buy(item, amount);
    }


    pub fn update_portfolio(&mut self, buyable: BuyableItem) {
        let price = match &buyable {
            BuyableItem::Stock(stock) => stock.get_price(),
            BuyableItem::Option(option) => option.get_price(),
        };

        for item in &mut self.items {
            match item {
                BuyableItem::Stock(stock) => {
                    if let BuyableItem::Stock(buyable_stock) = &buyable {
                        if stock.matches(buyable_stock) {
                            stock.set_price(price);
                        }
                    }
                }
                BuyableItem::Option(option) => {
                    if let BuyableItem::Option(buyable_option) = &buyable {
                        if option.matches(buyable_option) {
                            option.set_price(price);
                        }
                    }
                }
            }
        }

        self.current_value = 0.0;

        for item in &mut self.items {
            if let BuyableItem::Stock(stock) = item {
                let price = stock.get_price();
                self.current_value += price;
            }
            else if let BuyableItem::Option(option) = item {
                let price = option.get_price();
                self.current_value += price;
            }
        }
    }

    pub fn print_portfolio_information(&self) {
        println!("\n---PORTFOLIO---");
        println!("INVESTED:\t\t{}", self.initial_value);
        println!("TOTAL ITEMS:\t{}", self.items.len());
        println!("TOTAL NOW:\t\t{}", self.current_value);
        println!("CHANGE%:\t\t{:.2}%", calculate_change_percentage(self.initial_value, self.current_value));
    }
}

fn calculate_change_percentage(initial_value: f64, current_value: f64) -> f64 {
    ((current_value - initial_value) / initial_value) * 100.0
}

fn portfolio_item_matches(portfolio_item: &BuyableItem, item: &BuyableItem) -> bool {
    match (portfolio_item, item) {
        (BuyableItem::Stock(portfolio_stock), BuyableItem::Stock(stock)) => {
            portfolio_stock.matches(stock)
        }
        (BuyableItem::Option(portfolio_option), BuyableItem::Option(option)) => {
            portfolio_option.matches(option)
        }
        _ => false,
    }
}

