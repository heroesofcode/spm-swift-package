use crate::infrastructure::network_client::NetworkClient;
use serde::Deserialize;
use async_trait::async_trait;

#[async_trait]
pub trait SwiftLintRepository {
    async fn get_latest_tag(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Deserialize)]
struct SwiftLintResponse {
    tag_name: String,
}

pub struct SwiftLintRepositoryImpl {
    client: NetworkClient,
}

impl SwiftLintRepositoryImpl {
    pub fn new() -> Self {
        Self {
            client: NetworkClient::new(),
        }
    }
}

#[async_trait]
impl SwiftLintRepository for SwiftLintRepositoryImpl {
    async fn get_latest_tag(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = "https://api.github.com/repos/realm/SwiftLint/releases/latest";
        let user_agent = "spmswiftpackage";

        let response: SwiftLintResponse = self.client.get_json(url, user_agent).await?;
        Ok(response.tag_name)
    }
}