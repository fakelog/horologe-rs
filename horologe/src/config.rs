use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub worker_count: usize,
    pub poll_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .context("Environment variable DATABASE_URL not found")?,
            worker_count: env::var("WORKER_COUNT")
                .map(|s| s.parse().unwrap_or(4))
                .unwrap_or(4),
            poll_interval_secs: env::var("POLL_INTERVAL_SECS")
                .map(|s| s.parse().unwrap_or(10))
                .unwrap_or(10),
        })
    }
}
