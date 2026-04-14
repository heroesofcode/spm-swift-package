use spm_swift_package::core::file::file_creator::{
	OptionalFileCreator, PackageCreator, ProjectCreator,
};
use spm_swift_package::core::file::project_file_writer::ProjectFileWriter;
use std::path::Path;

#[test]
fn test_structure_create() {
	let project_name = "my_test_project";
	let _ = std::fs::remove_dir_all(project_name);
	let pf = ProjectFileWriter;

	pf.create_project(project_name).unwrap();

	let swift_file = format!(
		"{}/Sources/{}/{}.swift",
		project_name, project_name, project_name
	);

	assert!(Path::new(&swift_file).exists(), ".swift file not created");

	pf.create_test_folder(project_name, "XCTest").unwrap();

	let test_file = format!(
		"{}/Tests/{}Tests/{}Tests.swift",
		project_name, project_name, project_name
	);

	assert!(Path::new(&test_file).exists(), "Test file not created");

	ProjectFileWriter
		.create_package(project_name, "macOS", "15", false)
		.unwrap();
	assert!(Path::new(&format!("{}/Package.swift", project_name)).exists());

	pf.create_changelog(project_name).unwrap();
	assert!(Path::new(&format!("{}/CHANGELOG.md", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_readme() {
	let project_name = "test_readme_proj";
	let _ = std::fs::remove_dir_all(project_name);

	ProjectFileWriter.create_readme(project_name).unwrap();
	assert!(Path::new(&format!("{}/README.md", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_spi() {
	let project_name = "test_spi_proj";
	let _ = std::fs::remove_dir_all(project_name);

	ProjectFileWriter.create_spi(project_name).unwrap();
	assert!(Path::new(&format!("{}/.spi.yml", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}

#[test]
fn test_create_swiftlint() {
	let project_name = "test_swiftlint_proj";
	let _ = std::fs::remove_dir_all(project_name);

	ProjectFileWriter.create_swiftlint(project_name).unwrap();
	assert!(Path::new(&format!("{}/.swiftlint.yml", project_name)).exists());

	let _ = std::fs::remove_dir_all(project_name);
}
