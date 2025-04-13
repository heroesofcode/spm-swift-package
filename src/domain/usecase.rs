use crate::domain::spm_builder::*;

pub struct SpmUseCase;

impl SpmUseCase {

    pub async fn execute(project_name: &str, selected: Vec<&str>, platform: Vec<&str>) {
        SpmBuilder::builder(&project_name, selected, platform).await;
    }
}