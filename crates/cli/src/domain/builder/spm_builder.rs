use crate::domain::file::project_file::*;
use crate::domain::platform::platform_validator::*;

/// Handles the creation of all project components based on selected options
pub struct SpmBuilder;

impl SpmBuilder {
	/// Builds the project structure, templates, and optional configuration files
	pub async fn builder(
		project_name: &str,
		selected_file: Vec<&str>,
		platform: Vec<&str>,
	) -> Result<(), String> {
		ProjectFile::create_project(project_name)?;
		ProjectFile::create_test_folder(project_name)?;

		if selected_file.contains(&"SwiftLint") {
			PlatformValidator::generate_platform(project_name, platform, true);
			ProjectFile::create_swiftlint(project_name)?;
		} else {
			PlatformValidator::generate_platform(project_name, platform, false);
		}

		Self::validate_selected_file(project_name, selected_file).await
	}

	/// Validates which optional files must be generated based on user selection
	async fn validate_selected_file(
		project_name: &str,
		selected_file: Vec<&str>,
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

		Ok(())
	}
}
