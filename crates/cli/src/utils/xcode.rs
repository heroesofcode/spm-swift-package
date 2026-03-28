use std::process::Command;

/// Opens the generated Package.swift in Xcode using a shell command
pub fn open_xcode(project_name: &str) -> Result<(), String> {
	let command = format!("cd {} && open Package.swift", project_name);

	Command::new("sh")
		.arg("-c")
		.arg(&command)
		.spawn()
		.and_then(|mut child| child.wait())
		.map_err(|e| format!("Failed to launch Xcode: {e}"))?;

	Ok(())
}
