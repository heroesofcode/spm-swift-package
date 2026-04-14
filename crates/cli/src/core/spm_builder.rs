use crate::core::error::SpmError;
use crate::core::file::file_creator::FileCreator;
use crate::core::file::project_file_writer::ProjectFileWriter;
use crate::core::platform_validator::{PlatformGenerator, PlatformValidator};

/// Builds a Swift Package Manager project using a fluent builder API
pub struct SpmBuilder<F = ProjectFileWriter, P = PlatformValidator> {
	file_creator: F,
	platform_gen: P,
	project_name: String,
	platforms: Vec<String>,
	test_framework: String,
	selected_files: Vec<String>,
}

/// Constructor using the concrete default implementations
impl SpmBuilder {
	pub fn new(project_name: impl Into<String>) -> Self {
		Self {
			file_creator: ProjectFileWriter,
			platform_gen: PlatformValidator,
			project_name: project_name.into(),
			platforms: Vec::new(),
			test_framework: "XCTest".to_string(),
			selected_files: Vec::new(),
		}
	}
}

impl<F: FileCreator, P: PlatformGenerator> SpmBuilder<F, P> {
	/// Adds a target platform (e.g. "iOS", "macOS")
	pub fn platform(mut self, platform: impl Into<String>) -> Self {
		self.platforms.push(platform.into());
		self
	}

	/// Sets the test framework ("XCTest" or "Swift Testing")
	pub fn test_framework(mut self, framework: impl Into<String>) -> Self {
		self.test_framework = framework.into();
		self
	}

	/// Adds optional files to include (e.g. "Changelog", "Readme", "SwiftLint")
	pub fn files(mut self, files: impl IntoIterator<Item = impl Into<String>>) -> Self {
		self
			.selected_files
			.extend(files.into_iter().map(Into::into));
		self
	}

	/// Generates the project structure on disk
	pub fn build(self) -> Result<(), SpmError> {
		let name = &self.project_name;
		let has_swiftlint = self.selected_files.iter().any(|f| f == "SwiftLint");
		let platforms: Vec<&str> = self.platforms.iter().map(|s| s.as_str()).collect();

		self.file_creator.create_project(name)?;
		self
			.file_creator
			.create_test_folder(name, &self.test_framework)?;
		self
			.platform_gen
			.generate(&self.file_creator, name, platforms, has_swiftlint)?;

		if has_swiftlint {
			self.file_creator.create_swiftlint(name)?;
		}

		self.generate_optional_files(name)
	}

	/// Creates each optional file that was selected
	fn generate_optional_files(&self, name: &str) -> Result<(), SpmError> {
		for file in &self.selected_files {
			match file.as_str() {
				"Changelog" => self.file_creator.create_changelog(name)?,
				"Readme" => self.file_creator.create_readme(name)?,
				"Swift Package Index" => self.file_creator.create_spi(name)?,
				_ => {}
			}
		}

		Ok(())
	}
}
