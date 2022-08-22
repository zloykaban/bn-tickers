use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct CurrencyPrice {
    mins: u32,
    price: String,
}

impl CurrencyPrice {
    async fn get(ticker: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.binance.com/api/v3/avgPrice?symbol={}USDT",
            ticker
        );

        let url = Url::parse(url.as_str())?;

        let res = reqwest::get(url).await?.json::<CurrencyPrice>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let ticker = match env::args().nth(1) {
        Some(ticker) => ticker.to_uppercase(),
        None => String::from("BTC"),
    };

    let res = CurrencyPrice::get(&ticker).await?;
    println!("{}`s cost is {} USDT", ticker, res.price);

    Ok(())
}
