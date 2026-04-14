use spm_swift_package::core::file::project_file_writer::ProjectFileWriter;
use spm_swift_package::core::platform_validator::{PlatformGenerator, PlatformValidator};
use std::path::Path;

#[test]
fn test_generate_platform_ios() {
	let project_name = "test_ios_project";
	let _ = std::fs::remove_dir_all(project_name);

	let _ = PlatformValidator.generate(&ProjectFileWriter, project_name, vec!["iOS"], false);

	assert!(Path::new(&format!("{}/Package.swift", project_name)).exists());
	let content = std::fs::read_to_string(format!("{}/Package.swift", project_name)).unwrap();
	assert!(content.contains(".iOS(.v26)"));

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_generate_platform_macos() {
	let project_name = "test_macos_project";
	let _ = std::fs::remove_dir_all(project_name);

	let _ = PlatformValidator.generate(&ProjectFileWriter, project_name, vec!["macOS"], false);

	assert!(Path::new(&format!("{}/Package.swift", project_name)).exists());
	let content = std::fs::read_to_string(format!("{}/Package.swift", project_name)).unwrap();
	assert!(content.contains(".macOS(.v26)"));

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_generate_platform_with_plugin() {
	let project_name = "test_plugin_project";
	let _ = std::fs::remove_dir_all(project_name);

	let _ = PlatformValidator.generate(&ProjectFileWriter, project_name, vec!["iOS"], true);

	let content = std::fs::read_to_string(format!("{}/Package.swift", project_name)).unwrap();
	assert!(content.contains("SwiftLintPlugin"));

	let _ = std::fs::remove_dir_all(project_name);
}
