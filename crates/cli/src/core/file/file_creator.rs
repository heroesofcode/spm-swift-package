use crate::core::error::SpmError;

/// Abstraction for creating Swift package project files (DIP)
pub trait FileCreator {
	fn create_project(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_test_folder(&self, project_name: &str, test_framework: &str) -> Result<(), SpmError>;
	fn create_changelog(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_readme(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_spi(&self, project_name: &str) -> Result<(), SpmError>;
	fn create_swiftlint(&self, project_name: &str) -> Result<(), SpmError>;
}
