use crate::structure::*;
use crate::network::*;
use std::collections::HashMap;

pub struct Spm;

impl Spm {
    pub async fn create_spm(project_name: &str, selected: Vec<&str>, platform: Vec<&str>) {
        Structure::create_project(project_name);
        Structure::create_test_folder(project_name);
        Self::validation_platform(project_name, platform);
        
        if selected.contains(&"Changelog") {
            Structure::create_changelog(project_name);
        }

        if selected.contains(&"Readme") {
            Structure::create_readme(project_name);
        }

        if selected.contains(&"Swift Package Index") {
            Structure::create_spi(project_name);
        }

        if selected.contains(&"SwiftLint with mise") {
            let tag = Network::get_swiftlint_tag().await;
            Structure::create_mise(project_name, &tag);
            Structure::create_swiftlint(project_name);
        }
    }
    fn validation_platform(project_name: &str, platform: Vec<&str>) {
        let platforms = HashMap::from([
            ("iOS", "18"),
            ("macOS", "15"),
            ("watchOS", "11"),
            ("tvOS", "18"),
            ("visionOS", "2"),
        ]);

        for (key, version) in platforms {
            if platform.contains(&key) {
                Structure::create_package_swift(project_name, key, version);
            }
        }
    }

}
