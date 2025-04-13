use spm_swift_package::infrastructure::network_client::NetworkClient;
use serde::Deserialize;
use httpmock::prelude::*;
use tokio;

#[derive(Debug, Deserialize)]
struct SwiftLintResponse {
    tag_name: String,
}

#[tokio::test]
async fn test_get_swiftlint_tag() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method(GET).path("/json");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"tag_name":"v0.52.4"}"#);
    });

    let network = NetworkClient::new();
    let url = &format!("{}/json", &server.base_url());
    let user_agent = "test-agent";

    let response: SwiftLintResponse = network.get_json(url, user_agent).await.unwrap();

    assert_eq!(response.tag_name, "v0.52.4");
}
