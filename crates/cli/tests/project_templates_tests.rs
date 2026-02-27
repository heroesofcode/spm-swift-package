use spm_swift_package::domain::file::project_templates::ProjectTemplates;

#[test]
fn test_project_swift_content() {
	let content = ProjectTemplates::project_swift_content();
	assert!(content.contains("The Swift Programming Language"));
	assert!(content.contains("https://docs.swift.org/swift-book/"));
}

#[test]
fn test_xctest_content() {
	let content = ProjectTemplates::test_content("MyLib", "XCTest");
	assert!(content.contains("import XCTest"));
	assert!(content.contains("@testable import MyLib"));
	assert!(content.contains("final class MyLibTests: XCTestCase"));
	assert!(content.contains("func testExample()"));
}

#[test]
fn test_swift_testing_content() {
	let content = ProjectTemplates::test_content("MyLib", "Swift Testing");
	assert!(content.contains("import Testing"));
	assert!(content.contains("@testable import MyLib"));
	assert!(content.contains("@Test func example()"));
}

#[test]
fn test_package_swift_without_plugin() {
	let content = ProjectTemplates::package_swift_content("TestPkg", "iOS", "26", false);
	assert!(content.contains("swift-tools-version: 6.2"));
	assert!(content.contains("name: \"TestPkg\""));
	assert!(content.contains(".iOS(.v26)"));
	assert!(!content.contains("SwiftLintPlugin"));
}

#[test]
fn test_package_swift_with_plugin() {
	let content = ProjectTemplates::package_swift_content("TestPkg", "macOS", "26", true);
	assert!(content.contains("SwiftLintPlugin"));
	assert!(content.contains("dependencies:"));
	assert!(content.contains("plugins:"));
}

#[test]
fn test_changelog_content() {
	let content = ProjectTemplates::changelog_content();
	assert!(content.contains("# CHANGELOG"));
	assert!(content.contains("Version 1.0.0"));
}

#[test]
fn test_readme_content() {
	let content = ProjectTemplates::readme_content("MyProject");
	assert!(content.contains("# MyProject"));
}

#[test]
fn test_spi_content() {
	let content = ProjectTemplates::spi_content("MyLib");
	assert!(content.contains("version: 1"));
	assert!(content.contains("documentation_targets: [MyLib]"));
	assert!(content.contains("scheme: MyLib"));
}

#[test]
fn test_swiftlint_content() {
	let content = ProjectTemplates::swiftlint_content();
	assert!(content.contains("disabled_rules:"));
	assert!(content.contains("opt_in_rules:"));
	assert!(content.contains("excluded:"));
}
