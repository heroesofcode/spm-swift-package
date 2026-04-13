use crate::core::error::SpmError;

/// Core project file creation — sources and tests only
pub trait ProjectCreator {
	fn create_project(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_test_folder(&self, project_name: &str, test_framework: &str) -> Result<(), SpmError>;
}

/// Optional supplementary file creation
pub trait OptionalFileCreator {
	fn create_changelog(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_readme(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_spi(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_swiftlint(&self, project_name: &str) -> Result<(), SpmError>;
}

/// Package manifest creation — ensures Package.swift is part of the trait contract
pub trait PackageCreator {
	fn create_package(
		&self,
		project_name: &str,
		platform: &str,
		version: &str,
		is_plugin: bool,
	) -> Result<(), SpmError>;
}

/// Supertrait combining core and optional file creation
pub trait FileCreator: ProjectCreator + OptionalFileCreator {}
