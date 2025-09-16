use serde::Deserialize;

#[derive(Deserialize)]
pub struct SwiftLintDto {
	pub(crate) tag_name: String,
}
