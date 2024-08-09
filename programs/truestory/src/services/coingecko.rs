use reqwest::blocking::Client;

pub fn get_sol_price() -> Result<f64, reqwest::Error> {
    let client = Client::new();
    let res = client.get("https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd")
                    .send()?
                    .json::<serde_json::Value>()?;
    Ok(res["solana"]["usd"].as_f64().unwrap())
}
