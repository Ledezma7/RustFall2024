use std::time::Duration;

pub struct Config {
    pub num_workers: usize,
    pub timeout: Duration,
    pub max_retries: usize,
}