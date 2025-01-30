use spm_swift_package::Network;
use tokio::test;

#[test]
async fn test_get_swiftlint_tag() {
    let network = Network::new();

    match network.get_swiftlint_tag().await {
        Ok(tag) => assert!(!tag.is_empty()),
        Err(error) => eprintln!("Error {}", error)
    }
}