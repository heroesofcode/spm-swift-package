use crate::domain::project_file::*;
use crate::domain::platform_validator::*;
use crate::infrastructure::network::*;

pub struct SpmBuilder;

impl SpmBuilder {
    pub async fn builder(project_name: &str, selected: Vec<&str>, platform: Vec<&str>) {
        ProjectFile::create_project(project_name);
        ProjectFile::create_test_folder(project_name);
        PlatformValidator::generate_platform(project_name, platform);
        
        if selected.contains(&"Changelog") {
            ProjectFile::create_changelog(project_name);
        }

        if selected.contains(&"Readme") {
            ProjectFile::create_readme(project_name);
        }

        if selected.contains(&"Swift Package Index") {
            ProjectFile::create_spi(project_name);
        }

        if selected.contains(&"SwiftLint with mise") {
            let network = Network::new();

            match network.get_swiftlint_tag().await {
                Ok(tag) => {
                    ProjectFile::create_mise(project_name, &tag);
                    ProjectFile::create_swiftlint(project_name);
                },
                Err(error) => eprintln!("Error {}", error)
            }
        }
    }
}
