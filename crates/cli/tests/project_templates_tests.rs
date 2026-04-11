use spm_swift_package::core::file::project_templates::ProjectTemplates;

#[test]
fn test_project_swift_content() {
	insta::assert_snapshot!(ProjectTemplates::project_swift_content());
}

#[test]
fn test_xctest_content() {
	insta::assert_snapshot!(ProjectTemplates::test_content("MyLib", "XCTest"));
}

#[test]
fn test_swift_testing_content() {
	insta::assert_snapshot!(ProjectTemplates::test_content("MyLib", "Swift Testing"));
}

#[test]
fn test_package_swift_without_plugin() {
	insta::assert_snapshot!(ProjectTemplates::package_swift_content(
		"TestPkg", "iOS", "26", false
	));
}

#[test]
fn test_package_swift_with_plugin() {
	insta::assert_snapshot!(ProjectTemplates::package_swift_content(
		"TestPkg", "macOS", "26", true
	));
}

#[test]
fn test_changelog_content() {
	insta::assert_snapshot!(ProjectTemplates::changelog_content());
}

#[test]
fn test_readme_content() {
	insta::assert_snapshot!(ProjectTemplates::readme_content("MyProject"));
}

#[test]
fn test_spi_content() {
	insta::assert_snapshot!(ProjectTemplates::spi_content("MyLib"));
}

#[test]
fn test_swiftlint_content() {
	insta::assert_snapshot!(ProjectTemplates::swiftlint_content());
}
