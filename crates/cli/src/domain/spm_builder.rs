use crate::domain::file::project_file::*;
use crate::domain::platform_validator::*;

/// Handles the creation of all project components based on selected options
pub struct SpmBuilder;

impl SpmBuilder {
	/// Builds the project structure, templates, and optional configuration files
	pub fn create(
		project_name: &str,
		selected_files: &[&str],
		platforms: &[&str],
	) -> Result<(), String> {
		ProjectFile::create_project(project_name)?;
		ProjectFile::create_test_folder(project_name)?;

		let has_swiftlint = selected_files.contains(&"SwiftLint");
		PlatformValidator::generate_platform(project_name, platforms.to_vec(), has_swiftlint);

		if has_swiftlint {
			ProjectFile::create_swiftlint(project_name)?;
		}

		Self::generate_optional_files(project_name, selected_files)
	}

	/// Generates optional files based on user selection
	fn generate_optional_files(project_name: &str, selected_files: &[&str]) -> Result<(), String> {
		if selected_files.contains(&"Changelog") {
			ProjectFile::create_changelog(project_name)?;
		}

		if selected_files.contains(&"Readme") {
			ProjectFile::create_readme(project_name)?;
		}

		if selected_files.contains(&"Swift Package Index") {
			ProjectFile::create_spi(project_name)?;
		}

		Ok(())
	}
}
