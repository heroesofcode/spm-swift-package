use std::{
	fs,
	io::Write,
	path::{Path, PathBuf},
};

use crate::domain::file::project_templates::*;

type Result<T> = std::result::Result<T, String>;

fn display<P: AsRef<Path>>(p: P) -> String {
	p.as_ref().display().to_string()
}

pub struct ProjectFile;

impl ProjectFile {
	pub fn create_project(project_name: &str) -> Result<()> {
		let module_dir = Self::module_dir(project_name);
		Self::create_dir(&module_dir)?;

		let content = ProjectTemplates::project_swift_content();
		let file_path = module_dir.join(format!("{name}.swift", name = project_name));
		Self::write_file(&file_path, &content)
	}

	pub fn create_test_folder(project_name: &str) -> Result<()> {
		let tests_dir = Self::tests_dir(project_name);
		Self::create_dir(&tests_dir)?;

		let content = ProjectTemplates::test_content(project_name);
		let file_path = tests_dir.join(format!("{name}Tests.swift", name = project_name));
		Self::write_file(&file_path, &content)
	}

	pub fn create_package(
		project_name: &str,
		platform: &str,
		version: &str,
		is_plugin: bool,
	) -> Result<()> {
		let content =
			ProjectTemplates::package_swift_content(project_name, platform, version, is_plugin);
		Self::create_root_file(project_name, "Package.swift", content)
	}

	pub fn create_changelog(project_name: &str) -> Result<()> {
		let content = ProjectTemplates::changelog_content();
		Self::create_root_file(project_name, "CHANGELOG.md", content)
	}

	pub fn create_readme(project_name: &str) -> Result<()> {
		let content = ProjectTemplates::readme_content(project_name);
		Self::create_root_file(project_name, "README.md", content)
	}

	pub fn create_spi(project_name: &str) -> Result<()> {
		let content = ProjectTemplates::spi_content(project_name);
		Self::create_root_file(project_name, ".spi.yml", content)
	}

	pub fn create_swiftlint(project_name: &str) -> Result<()> {
		let content = ProjectTemplates::swiftlint_content();
		Self::create_root_file(project_name, ".swiftlint.yml", content)
	}

	// Private

	fn module_dir(project_name: &str) -> PathBuf {
		Path::new(project_name).join("Sources").join(project_name)
	}

	fn tests_dir(project_name: &str) -> PathBuf {
		Path::new(project_name)
			.join("Tests")
			.join(format!("{name}Tests", name = project_name))
	}

	fn create_root_file(project_name: &str, filename: &str, content: String) -> Result<()> {
		let root = Path::new(project_name);
		Self::create_dir(root)?;
		let file_path = root.join(filename);
		Self::write_file(&file_path, &content)
	}

	fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
		fs::create_dir_all(&path)
			.map_err(|e| format!("Error creating directory '{}': {}", display(path), e))
	}

	fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
		let mut file = fs::File::create(&path)
			.map_err(|e| format!("Error creating file '{}': {}", display(&path), e))?;
		file
			.write_all(content.as_bytes())
			.map_err(|e| format!("Error writing to file '{}': {}", display(&path), e))
	}
}