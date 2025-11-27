use spm_swift_package::domain::file::project_file::ProjectFile;
use std::path::Path;

#[test]
fn test_structure_create() {
	let project_name = "my_test_project";
	let _ = std::fs::remove_dir_all(project_name);
	ProjectFile::create_project(project_name).unwrap();

	let swift_file = format!(
		"{}/Sources/{}/{}.swift",
		project_name, project_name, project_name
	);
	assert!(Path::new(&swift_file).exists(), ".swift file not created");

	ProjectFile::create_test_folder(project_name).unwrap();

	let test_file = format!(
		"{}/Tests/{}Tests/{}Tests.swift",
		project_name, project_name, project_name
	);
	assert!(Path::new(&test_file).exists(), "Test file not created");

	ProjectFile::create_package(project_name, "macOS", "15", false).unwrap();
	assert!(Path::new(&format!("{}/Package.swift", project_name)).exists());

	ProjectFile::create_changelog(project_name).unwrap();
	assert!(Path::new(&format!("{}/CHANGELOG.md", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}