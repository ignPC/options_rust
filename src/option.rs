use rand::Rng;
use rand_distr::Normal;
use statrs::distribution::ContinuousCDF;
use crate::{Buyable, Stocks};

#[derive(Clone)]
pub struct Options {
    pub(crate) name: String,
    pub(crate) option_type: OptionType,
    pub(crate) spot_price: f64,
    pub(crate) strike_price: f64,
    pub(crate) time_to_expiration: f64,
    pub(crate) risk_free_rate: f64,
    pub(crate) volatility: f64,
    pub(crate) price: f64,
    pub(crate) price_history: Vec<f64>,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum OptionType {
    Call,
    Put,
}

impl Options {
    pub fn new(
        name: String,
        option_type: OptionType,
        spot_price: f64,
        strike_price: f64,
        time_to_expiration: f64,
        risk_free_rate: f64,
        volatility: f64,
    ) -> Self {
        let price = Self::calculate_option_price(
            spot_price.clone(),
            strike_price.clone(),
            time_to_expiration.clone(),
            risk_free_rate.clone(),
            volatility.clone(),
            option_type.clone(),
        );

        Options {
            name,
            option_type,
            spot_price,
            strike_price,
            time_to_expiration,
            risk_free_rate,
            volatility,
            price,
            price_history: Vec::new()
        }
    }

    pub fn calculate_option_price(
        spot_price: f64,
        strike_price: f64,
        time_to_expiration: f64,
        risk_free_rate: f64,
        volatility: f64,
        option_type: OptionType,
    ) -> f64 {
        let d1 = (spot_price.ln()
            - strike_price.ln()
            + (risk_free_rate + 0.5 * volatility * volatility) * time_to_expiration)
            / (volatility * time_to_expiration.sqrt());
        let d2 = d1 - volatility * time_to_expiration.sqrt();

        let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
        let cdf_d1 = normal.cdf(d1);
        let cdf_d2 = normal.cdf(d2);

        let call_price = spot_price * cdf_d1
            - strike_price * (-risk_free_rate * time_to_expiration).exp()
            * cdf_d2;
        let put_price = call_price - spot_price
            + strike_price * (-risk_free_rate * time_to_expiration).exp();

        match option_type {
            OptionType::Call => call_price,
            OptionType::Put => put_price,
        }
    }

    pub fn update_option_price(&mut self, stock: Stocks, time_expired: f64) {
        self.volatility = calculate_volatility(stock.price_history.clone());
        self.spot_price = stock.price.clone();
        self.time_to_expiration -= time_expired / 365.0;

        let d1 = (self.spot_price.ln()
            - self.strike_price.ln()
            + (self.risk_free_rate + 0.5 * self.volatility * self.volatility)
            * self.time_to_expiration)
            / (self.volatility * self.time_to_expiration.sqrt());
        let d2 = d1 - self.volatility * self.time_to_expiration.sqrt();

        let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
        let cdf_d1 = normal.cdf(d1);
        let cdf_d2 = normal.cdf(d2);

        let call_price = self.spot_price * cdf_d1
            - self.strike_price * (-self.risk_free_rate * self.time_to_expiration).exp()
            * cdf_d2;
        let put_price = call_price - self.spot_price
            + self.strike_price * (-self.risk_free_rate * self.time_to_expiration).exp();

        self.price = match self.option_type {
            OptionType::Call => call_price,
            OptionType::Put => put_price,
        };
    }

    pub fn print_option_information(&self) {
        println!("---OPTION---");
        println!("Option Name: {}", self.name);
        println!("Option Type: {:?}", self.option_type);
        println!("Spot Price: {}", self.spot_price);
        println!("Strike Price: {}", self.strike_price);
        println!("Time to Expiration: {}", self.time_to_expiration);
        println!("Risk-Free Rate: {}", self.risk_free_rate);
        println!("Volatility: {}", self.volatility);
        println!("Price: {}", self.price);
    }
}

pub(crate) fn generate_random_option(stock: &Stocks) -> Options {
    let name = stock.name.clone();
    let option_type = if rand::random::<bool>() {
        OptionType::Call
    } else {
        OptionType::Put
    };

    // Generate random values for option parameters based on the stock's characteristics
    let spot_price = stock.price.clone();
    let strike_price = generate_random_strike_price(stock.price);
    let time_to_expiration = generate_random_time_to_expiration();
    let volatility = 0.11; // Random Volatility as stock has no history
    let risk_free_rate = 0.03315; // Current RFR in Belgium

    Options::new(
        name,
        option_type,
        spot_price,
        strike_price,
        time_to_expiration,
        risk_free_rate,
        volatility,
    )
}

fn generate_random_strike_price(current_price: f64) -> f64 {
    // Generate a random strike price based on the current price of the stock
    // Example: Generating a strike price within a range of +/- 10% of the current price
    let min_strike_price = current_price * 0.5;
    let max_strike_price = current_price * 2.0;

    let strike_price = rand::thread_rng().gen_range(min_strike_price..=max_strike_price);

    strike_price
}

fn generate_random_time_to_expiration() -> f64 {
    // Generate a random time to expiration
    // Example: Generating a time to expiration between 0.25 and 2 years
    let min_time_to_expiration = 0.25;
    let max_time_to_expiration = 2.0;

    let time_to_expiration = rand::thread_rng().gen_range(min_time_to_expiration..=max_time_to_expiration);

    time_to_expiration
}

fn calculate_volatility(price_history: Vec<f64>) -> f64 {
    if price_history.len() < 2 {
        return 0.0;
    }

    let returns: Vec<f64> = price_history
        .windows(2)
        .map(|window| {
            let prev_price = window[0];
            let current_price = window[1];
            (current_price - prev_price) / prev_price
        })
        .collect();

    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let squared_diff_sum = returns
        .iter()
        .map(|return_val| (*return_val - mean).powi(2))
        .sum::<f64>();
    let volatility = squared_diff_sum.sqrt() * (returns.len() as f64 / (returns.len() - 1) as f64).sqrt();

    volatility
}

