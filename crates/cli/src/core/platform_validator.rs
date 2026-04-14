use crate::core::error::SpmError;
use crate::core::file::file_creator::PackageCreator;

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
		package: &impl PackageCreator,
		project_name: &str,
		platforms: Vec<&str>,
		is_plugin: bool,
	) -> Result<(), SpmError>;
}

/// Validates and generates the appropriate platform configuration using the injected
/// [`PackageCreator`] implementation (the same object as `SpmBuilder`'s file creator).
pub struct PlatformValidator;

impl PlatformGenerator for PlatformValidator {
	fn generate(
		&self,
		package: &impl PackageCreator,
		project_name: &str,
		platforms: Vec<&str>,
		is_plugin: bool,
	) -> Result<(), SpmError> {
		for platform in &platforms {
			if let Some(&(_, ver)) = PLATFORM_VERSIONS.iter().find(|(p, _)| p == platform) {
				package.create_package(project_name, platform, ver, is_plugin)?;
			}
		}

		Ok(())
	}
}
