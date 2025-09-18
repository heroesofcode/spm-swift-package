use crate::domain::file::project_file::*;
use std::collections::HashMap;

pub struct PlatformValidator;

impl PlatformValidator {
	pub fn generate_platform(project_name: &str, platform: Vec<&str>) {
		let platforms = HashMap::from([
			("iOS", "26"),
			("macOS", "26"),
			("watchOS", "26"),
			("tvOS", "26"),
			("visionOS", "26"),
		]);

		for (key, version) in platforms {
			if platform.contains(&key) {
				let _ = ProjectFile::create_package(project_name, key, version);
			}
		}
	}
}
