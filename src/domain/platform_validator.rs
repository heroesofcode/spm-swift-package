use crate::domain::project_file::*;
use std::collections::HashMap;

pub struct PlatformValidator;

impl PlatformValidator {

    pub fn generate_platform(project_name: &str, platform: Vec<&str>) {
        let platforms = HashMap::from([
            ("iOS", "18"),
            ("macOS", "15"),
            ("watchOS", "11"),
            ("tvOS", "18"),
            ("visionOS", "2"),
        ]);

        for (key, version) in platforms {
            if platform.contains(&key) {
                ProjectFile::create_package_swift(project_name, key, version);
            }
        }
    }
}
