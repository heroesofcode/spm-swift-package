use crate::core::error::SpmError;
use crate::core::file::project_file_writer::ProjectFileWriter;

/// Maps each supported platform to its minimum version string (OCP: add a platform here only)
const PLATFORM_VERSIONS: &[(&str, &str)] = &[
	("iOS", "26"),
	("macOS", "26"),
	("watchOS", "26"),
	("tvOS", "26"),
	("visionOS", "26"),
];

/// Abstraction for generating platform configuration (DIP)
pub trait PlatformGenerator {
	fn generate(
		&self,
		project_name: &str,
		platforms: Vec<&str>,
		is_plugin: bool,
	) -> Result<(), SpmError>;
}

/// Validates and generates the appropriate platform configuration
/// Responsible for creating the Package.swift file using the selected platform
pub struct PlatformValidator;

impl PlatformValidator {
	/// Generates the platform configuration for the project
	/// Creates a Package.swift file based on the selected platform and plugin flag
	///
	/// * `project_name` - name of the generated Swift package
	/// * `platforms` - vector containing the selected platform(s)
	/// * `is_plugin` - indicates whether the package includes a SwiftLint plugin
	pub fn generate_platform(
		project_name: &str,
		platforms: Vec<&str>,
		is_plugin: bool,
	) -> Result<(), SpmError> {
		for platform in &platforms {
			if let Some(&(_, ver)) = PLATFORM_VERSIONS.iter().find(|(p, _)| p == platform) {
				ProjectFileWriter::create_package(project_name, platform, ver, is_plugin)?;
			}
		}
		Ok(())
	}
}

impl PlatformGenerator for PlatformValidator {
	fn generate(
		&self,
		project_name: &str,
		platforms: Vec<&str>,
		is_plugin: bool,
	) -> Result<(), SpmError> {
		Self::generate_platform(project_name, platforms, is_plugin)
	}
}
