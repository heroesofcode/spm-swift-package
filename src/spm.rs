use crate::structure::*;
use crate::network::*;

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
        if platform.contains(&"iOS") {
            Structure::create_package_swift(project_name, "iOS", "18");
        } else if platform.contains(&"macOS") {
            Structure::create_package_swift(project_name, "macOS", "15");
        } else if platform.contains(&"watchOS") {
            Structure::create_package_swift(project_name, "watchOS", "11");
        } else if platform.contains(&"tvOS") {
            Structure::create_package_swift(project_name, "tvOS", "18");
        } else if platform.contains(&"visionOS") {
            Structure::create_package_swift(project_name, "visionOS", "2");
        }
    }
}
