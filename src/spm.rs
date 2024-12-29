use crate::structure::*;
use crate::network::*;

pub struct Spm;

impl Spm {
    pub async fn create_spm(project_name: &str, selected: Vec<&str>) {
        Structure::create_project(project_name);
        Structure::create_test_folder(project_name);
        Structure::create_package_swift(project_name);

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
}
