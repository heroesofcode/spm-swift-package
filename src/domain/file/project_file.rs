use std::fs;
use std::io::Write;

use crate::domain::file::project_templates::*;

pub struct ProjectFile;

impl ProjectFile {
    pub fn create_project(project_name: &str) -> Result<(), String> {
        let path = format!("{}/Sources/{}", project_name, project_name);
        Self::create_dir(&path)?;

        let content = ProjectTemplates::project_swift_content();
        let file_path = format!("{}/Sources/{}/{}.swift", project_name, project_name, project_name);
        Self::write_file(&file_path, &content)?;
        Ok(())
    }

    pub fn create_test_folder(project_name: &str) -> Result<(), String> {
        let path = format!("{}/Tests/{}Tests", project_name, project_name);
        Self::create_dir(&path)?;

        let content = ProjectTemplates::test_content(project_name);
        let file_path = format!("{}/Tests/{}Tests/{}Tests.swift", project_name, project_name, project_name);
        Self::write_file(&file_path, &content)?;
        Ok(())
    }

    pub fn create_package(project_name: &str, platform: &str, version: &str) -> Result<(), String> {
        let content = ProjectTemplates::package_swift_content(project_name, platform, version);
        Self::base_root_project(project_name, "Package.swift", content)
    }

    pub fn create_changelog(project_name: &str) -> Result<(), String> {
        let content = ProjectTemplates::changelog_content();
        Self::base_root_project(project_name, "CHANGELOG.md", content)
    }

    pub fn create_readme(project_name: &str) -> Result<(), String> {
        let content = ProjectTemplates::readme_content(project_name);
        Self::base_root_project(project_name, "README.md", content)
    }

    pub fn create_spi(project_name: &str) -> Result<(), String> {
        let content = ProjectTemplates::spi_content(project_name);
        Self::base_root_project(project_name, ".spi.yml", content)
    }

    pub fn create_swiftlint(project_name: &str) -> Result<(), String> {
        let content = ProjectTemplates::swiftlint_content();
        Self::base_root_project(project_name, ".swiftlint.yml", content)
    }

    pub fn create_mise(project_name: &str, tag: &str) -> Result<(), String> {
        let content = ProjectTemplates::mise_content(tag);
        Self::base_root_project(project_name, "mise.toml", content)
    }

    fn base_root_project(project_name: &str, name_file: &str, content: String) -> Result<(), String> {
        let path = project_name.to_string();
        Self::create_dir(&path)?;
        let file_path = format!("{}/{}", project_name, name_file);
        Self::write_file(&file_path, &content)
    }

    fn create_dir(path: &str) -> Result<(), String> {
        fs::create_dir_all(path)
            .map_err(|e| format!("Error creating directory '{}': {}", path, e))
    }

    fn write_file(path: &str, content: &str) -> Result<(), String> {
        let mut file = fs::File::create(path)
            .map_err(|e| format!("Error creating file '{}': {}", path, e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Error writing to file '{}': {}", path, e))
    }
}
