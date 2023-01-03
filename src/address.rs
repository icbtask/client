use crate::config;
use crate::todolist;
use crate::utils;

use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AllowedAddress {
    pub username: String,
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub address: String,
    pub allowed_addresses: Vec<AllowedAddress>,
    pub todolist: Option<todolist::Todolist>,
}

pub async fn get_addresses() -> Result<Vec<Address>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = format!(
        "{}/addresses",
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

    let addresses: Vec<Address> = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(addresses)
}

pub async fn create_address() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/address",
        envy::from_env::<config::Config>()
            .expect("`BASE_URL` is required")
            .base_url
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
    let response = client.post(url).headers(headers).send().await?;

    if !response.status().is_success() {
        utils::display_response_error(response).await?;
    }
    Ok(())
}

pub async fn delete_address(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/address/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        address
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
        utils::display_response_error(response).await?;
    }

    Ok(())
}

pub async fn attach_address(
    address: &str,
    todolist_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/address/todolist/{}/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        address,
        todolist_id,
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

    let response = client.post(url).headers(headers).send().await?;

    if !response.status().is_success() {
        utils::display_response_error(response).await?;
    }

    Ok(())
}

pub async fn detach_address(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/address/todolist/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        address,
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
        utils::display_response_error(response).await?;
    }

    Ok(())
}

pub async fn allow_address(
    address: &str,
    remote_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/address/access/{}/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        address,
        remote_address
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

    let response = client.post(url).headers(headers).send().await?;

    if !response.status().is_success() {
        utils::display_response_error(response).await?;
    }

    Ok(())
}

pub async fn revoke_address(
    address: &str,
    remote_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/address/access/{}/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        address,
        remote_address
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
        utils::display_response_error(response).await?;
    }

    Ok(())
}
