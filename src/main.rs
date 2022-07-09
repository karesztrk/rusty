use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    coins: Vec<Coin>,
}

#[derive(Deserialize, Debug)]
struct Coin {
    id: String,
    name: String,
    icon: String,
    symbol: String,
    price: f32,
    priceBtc: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let http_response = reqwest::get("https://api.coinstats.app/public/v1/coins?skip=0&limit=10").await?;
    let response = http_response.json::<Response>().await?;
    println!("{:#?}", response.coins);
    Ok(())
}