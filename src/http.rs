use serde::Deserialize;
use std::collections::HashMap;

use crate::config;

#[derive(Deserialize, Debug)]
struct CoinMarketCapResponse {
    data: HashMap<String, AssetData>,
}

#[derive(Deserialize, Debug)]
struct AssetData {
    id: i32,
    name: String,
    symbol: String,
    quote: HashMap<String, FiatData>
}

#[derive(Deserialize, Debug)]
struct FiatData {
   price: f32,
}


#[tokio::main]
pub async fn fetch_current_price(coin: &str) -> Result<f32, reqwest::Error> {
    let api_key = config::load_env_vars("API_KEY");

    let client = reqwest::Client::new();

    let request_url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={coin}&convert={fiat}",
        coin=coin,
        fiat="GBP");

    let res = client
        .get(request_url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?;
    
    
    let data: CoinMarketCapResponse = serde_json::from_value(res.json().await?).unwrap();
    
    let mut current_value: f32 = 0.0;
    for (_key, value) in data.data {
        for (_k, v) in value.quote {
            current_value = v.price;
        }
    }

    Ok(current_value)
}