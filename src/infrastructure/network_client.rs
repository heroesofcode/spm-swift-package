use reqwest;

pub struct NetworkClient {
    client: reqwest::Client,
}

impl NetworkClient {
    pub fn new() -> Self {
        NetworkClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_json<T: for<'de> serde::Deserialize<'de>>(
        &self,
        url: &str,
        user_agent: &str,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get(url)
            .header(reqwest::header::USER_AGENT, user_agent)
            .send()
            .await?;

        if response.status().is_success() {
            let parsed = response.json::<T>().await?;
            Ok(parsed)
        } else {
            Err(format!("Request failed: {}", response.status()).into())
        }
    }
}
