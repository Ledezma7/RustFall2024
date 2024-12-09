mod config;
mod monitor;
mod worker;

use config::Config;
use monitor::{start_monitoring};
use std::time::Duration;

fn main() {
    let config = Config {
        num_workers: 10,
        timeout: Duration::from_secs(5),
        max_retries: 3,
    };

    let websites = vec![
        "https://www.google.com".to_string(),
        "https://www.github.com".to_string(),
        "https://doesnotexist.example".to_string(),
    ];

    println!("Starting monitoring for {} websites...", websites.len());
    let results = start_monitoring(&websites, &config);

    println!("Monitoring Results: ");
    for status in results {
        println!("URL: {}, \nStatus: {:?}, \nResponse Time: {:?}, \nTimestamp: {}",
                 status.url, status.status, status.response_time, status.timestamp);
    }
}