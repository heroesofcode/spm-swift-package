use crate::core::error::SpmError;
use std::process::Command;

/// Opens the generated Package.swift in Xcode using a shell command
pub fn open_xcode(project_name: &str) -> Result<(), SpmError> {
	let command = format!("cd {} && open Package.swift", project_name);

	Command::new("sh")
		.arg("-c")
		.arg(&command)
		.spawn()
		.and_then(|mut child| child.wait())
		.map_err(SpmError::Io)?;

	Ok(())
}
