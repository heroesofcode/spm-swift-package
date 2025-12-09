/// Provides template generators for all project files
/// Each method returns the content of a specific file as a String
pub struct ProjectTemplates;

impl ProjectTemplates {
	/// Returns the default Swift source file template
	/// Used to generate the main `{name}.swift` file inside Sources
	pub fn project_swift_content() -> String {
		r#"// The Swift Programming Language
// https://docs.swift.org/swift-book/
"#
		.to_string()
	}

	/// Returns the default XCTest file template
	/// Includes boilerplate test structure for `{project_name}Tests.swift`
	pub fn test_content(project_name: &str) -> String {
		format!(
			r#"import XCTest
@testable import {}

final class {}Tests: XCTestCase {{
    func testExample() throws {{
        // XCTest Documentation
        // https://developer.apple.com/documentation/xctest

        // Defining Test Cases and Test Methods
        // https://developer.apple.com/documentation/xctest/defining_test_cases_and_test_methods
    }}
}}
"#,
			project_name, project_name,
		)
	}

	/// Returns the Package.swift template
	/// Supports two variants: plugin-enabled or standard package
	///
	/// * `project_name` - name of the Swift package
	/// * `platform` - selected platform such as iOS or macOS
	/// * `version` - minimum deployment version
	/// * `is_plugin` - adds SwiftLint plugin configuration when true
	pub fn package_swift_content(
		project_name: &str,
		platform: &str,
		version: &str,
		is_plugin: bool,
	) -> String {
		if is_plugin {
			format!(
				r#"// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "{}",
    platforms: [
        .{}(.v{})
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "{}",
            targets: ["{}"]),
    ],
    dependencies: [
        .package(url: "https://github.com/lukepistrol/SwiftLintPlugin", exact: "0.62.2")
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "{}",
            plugins: [
                .plugin(name: "SwiftLint", package: "SwiftLintPlugin")
            ]
        ),
        .testTarget(
            name: "{}Tests",
            dependencies: ["{}"]
        ),
    ]
)
"#,
				project_name,
				platform,
				version,
				project_name,
				project_name,
				project_name,
				project_name,
				project_name,
			)
		} else {
			format!(
				r#"// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "{}",
    platforms: [
        .{}(.v{})
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "{}",
            targets: ["{}"]),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "{}"
        ),
        .testTarget(
            name: "{}Tests",
            dependencies: ["{}"]
        ),
    ]
)
"#,
				project_name,
				platform,
				version,
				project_name,
				project_name,
				project_name,
				project_name,
				project_name,
			)
		}
	}

	/// Returns the default CHANGELOG.md template
	/// Used when the Changelog option is selected
	pub fn changelog_content() -> String {
		r#"# CHANGELOG

## Version 1.0.0
**2024-01-18**

- First release
"#
		.to_string()
	}

	/// Returns the README.md template
	/// Includes only the project title by default
	pub fn readme_content(project_name: &str) -> String {
		format!(
			r#"# {}
"#,
			project_name
		)
	}

	/// Returns the .spi.yml template for Swift Package Index configuration
	/// Defines documentation targets and scheme attributes
	pub fn spi_content(project_name: &str) -> String {
		format!(
			r#"version: 1
builder:
  configs:
    - documentation_targets: [{}]
      scheme: {}
"#,
			project_name, project_name
		)
	}

	/// Returns a default SwiftLint configuration template
	/// Includes baseline rules, opt-in rules, and excluded directories
	pub fn swiftlint_content() -> String {
		r#"disabled_rules:
  - trailing_whitespace

vertical_whitespace:
    severity: error

opt_in_rules:
  - empty_count
  - comma

excluded:
  - Pods
  - Carthage
  - Fastlane
"#
		.to_string()
	}
}
