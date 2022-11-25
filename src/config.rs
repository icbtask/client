use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
}
