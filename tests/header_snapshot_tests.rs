use insta::assert_snapshot;
use spm_swift_package::Header;

#[test]
fn test_show_header_snapshot() {
    let header = Header::show_header();
    assert_snapshot!(header);
}