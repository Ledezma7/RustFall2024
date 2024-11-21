use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;


#[derive(Debug)]
struct Bitcoin;

#[derive(Debug)]
struct Ethereum;

#[derive(Debug)]
struct SP500;

trait Pricing {
    fn fetch_price() -> Result<f64, String>;
}

// Implementation of pricing trait
impl Pricing for Bitcoin {
    fn fetch_price() -> Result<f64, String> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd").call();
        match response {
            Ok(resp) => {
                let json: Value = resp.into_json().map_err(|_| "Failed to parse Bitcoin JSON".to_string())?;
                json["bitcoin"]["usd"].as_f64().ok_or_else(|| "Failed to extract Bitcoin price".to_string())
            }
            Err(err) => Err(format!("Failed to fetch Bitcoin price: {}", err)),
        }
    }
}

impl Pricing for Ethereum {
    fn fetch_price() -> Result<f64, String> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd").call();
        match response {
            Ok(resp) => {
                let json: Value = resp.into_json().map_err(|_| "Failed to parse Ethereum JSON".to_string())?;
                json["ethereum"]["usd"].as_f64().ok_or_else(|| "Failed to extract Ethereum price".to_string())
            }
            Err(err) => Err(format!("Failed to fetch Ethereum price: {}", err)),
        }
    }
}

impl Pricing for SP500 {
    fn fetch_price() -> Result<f64, String> {
        let response = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d").call();
        match response {
            Ok(resp) => {
                let json: Value = resp.into_json().map_err(|_| "Failed to parse S&P 500 JSON".to_string())?;
                json["chart"]["result"][0]["indicators"]["quote"][0]["close"].as_array().and_then(|arr| arr.last()).and_then(|val| val.as_f64()).ok_or_else(|| "Failed to extract SP500 price".to_string())
            }
            Err(err) => Err(format!("Failed to fetch SP500 price: {}", err)),
        }
    }
}

// Function to save the data to a file
fn save_to_file(file_name: &str, data: &str) {
    let result = OpenOptions::new().append(true).create(true).open(file_name);
    match result {
        Ok(mut file) => {
            if let Err(err) = writeln!(file, "{}", data) {
                eprintln!("Failed to write to file {}: {}", file_name, err);
            }
        }
        Err(err) => eprintln!("Failed to open file {}: {}", file_name, err),
    }
}

fn main() {
    loop {
        // Fetch and save Bitcoin price
        if let Ok(price) = Bitcoin::fetch_price() {
            save_to_file("bitcoin.txt", &format!("Bitcoin price: ${:.2}", price));
        }

        // Fetch and save Etherum price
        if let Ok(price) = Ethereum::fetch_price() {
            save_to_file("ethereum.txt", &format!("Ethereum price: ${:.2}", price));
        }

        // Fetch and save SP500 price
        if let Ok(price) = SP500::fetch_price() {
            save_to_file("sp500.txt", &format!("SP500 price: ${:.2}", price));
        }

        // Sleep for 10 seconds
        thread::sleep(Duration::from_secs(10));
    }
}
