use serde::Deserialize;
use std::process::exit;

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

pub async fn display_response_error(response: reqwest::Response) -> Result<(), reqwest::Error> {
    println!("{}", response.json::<ErrorResponse>().await?.error);
    exit(1);
}
