use crate::core::error::SpmError;
use crate::core::file::file_creator::PackageCreator;
use crate::core::file::project_file_writer::ProjectFileWriter;

/// Maps each supported platform to its minimum version string
const PLATFORM_VERSIONS: &[(&str, &str)] = &[
	("iOS", "26"),
	("macOS", "26"),
	("watchOS", "26"),
	("tvOS", "26"),
	("visionOS", "26"),
];

/// Abstraction for generating platform configuration
pub trait PlatformGenerator {
	fn generate(
		&self,
		project_name: &str,
		platforms: Vec<&str>,
		is_plugin: bool,
	) -> Result<(), SpmError>;
}

/// Validates and generates the appropriate platform configuration.
/// Calls `ProjectFileWriter` through the `PackageCreator` trait.
pub struct PlatformValidator;

impl PlatformGenerator for PlatformValidator {
	fn generate(
		&self,
		project_name: &str,
		platforms: Vec<&str>,
		is_plugin: bool,
	) -> Result<(), SpmError> {
		let writer = ProjectFileWriter;

		for platform in &platforms {
			if let Some(&(_, ver)) = PLATFORM_VERSIONS.iter().find(|(p, _)| p == platform) {
				writer.create_package(project_name, platform, ver, is_plugin)?;
			}
		}

		Ok(())
	}
}
