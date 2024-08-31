use reqwest::Error;
use serde::Deserialize;
// use std::collections::HashMap;

// mod client;
mod http;

#[derive(Debug)]
pub struct PapersWithCodeClient {
    http: http::HttpClient,
}

#[derive(Debug, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub abstract_: Option<String>,
    // Add other fields as necessary
}

impl PapersWithCodeClient {
    pub fn new(token: Option<String>, url: Option<String>) -> Self {
        let http: http::HttpClient = http::HttpClient::new(
            url.unwrap_or_else(|| "https://paperswithcode.com/api/v1".to_string()),
            token,
            http::AuthorizationMethod::Token,
        );

        PapersWithCodeClient { http }
    }

    pub async fn paper_get(&self) -> Result<serde_json::Value, Error> {
        let response = self.http.get("/papers/", None, None).await?;
        let paper = response;
        Ok(paper)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your actual token if needed
    let token: Option<String> = Some("".to_string());
    let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);

    match client.paper_get().await {
        Ok(paper) => println!("Paper: {:?}", paper),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}