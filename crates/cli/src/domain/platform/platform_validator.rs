use crate::domain::file::project_file::*;
use std::collections::HashMap;

/// Validates and generates the appropriate platform configuration
/// Responsible for creating the Package.swift file using the selected platform
pub struct PlatformValidator;

impl PlatformValidator {
	/// Generates the platform configuration for the project
	/// Creates a Package.swift file based on the selected platform and plugin flag
	///
	/// * `project_name` - name of the generated Swift package
	/// * `platform` - vector containing the selected platform
	/// * `is_plugin` - indicates whether the package is a Swift plugin
	pub fn generate_platform(project_name: &str, platform: Vec<&str>, is_plugin: bool) {
		let platforms = HashMap::from([
			("iOS", "26"),
			("macOS", "26"),
			("watchOS", "26"),
			("tvOS", "26"),
			("visionOS", "26"),
		]);

		for (key, version) in platforms {
			if platform.contains(&key) {
				let _ = ProjectFile::create_package(project_name, key, version, is_plugin);
			}
		}
	}
}
