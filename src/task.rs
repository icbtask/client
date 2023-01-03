use crate::config;
use crate::todolist::Todolist;
use crate::utils;

use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Task {
    pub created_at: String,
    pub description: String,
    pub project: String,
    pub shared_from: Option<String>,
    pub shared_with: Vec<String>,
    pub status: String,
    pub todolist: Todolist,
    pub task_id: String,
    pub updated_at: String,
}

pub async fn create_task(
    todolist_id: &str,
    project: &str,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/task",
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
    let mut json = HashMap::new();
    json.insert("todolist_id", todolist_id);
    json.insert("project", project);
    json.insert("description", description);
    let response = client.post(url).headers(headers).json(&json).send().await?;

    if !response.status().is_success() {
        utils::display_response_error(response).await?;
    }
    Ok(())
}

pub async fn get_tasks(
    todolist_id: Option<&String>,
) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/tasks",
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
    match todolist_id {
        Some(id) => {
            let tasks: Vec<Task> = client
                .get(url)
                .headers(headers)
                .query(&[("todolist_id", id)])
                .send()
                .await?
                .json()
                .await?;
            Ok(tasks)
        }
        None => {
            let tasks: Vec<Task> = client
                .get(url)
                .headers(headers)
                .send()
                .await?
                .json()
                .await?;

            Ok(tasks)
        }
    }
}

pub async fn delete_task(task_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/task/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        task_id
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

pub async fn complete_task(task_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/task/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        task_id
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
    json.insert("status", "completed");
    let response = client
        .patch(url)
        .headers(headers)
        .json(&json)
        .send()
        .await?;

    if !response.status().is_success() {
        utils::display_response_error(response).await?;
    }
    Ok(())
}

pub async fn edit_task(
    task_id: &str,
    project: Option<&String>,
    description: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/task/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        task_id
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
    if let Some(updated_project) = project {
        json.insert("project", updated_project);
    }
    if let Some(updated_description) = description {
        json.insert("description", updated_description);
    }

    let response = client
        .patch(url)
        .headers(headers)
        .json(&json)
        .send()
        .await?;

    if !response.status().is_success() {
        utils::display_response_error(response).await?;
    }
    Ok(())
}

pub async fn share_task(task_id: &str, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/task/share/{}/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        task_id,
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

    let response = client.post(url).headers(headers).send().await?;

    if !response.status().is_success() {
        utils::display_response_error(response).await?;
    }
    Ok(())
}

pub async fn unshare_task(task_id: &str, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/task/share/{}/{}",
        envy::from_env::<config::Config>().unwrap().base_url,
        task_id,
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
