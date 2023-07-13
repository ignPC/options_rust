use crate::{BuyableItem, Stocks};
use crate::option::Options;

pub trait Buyable {
    fn get_price(&self) -> f64;
    fn set_price(&mut self, price: f64);
    fn matches(&self, other: &Self) -> bool;
}

impl Buyable for Options {
    fn get_price(&self) -> f64 {
        self.price
    }

    fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    fn matches(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Buyable for Stocks {
    fn get_price(&self) -> f64 {
        self.price
    }

    fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    fn matches(&self, other: &Self) -> bool {
        self.name== other.name
    }
}


