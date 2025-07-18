use crate::models::MarketData;
use reqwest::Client;
use sled;

pub async fn fetch_market_data(symbol_list: &str) -> sled::Result<Vec<MarketData>> {
    let url = "https://api.coingecko.com/api/v3/coins/markets";
    let client = Client::new();

    let response = client
        .get(url)
        .query(&[("vs_currency", "usd"), ("symbols", symbol_list)])
        .header("accept", "application/json")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                              AppleWebKit/537.36 (KHTML, like Gecko) \
                              Chrome/115.0.0.0 Safari/537.36",
        )
        .send()
        .await
        .map_err(|e| sled::Error::ReportableBug(e.to_string()))?
        .json::<Vec<MarketData>>()
        .await
        .map_err(|e| sled::Error::ReportableBug(e.to_string()))?;

    if response.len() == 0 {
        return Err(sled::Error::ReportableBug(
            "Market data is empty".to_string(),
        ));
    }

    Ok(response)
}
