use std::{
	borrow::Cow,
	path::{Path, PathBuf},
};
use xx::file;

use crate::domain::file::project_templates::ProjectTemplates;

type Result<T> = std::result::Result<T, String>;

pub struct ProjectFile;

impl ProjectFile {
	pub fn create_project(project_name: &str) -> Result<()> {
		let content = ProjectTemplates::project_swift_content();
		let file_path = Self::module_dir(project_name).join(format!("{project_name}.swift"));
		Self::write(&file_path, content)
	}

	pub fn create_test_folder(project_name: &str) -> Result<()> {
		let content = ProjectTemplates::test_content(project_name);
		let file_path = Self::tests_dir(project_name).join(format!("{project_name}Tests.swift"));
		Self::write(&file_path, content)
	}

	pub fn create_package(
		project_name: &str,
		platform: &str,
		version: &str,
		is_plugin: bool,
	) -> Result<()> {
		let content =
			ProjectTemplates::package_swift_content(project_name, platform, version, is_plugin);
		Self::root_write(project_name, "Package.swift", content)
	}

	pub fn create_changelog(project_name: &str) -> Result<()> {
		Self::root_write(
			project_name,
			"CHANGELOG.md",
			ProjectTemplates::changelog_content(),
		)
	}

	pub fn create_readme(project_name: &str) -> Result<()> {
		Self::root_write(
			project_name,
			"README.md",
			ProjectTemplates::readme_content(project_name),
		)
	}

	pub fn create_spi(project_name: &str) -> Result<()> {
		Self::root_write(
			project_name,
			".spi.yml",
			ProjectTemplates::spi_content(project_name),
		)
	}

	pub fn create_swiftlint(project_name: &str) -> Result<()> {
		Self::root_write(
			project_name,
			".swiftlint.yml",
			ProjectTemplates::swiftlint_content(),
		)
	}

	fn module_dir(project_name: &str) -> PathBuf {
		Path::new(project_name).join("Sources").join(project_name)
	}

	fn tests_dir(project_name: &str) -> PathBuf {
		Path::new(project_name)
			.join("Tests")
			.join(format!("{project_name}Tests"))
	}

	fn root_write<'a>(project_name: &str, filename: &str, content: Cow<'a, str>) -> Result<()> {
		let file_path = Path::new(project_name).join(filename);
		Self::write(&file_path, content)
	}

	fn write<P: AsRef<Path>, C: AsRef<str>>(path: P, content: C) -> Result<()> {
		file::write(path.as_ref(), content.as_ref().as_bytes()).map_err(|e| e.to_string())
	}
}
