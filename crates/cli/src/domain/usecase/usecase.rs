use crate::domain::builder::spm_builder::*;

pub struct SpmUseCase;

impl SpmUseCase {
	pub async fn execute(
		project_name: &str,
		selected_file: Vec<&str>,
		platform: Vec<&str>,
	) -> Result<(), String> {
		SpmBuilder::builder(project_name, selected_file, platform).await
	}
}