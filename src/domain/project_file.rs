use std::fs;
use std::io::Write;

use crate::domain::project_templates::*;

pub struct ProjectFile;

impl ProjectFile {
    pub fn create_project(project_name: &str) {
        let path = format!("{}/Sources/{}", project_name, project_name);
        fs::create_dir_all(&path).expect("Error creating project");

        let content = ProjectTemplates::project_swift_content();

        let mut file = fs::File::create(format!(
            "{}/Sources/{}/{}.swift",
            project_name, project_name, project_name
        )).expect("Error writing to file");

        file.write_all(content.as_bytes()).expect("Error creating file");
    }

    pub fn create_test_folder(project_name: &str) {
        let path = format!("{}/Tests/{}Tests", project_name, project_name);
        fs::create_dir_all(&path).expect("Error creating test folder");

        let content = ProjectTemplates::test_content(project_name);

        let mut file = fs::File::create(format!(
            "{}/Tests/{}Tests/{}Tests.swift",
            project_name, project_name, project_name
        )).expect("Error creating file");

        file.write_all(content.as_bytes()).expect("Error writing to file");
    }

    pub fn create_package_swift(project_name: &str, platform: &str, version: &str) {
        let content = ProjectTemplates::package_swift_content(project_name, platform, version);
        Self::base_root_project(project_name, "Package.swift", content);
    }

    pub fn create_changelog(project_name: &str) {
        let content = ProjectTemplates::changelog_content();
        Self::base_root_project(project_name, "CHANGELOG.md", content);
    }

    pub fn create_readme(project_name: &str) {
        let content = ProjectTemplates::readme_content(project_name);
        Self::base_root_project(project_name, "README.md", content);
    }

    pub fn create_spi(project_name: &str) {
        let content = ProjectTemplates::spi_content(project_name);
        Self::base_root_project(project_name, ".spi.yml", content);
    }

    pub fn create_swiftlint(project_name: &str) {
        let content = ProjectTemplates::swiftlint_content();
        Self::base_root_project(project_name, ".swiftlint.yml", content);
    }

    pub fn create_mise(project_name: &str, tag: &str) {
        let content = ProjectTemplates::mise_content(tag);
        Self::base_root_project(project_name, "mise.toml", content);
    }

    fn base_root_project(project_name: &str, name_file: &str, content: String) {
        let path = project_name.to_string();
        fs::create_dir_all(&path).expect("Error creating test folder");

        let mut file =
            fs::File::create(format!("{}/{}", project_name, name_file))
                .expect("Error creating file");

        file.write_all(content.as_bytes()).expect("Error writing to file");
    }
}
