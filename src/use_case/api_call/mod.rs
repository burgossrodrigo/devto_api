use crate::domain::entities::post::Post;

use crate::domain::use_case::api_call::{GetApiCallInputType, GetApiCallOutputType, GetApiCallUseCaseType};
use reqwest::{Client, Error};
use std::env;

pub struct GetApiCallUseCase;

impl GetApiCallUseCaseType for GetApiCallUseCase {
    type Input = GetApiCallInputType;
    type Output = GetApiCallOutputType;

    async fn execute(&self, _input: Self::Input) -> Self::Output {
        // Use a blocking runtime to call the async function
        fetch_posts()
            .await
            .unwrap_or_else(|_| GetApiCallOutputType { articles: vec![] })
    }
}

// Async function to fetch posts from the API
use anyhow::{Result, anyhow}; // no lugar de reqwest::Error

async fn fetch_posts() -> Result<GetApiCallOutputType> {
    let url = "https://dev.to/api/articles/me/all";
    let api_key = env::var("API_KEY").expect("API key not set in environment");
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("api-key", api_key)
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.forem.api-v1+json")
        .header("User-Agent", "reqwest")
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(anyhow!("Request failed: {}, body: {}", status, body));
    }

    let body = response.text().await?;
    println!("Raw JSON Body: {}", body);



    let articles: Vec<Post> = serde_json::from_str(&body)?;

    Ok(GetApiCallOutputType { articles })
}

