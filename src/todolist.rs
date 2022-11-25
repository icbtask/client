#[path = "config.rs"]
mod config;

use std::process::exit;
use serde::Deserialize;
use reqwest::header::HeaderMap;
use reqwest::Response;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Todolist{
    pub name: String,
    pub todolist_id: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    description: String
}


async fn display_response_error(response: Response)  -> Result<(), reqwest::Error>{
    println!("{}", response.json::<ErrorResponse>().await?.description);
    exit(1);
}

pub async fn get_todolists() -> Result<Vec<Todolist>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = format!(
        "{}/todolists",
        envy::from_env::<config::Config>().unwrap().base_url
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-KEY",
        envy::from_env::<config::Config>()
            .unwrap()
            .api_key
            .parse()
            .unwrap(),
    );

    let todolists :Vec<Todolist>= client.get(url).headers(headers).send().await?.json().await?;

    Ok(todolists)
}

pub async fn create_todolist(todolist_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/todolist",
        envy::from_env::<config::Config>().expect("`BASE_URL` is required").base_url
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-KEY",
        envy::from_env::<config::Config>()
            .unwrap()
            .api_key
            .parse()
            .unwrap(),
    );
    let mut json = HashMap::new();
    json.insert("name", todolist_name);
    let response = client.post(url).headers(headers).json(&json).send().await?;

    if ! response.status().is_success() {
        display_response_error(response).await?;
    }
    Ok(())
}

pub async fn delete_todolist(todolist_id : &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/todolist/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        todolist_id
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-KEY",
        envy::from_env::<config::Config>()
            .unwrap()
            .api_key
            .parse()
            .unwrap(),
    );

    let response = client.delete(url).headers(headers).send().await?;

    if !response.status().is_success() {
        display_response_error(response).await?;
    }

    Ok(())
}
