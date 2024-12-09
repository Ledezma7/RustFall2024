use crate::config::Config;
use crate::monitor::WebsiteStatus;
use chrono::Utc;
use std::time::{Instant};
use ureq;

pub fn check_website(url: &str, config: &Config) -> WebsiteStatus {
    let mut retries = 0;
    let start = Instant::now();
    let status = loop {
        if retries >= config.max_retries {
            break Err(format!("Failed after {} retries", retries));
        }

        let result = ureq::get(url).timeout(config.timeout).call();

        match result {
            Ok(response) => break Ok(response.status()),
            Err(e) => {
                retries += 1;
                if retries >= config.max_retries {
                    break Err(e.to_string());
                }
            }
        }
    };

    let response_time = start.elapsed();

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time,
        timestamp: Utc::now(),
    }
}