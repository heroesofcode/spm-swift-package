use crate::infrastructure::network_client::NetworkClient;
use crate::data::swiftlint_dto::SwiftLintDto;
use async_trait::async_trait;

#[async_trait]
pub trait SwiftLintRepository {
    async fn get_latest_tag(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
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
        #[cfg(debug_assertions)]
        let url = "http://127.0.0.1:3000/get_tag";

        #[cfg(not(debug_assertions))]
        let url = "https://api.github.com/repos/realm/SwiftLint/releases/latest";

        let user_agent = "spmswiftpackage";

        let response: SwiftLintDto = self.client.get_json(url, user_agent).await?;
        Ok(response.tag_name)
    }
}