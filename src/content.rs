pub struct Content;

impl Content {
    pub fn project_swift_content() -> String {
        let content =
            r#"// The Swift Programming Language
// https://docs.swift.org/swift-book/
"#;

        content.to_string()
    }

    pub fn test_content(project_name: &str) -> String {
        let content = format!(
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
"#, project_name, project_name);

        content
    }

    pub fn package_swift_content(project_name: &str) -> String {
        let content = format!(
            r#"// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "{}",
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
"#, project_name, project_name, project_name, project_name, project_name, project_name);

        content
    }

    pub fn changelog_content() -> String {
        let content =
            r#"# CHANGELOG

## Version 1.0.0
**2024-01-18**

- First release
"#;

        content.to_string()
    }

    pub fn readme_content(project_name: &str) -> String {
        project_name.to_string()
    }
}