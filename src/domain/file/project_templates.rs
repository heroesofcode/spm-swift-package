pub struct ProjectTemplates;

impl ProjectTemplates {
    pub fn project_swift_content() -> String {
        r#"// The Swift Programming Language
// https://docs.swift.org/swift-book/
"#
        .to_string()
    }

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
            project_name, project_name
        )
    }

    pub fn package_swift_content(project_name: &str, platform: &str, version: &str) -> String {
        format!(
            r#"// swift-tools-version: 6.1
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
            name: "{}"),
        .testTarget(
            name: "{}Tests",
            dependencies: ["{}"]
        ),
    ]
)
"#,
            project_name, platform, version, project_name, 
            project_name, project_name, project_name, project_name
        )
    }

    pub fn changelog_content() -> String {
        r#"# CHANGELOG

## Version 1.0.0
**2024-01-18**

- First release
"#
        .to_string()
    }

    pub fn readme_content(project_name: &str) -> String {
        format!(
            r#"# {}
"#,
            project_name
        )
    }

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

    pub fn swiftlint_content() -> String {
        r#"disabled_rules:
  - line_length
  - force_try
  - identifier_name
  - trailing_whitespace
  - force_cast
  - colon
  - implicit_getter
  - void_return
  - unused_enumerated
  - function_parameter_count
  - file_length
  
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

    pub fn mise_content(tag: &str) -> String {
        format!(
            r#"[tools]
swiftlint = "{}"
"#,
            tag
        )
    }
}