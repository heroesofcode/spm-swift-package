use spm_swift_package::Network;
use tokio::test;

#[test]
async fn test_get_swiftlint_tag() {
    let result = Network::get_swiftlint_tag().await;
    assert!(!result.is_empty())
}