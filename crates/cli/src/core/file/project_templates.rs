use std::borrow::Cow;

/// Provides template generators for all project files.
pub struct ProjectTemplates;

impl ProjectTemplates {
	const SWIFT_TOOLS_VERSION: &'static str = "6.2";
	const SWIFTLINT_PLUGIN_URL: &'static str = "https://github.com/lukepistrol/SwiftLintPlugin";
	const SWIFTLINT_PLUGIN_VERSION: &'static str = "0.63.1";

	/// Returns the default Swift source file template
	pub fn project_swift_content() -> Cow<'static, str> {
		Cow::Borrowed(
			r#"// The Swift Programming Language
// https://docs.swift.org/swift-book/
"#,
		)
	}

	/// Returns the test file template based on the selected framework
	pub fn test_content(project_name: &str, test_framework: &str) -> Cow<'static, str> {
		match test_framework {
			"Swift Testing" => Cow::Owned(format!(
				r#"import Testing
@testable import {name}

@Test func example() {{
    // Swift Testing Documentation
    // https://developer.apple.com/documentation/testing
}}
"#,
				name = project_name
			)),
			_ => Cow::Owned(format!(
				r#"import XCTest
@testable import {name}

final class {name}Tests: XCTestCase {{
    func testExample() throws {{
        // XCTest Documentation
        // https://developer.apple.com/documentation/xctest

        // Defining Test Cases and Test Methods
        // https://developer.apple.com/documentation/xctest/defining_test_cases_and_test_methods
    }}
}}
"#,
				name = project_name
			)),
		}
	}

	/// Returns the Package.swift template
	pub fn package_swift_content(
		project_name: &str,
		platform: &str,
		version: &str,
		is_plugin: bool,
	) -> Cow<'static, str> {
		let deps = if is_plugin {
			// keep indentation exactly as it should appear in Package.swift
			format!(
				r#"    dependencies: [
        .package(url: "{url}", exact: "{ver}")
    ],
"#,
				url = Self::SWIFTLINT_PLUGIN_URL,
				ver = Self::SWIFTLINT_PLUGIN_VERSION
			)
		} else {
			String::new()
		};

		let plugins = if is_plugin {
			r#"            plugins: [
                .plugin(name: "SwiftLint", package: "SwiftLintPlugin")
            ]
"#
			.to_string()
		} else {
			String::new()
		};

		Cow::Owned(format!(
			r#"// swift-tools-version: {tools}
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "{name}",
    platforms: [
        .{platform}(.v{version})
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "{name}",
            targets: ["{name}"]
        ),
    ],
{deps}    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "{name}",
{plugins}        ),
        .testTarget(
            name: "{name}Tests",
            dependencies: ["{name}"]
        ),
    ]
)
"#,
			tools = Self::SWIFT_TOOLS_VERSION,
			name = project_name,
			platform = platform,
			version = version,
			deps = deps,
			plugins = plugins
		))
	}

	/// Returns the default CHANGELOG.md template
	pub fn changelog_content() -> Cow<'static, str> {
		Cow::Borrowed(
			r#"# CHANGELOG

## Version 1.0.0
**2024-01-18**

- First release
"#,
		)
	}

	/// Returns the README.md template
	pub fn readme_content(project_name: &str) -> Cow<'static, str> {
		Cow::Owned(format!("# {name}\n", name = project_name))
	}

	/// Returns the .spi.yml template for Swift Package Index configuration
	pub fn spi_content(project_name: &str) -> Cow<'static, str> {
		Cow::Owned(format!(
			r#"version: 1
builder:
  configs:
    - documentation_targets: [{name}]
      scheme: {name}
"#,
			name = project_name
		))
	}

	/// Returns a default SwiftLint configuration template
	pub fn swiftlint_content() -> Cow<'static, str> {
		Cow::Borrowed(
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
"#,
		)
	}
}
