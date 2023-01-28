use serde::Deserialize;

fn default_url() -> String {
    String::from("https://api.icbtask.com")
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base_url: String,
    #[serde(default = "default_url")]
    pub api_key: String,
}
