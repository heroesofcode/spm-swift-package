pub struct Content;

impl Content {
    pub fn project_swift_content() -> String {
        let content = 
r#"// The Swift Programming Language
// https://docs.swift.org/swift-book/
"#;

        return content.to_string()
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

        return content
    }
}