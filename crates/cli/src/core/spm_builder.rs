use crate::core::file::project_file::*;
use crate::core::platform_validator::*;

/// Handles the creation of all project components based on selected options
pub struct SpmBuilder;

impl SpmBuilder {
	/// Builds the project structure, templates, and optional configuration files
	pub fn create<T: AsRef<str>>(
		project_name: &str,
		selected_files: &[T],
		platforms: &[&str],
		test_framework: &str,
	) -> Result<(), String> {
		ProjectFile::create_project(project_name)?;
		ProjectFile::create_test_folder(project_name, test_framework)?;

		let has_swiftlint = selected_files.iter().any(|f| f.as_ref() == "SwiftLint");
		PlatformValidator::generate_platform(project_name, platforms.to_vec(), has_swiftlint);

		if has_swiftlint {
			ProjectFile::create_swiftlint(project_name)?;
		}

		Self::generate_optional_files(project_name, selected_files)
	}

	/// Generates optional files based on user selection
	fn generate_optional_files<T: AsRef<str>>(
		project_name: &str,
		selected_files: &[T],
	) -> Result<(), String> {
		if selected_files.iter().any(|f| f.as_ref() == "Changelog") {
			ProjectFile::create_changelog(project_name)?;
		}

		if selected_files.iter().any(|f| f.as_ref() == "Readme") {
			ProjectFile::create_readme(project_name)?;
		}

		if selected_files.iter().any(|f| f.as_ref() == "Swift Package Index") {
			ProjectFile::create_spi(project_name)?;
		}

		Ok(())
	}
}
