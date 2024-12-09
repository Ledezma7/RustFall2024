use crate::config::Config;
use crate::worker::check_website;
use std::sync::mpsc::{self, Receiver, Sender};
use chrono::{DateTime, Utc};
use std::thread;
use std::time::{Duration};


pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

pub fn start_monitoring(websites: &[String], config: &Config) -> Vec<WebsiteStatus> {
    let (tx, rx): (Sender<WebsiteStatus>, Receiver<WebsiteStatus>) = mpsc::channel();

    let num_workers = config.num_workers.min(websites.len());
    let mut handles = vec![];

    for i in 0..num_workers {
        let tx_clone = tx.clone();
        let websites_chunk: Vec<String> = websites.iter().enumerate().filter(|(index, _)| index % num_workers == i)
            .map(|(_, url)| url.clone()).collect();
        
       
        let config_clone = Config {
            num_workers: config.num_workers,
            timeout: config.timeout,
            max_retries: config.max_retries,
        };

        let handle = thread::spawn(move || {
            for url in websites_chunk {
                let status = check_website(&url, &config_clone);
                tx_clone.send(status).expect("Failed to send status");
            }
        });

        handles.push(handle);
    }

    drop(tx); //close sender to allow threads to complete
    for handle in handles {
        handle.join().expect("Worker thread failed due to error");
    }

    rx.iter().collect()
}