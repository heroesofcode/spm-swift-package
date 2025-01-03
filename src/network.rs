use std::process::exit;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SwiftLintResponse {
    tag_name: String
}

pub struct Network;

impl Network {
    pub async fn get_swiftlint_tag() -> String {
        let url = "https://api.github.com/repos/realm/SwiftLint/releases/latest";

        let response = reqwest::Client::new()
            .get(url)
            .header(reqwest::header::USER_AGENT, "spmswiftpackage")
            .send()
            .await
            .expect("You may be offline or another error has occurred.");

        if response.status().is_success() {
            let result: SwiftLintResponse = response
                .json()
                .await
                .expect("You may be offline or another error has occurred.");

            result.tag_name
        } else {
            exit(1);
        }
    }
}