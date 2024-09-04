use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

use crate::http;

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

    pub async fn paper_list(&self, params:Option<HashMap<String, String>>) -> Result<serde_json::Value, Error> {
        // params: A dictionary of parameters to filter the papers. List of keys in the dictionary:
        //      q: Filter papers by querying the paper title and abstract.
        //      arxiv_id: Filter papers by arxiv id.
        //      title: Filter papers by part of the title.
        //      abstract: Filter papers by part of the abstract.
        //      ordering: Which field to use when ordering the results.
        //      page: Desired page.
        //      items_per_page: Desired number of items per page.
        let response = self.http.get("/papers/", None, params).await?;
        let paper = response;
        Ok(paper)
    }

    pub async fn paper_get(&self, paper_id: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/papers/{}/", paper_id), None, None).await?;
        let paper = response;
        Ok(paper)
    }

    pub async fn repository_get(&self, owner: &str, name: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/repositories/{}/{}", owner, name), None, None).await?;
        let repository = response;
        Ok(repository)
    }

    pub async fn area_get(&self, area_id: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/areas/{}", area_id), None, None).await?;
        let repository = response;
        Ok(repository)
    }  

    pub async fn dataset_get(&self, dataset_id: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/datasets/{}", dataset_id), None, None).await?;
        let repository = response;
        Ok(repository)
    }  

    pub async fn conferences_get(&self, conference_id: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/conferences/{}", conference_id), None, None).await?;
        let repository = response;
        Ok(repository)
    } 

    pub async fn authors_get(&self, author_id: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/authors/{}", author_id), None, None).await?;
        let repository = response;
        Ok(repository)
    } 

    pub async fn tasks_get(&self, task_id: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/tasks/{}", task_id), None, None).await?;
        let repository = response;
        Ok(repository)
    } 

    pub async fn evaluations_get(&self, evaluation_id: &str) -> Result<serde_json::Value, Error> {
        let response = self.http.get(&format!("/evaluations/{}", evaluation_id), None, None).await?;
        let repository = response;
        Ok(repository)
    } 

}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_paper_list() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        let mut params = HashMap::new();
        params.insert("q".to_string(), "robotics".to_string());

        match client.paper_list(Some(params)).await {
            Ok(paper) => println!("Paper List: {:?}", paper),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_paper_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.paper_get("0-step-capturability-motion-decomposition-and").await {
            Ok(paper) => println!("Paper: {:?}", paper),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_repository_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.repository_get("jishengpeng", "").await {
            Ok(repo) => println!("Repository: {:?}", repo),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_area_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.area_get("robots").await {
            Ok(repo) => println!("Area: {:?}", repo),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_dataset_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.dataset_get("17-places").await {
            Ok(repo) => println!("Dataset: {:?}", repo),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_conferences_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.conferences_get("10th-international-advanced-computing").await {
            Ok(repo) => println!("Conference: {:?}", repo),
            Err(e) => eprintln!("Error: {}", e),
        };
    }
    
    #[tokio::test]
    async fn test_authors_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.authors_get("1-beth-clark").await {
            Ok(repo) => println!("Author: {:?}", repo),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_tasks_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.tasks_get("2d-pose-estimation").await {
            Ok(repo) => println!("Task: {:?}", repo),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    #[tokio::test]
    async fn test_evaluations_get() {
        let token: Option<String> = Some("".to_string());
        let client: PapersWithCodeClient = PapersWithCodeClient::new(token, None);
        match client.evaluations_get("face-sketch-synthesis-on-cuhk").await {
            Ok(repo) => println!("Task: {:?}", repo),
            Err(e) => eprintln!("Error: {}", e),
        };
    }

}