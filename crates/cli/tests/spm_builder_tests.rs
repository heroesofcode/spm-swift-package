use spm_swift_package::core::spm_builder::SpmBuilder;
use std::path::Path;

#[test]
fn test_create_basic_project() {
	let project_name = "test_basic";
	let _ = std::fs::remove_dir_all(project_name);

	let result = SpmBuilder::new(project_name).platform("iOS").build();
	assert!(result.is_ok());

	assert!(Path::new(&format!("{}/Package.swift", project_name)).exists());
	assert!(
		Path::new(&format!(
			"{}/Sources/{}/{}.swift",
			project_name, project_name, project_name
		))
		.exists()
	);

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_with_changelog() {
	let project_name = "test_changelog";
	let _ = std::fs::remove_dir_all(project_name);

	let result = SpmBuilder::new(project_name)
		.platform("macOS")
		.files(["Changelog"])
		.build();

	assert!(result.is_ok());
	assert!(Path::new(&format!("{}/CHANGELOG.md", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_with_readme() {
	let project_name = "test_readme";
	let _ = std::fs::remove_dir_all(project_name);

	let result = SpmBuilder::new(project_name)
		.platform("iOS")
		.files(["Readme"])
		.build();
	
    assert!(result.is_ok());
	assert!(Path::new(&format!("{}/README.md", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_with_spi() {
	let project_name = "test_spi";
	let _ = std::fs::remove_dir_all(project_name);

	let result = SpmBuilder::new(project_name)
		.platform("iOS")
		.files(["Swift Package Index"])
		.build();
	
    assert!(result.is_ok());
	assert!(Path::new(&format!("{}/.spi.yml", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_with_swiftlint() {
	let project_name = "test_swiftlint";
	let _ = std::fs::remove_dir_all(project_name);

	let result = SpmBuilder::new(project_name)
		.platform("iOS")
		.files(["SwiftLint"])
		.build();
	
    assert!(result.is_ok());
	assert!(Path::new(&format!("{}/.swiftlint.yml", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_with_swift_testing() {
	let project_name = "test_swift_testing";
	let _ = std::fs::remove_dir_all(project_name);

	let result = SpmBuilder::new(project_name)
		.platform("iOS")
		.test_framework("Swift Testing")
		.build();
	
    assert!(result.is_ok());

	let test_file = format!(
		"{}/Tests/{}Tests/{}Tests.swift",
		project_name, project_name, project_name
	);
	let content = std::fs::read_to_string(&test_file).unwrap();
	assert!(content.contains("import Testing"));

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_with_all_options() {
	let project_name = "test_all";
	let _ = std::fs::remove_dir_all(project_name);

	let result = SpmBuilder::new(project_name)
		.platform("macOS")
		.test_framework("XCTest")
		.files(["Changelog", "Readme", "Swift Package Index", "SwiftLint"])
		.build();
	
    assert!(result.is_ok());

	assert!(Path::new(&format!("{}/CHANGELOG.md", project_name)).exists());
	assert!(Path::new(&format!("{}/README.md", project_name)).exists());
	assert!(Path::new(&format!("{}/.spi.yml", project_name)).exists());
	assert!(Path::new(&format!("{}/.swiftlint.yml", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}
