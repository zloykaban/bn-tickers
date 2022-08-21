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
            "https://api.binance.com/api/v3/avgPrice?symbol={}USDT", ticker
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CurrencyPrice>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    let mut ticker: String = "BTC".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a ticker, it is defaulted to BTC.");
    } else {
        ticker = args[1].clone();
    }

    let res = CurrencyPrice::get(&ticker).await?;
    println!("{}`s cost is {} USDT", ticker, res.price);

    Ok(())

}

