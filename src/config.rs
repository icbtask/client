use serde::Deserialize;

fn default_url() -> String {
    String::from("https://api.icbtask.com")
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_url")]
    pub base_url: String,
    pub api_key: String,
}
