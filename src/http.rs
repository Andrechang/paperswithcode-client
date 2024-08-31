use reqwest::{Client, Method, Error};
// use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AuthorizationMethod {
    Basic,
    Token,
    Jwt,
}

#[derive(Debug)]
pub struct HttpClient {
    url: String,
    token: Option<String>,
    authorization_method: AuthorizationMethod,
    client: Client,
}

impl HttpClient {
    pub fn new(url: String, token: Option<String>, authorization_method: AuthorizationMethod) -> Self {
        Self {
            url,
            token,
            authorization_method,
            client: Client::new(),
        }
    }

    async fn request(&self, method: Method, endpoint: &str, headers: Option<HashMap<String, String>>, params: Option<HashMap<String, String>>, body: Option<serde_json::Value>) -> Result<serde_json::Value, Error> {
        let mut req = self.client.request(method, &format!("{}{}", self.url, endpoint));

        if let Some(ref token) = self.token {
            if matches!(self.authorization_method, AuthorizationMethod::Jwt) {
                req = req.bearer_auth(token);
            }
        }

        if let Some(h) = headers {
            for (key, value) in h {
                req = req.header(&key, &value);
            }
        }

        if let Some(p) = params {
            req = req.query(&p);
        }

        if let Some(b) = body {
            req = req.json(&b);
        }

        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn get(&self, endpoint: &str, headers: Option<HashMap<String, String>>, params: Option<HashMap<String, String>>) -> Result<serde_json::Value, Error> {
        self.request(Method::GET, endpoint, headers, params, None).await
    }
}