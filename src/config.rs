use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub interval_seconds: u64,
    pub pump_mc_url: String, // https://frontend-api.pump.fun/coins?sort=market_cap&order=DESC&includeNsfw=false&offset=1&limit=2
    pub pump_mc_url_index: u64, //一次50个，这里不要太多5页就不少了
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse config file")
    }
}
