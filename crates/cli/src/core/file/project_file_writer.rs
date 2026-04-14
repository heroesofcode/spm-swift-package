use std::{
	borrow::Cow,
	path::{Path, PathBuf},
};
use xx::file;

use crate::core::error::SpmError;
use crate::core::file::file_creator::{
	FileCreator, OptionalFileCreator, PackageCreator, ProjectCreator,
};
use crate::core::file::project_templates::ProjectTemplates;

type Result<T> = std::result::Result<T, SpmError>;

pub struct ProjectFileWriter;

impl PackageCreator for ProjectFileWriter {
	fn create_package(
		&self,
		project_name: &str,
		platform: &str,
		version: &str,
		is_plugin: bool,
	) -> Result<()> {
		let content =
			ProjectTemplates::package_swift_content(project_name, platform, version, is_plugin);
		root_write(project_name, "Package.swift", content)
	}
}

impl ProjectCreator for ProjectFileWriter {
	fn create_project(&self, project_name: &str) -> Result<()> {
		let content = ProjectTemplates::project_swift_content();
		let file_path = module_dir(project_name).join(format!("{project_name}.swift"));
		write(&file_path, content)
	}

	fn create_test_folder(&self, project_name: &str, test_framework: &str) -> Result<()> {
		let content = ProjectTemplates::test_content(project_name, test_framework);
		let file_path = tests_dir(project_name).join(format!("{project_name}Tests.swift"));
		write(&file_path, content)
	}
}

impl OptionalFileCreator for ProjectFileWriter {
	fn create_changelog(&self, project_name: &str) -> Result<()> {
		root_write(
			project_name,
			"CHANGELOG.md",
			ProjectTemplates::changelog_content(),
		)
	}

	fn create_readme(&self, project_name: &str) -> Result<()> {
		root_write(
			project_name,
			"README.md",
			ProjectTemplates::readme_content(project_name),
		)
	}

	fn create_spi(&self, project_name: &str) -> Result<()> {
		root_write(
			project_name,
			".spi.yml",
			ProjectTemplates::spi_content(project_name),
		)
	}

	fn create_swiftlint(&self, project_name: &str) -> Result<()> {
		root_write(
			project_name,
			".swiftlint.yml",
			ProjectTemplates::swiftlint_content(),
		)
	}
}

impl FileCreator for ProjectFileWriter {}

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
	write(&file_path, content)
}

fn write<P: AsRef<Path>, C: AsRef<str>>(path: P, content: C) -> Result<()> {
	if let Some(parent) = path.as_ref().parent() {
		std::fs::create_dir_all(parent).map_err(SpmError::Io)?;
	}

	file::write(path.as_ref(), content.as_ref().as_bytes())
		.map_err(|e| SpmError::Write(e.to_string()))
}
