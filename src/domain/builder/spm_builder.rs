use crate::domain::file::project_file::*;
use crate::domain::platform::platform_validator::*;
use crate::domain::repository::swiftlint_repository::{SwiftLintRepository, SwiftLintRepositoryImpl};

pub struct SpmBuilder;

impl SpmBuilder {
    pub async fn builder(
        project_name: &str, 
        selected_file: Vec<&str>,
        platform: Vec<&str>
    ) -> Result<(), String> {
        ProjectFile::create_project(project_name)?;
        ProjectFile::create_test_folder(project_name)?;
        PlatformValidator::generate_platform(project_name, platform);
        Self::validate_selected_file(project_name, selected_file).await
    }

    async fn validate_selected_file(
        project_name: &str,
        selected_file: Vec<&str>
    ) -> Result<(), String> {
        if selected_file.contains(&"Changelog") {
            ProjectFile::create_changelog(project_name)?;
        }

        if selected_file.contains(&"Readme") {
            ProjectFile::create_readme(project_name)?;
        }

        if selected_file.contains(&"Swift Package Index") {
            ProjectFile::create_spi(project_name)?;
        }

        if selected_file.contains(&"SwiftLint with mise") {
            let repository = SwiftLintRepositoryImpl::new();

            match repository.get_latest_tag().await {
                Ok(tag) => {
                    ProjectFile::create_mise(project_name, &tag)?;
                    ProjectFile::create_swiftlint(project_name)?;
                },
                Err(error) => return Err(format!("Error fetching SwiftLint tag: {}", error)),
            }
        }
        Ok(())
    }
}
