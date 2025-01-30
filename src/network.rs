use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct SwiftLintResponse {
    tag_name: String,
}

pub struct Network {
    client: reqwest::Client,
}

impl Network {
    pub fn new() -> Self {
        Network {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_swiftlint_tag(&self) -> Result<String, Box<dyn Error>> {
        const URL: &str = "https://api.github.com/repos/realm/SwiftLint/releases/latest";
        const USER_AGENT: &str = "spmswiftpackage";

        let response = self
            .client
            .get(URL)
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send()
            .await?;

        if response.status().is_success() {
            let result: SwiftLintResponse = response.json().await?;
            Ok(result.tag_name)
        } else {
            Err(format!("Failed to fetch SwiftLint tag: HTTP status {}", response.status()).into())
        }
    }
}