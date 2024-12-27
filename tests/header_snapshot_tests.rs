use insta::assert_snapshot;
use colored;
use spm_swift_package::Header;

#[test]
fn test_show_header_snapshot() {
    colored::control::set_override(false);
    let header = Header::show_header();
    assert_snapshot!(header);
}