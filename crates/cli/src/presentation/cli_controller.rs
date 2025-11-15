use colored::Colorize;
use demand::{DemandOption, Input, MultiSelect, Select, Spinner, SpinnerStyle};
use std::process::Command;

use spm_core::domain::usecase::usecase::*;

pub struct CliController;

impl CliController {
	pub async fn execute_flow() -> Result<(), String> {
		let project_name = Self::project_name_input()?;
		let file_selected = Self::multiselect_files()?;
		let platform_selected = Self::select_platform()?;

		Self::loading().await?;
		SpmUseCase::execute(
			&project_name, 
			file_selected, 
			vec![platform_selected]
		).await?;
		
		Self::command_open_xcode(&project_name)?;
		Ok(())
	}

	// Internal functions

	fn project_name_input() -> Result<String, String> {
		let validation_empty = |s: &str| {
			if s.is_empty() {
				Err("Library name cannot be empty")
			} else {
				Ok(())
			}
		};

		let input = Input::new("Library name")
			.placeholder("Enter the library name")
			.prompt("Library: ")
			.validation(validation_empty);

		input.run().map_err(|e| {
			if e.kind() == std::io::ErrorKind::Interrupted {
				"Operation interrupted by user.".to_string()
			} else {
				format!("Error getting library name: {}", e)
			}
		})
	}

	fn multiselect_options(
		prompt: &str,
		description: &str,
		options: &[&'static str],
	) -> Result<Vec<&'static str>, String> {
		loop {
			let mut multiselect = MultiSelect::new(prompt)
				.description(description)
				.filterable(true);

			for &option in options {
				multiselect = multiselect.option(DemandOption::new(option));
			}

			let result = match multiselect.run() {
				Ok(selection) => selection,
				Err(e) => {
					if e.kind() == std::io::ErrorKind::Interrupted {
						return Err("Operation interrupted by user.".to_string());
					} else {
						return Err(format!("Error selecting options: {}", e));
					}
				}
			};

			let selected: Vec<&str> = result
				.iter()
				.filter(|opt| !opt.is_empty())
				.copied()
				.collect();

			if selected.is_empty() {
				println!(
					"{}",
					"You need to choose at least one option to continue".yellow()
				);
				continue;
			}

			return Ok(selected);
		}
	}

	fn multiselect_files() -> Result<Vec<&'static str>, String> {
		Self::multiselect_options(
			"Add files",
			"Do you want to add some of these files?",
			&[
				"Changelog",
				"Swift Package Index",
				"Readme",
				"SwiftLint",
			],
		)
	}

	fn select_platform() -> Result<&'static str, String> {
		let mut select = Select::new("Choose platform")
			.description("Which platform do you want to choose?")
			.filterable(true);
		
		for option in ["iOS", "macOS", "tvOS", "watchOS", "visionOS"].iter() {
			select = select.option(DemandOption::new(*option));
		}

		select.run().map_err(|e| {
			if e.kind() == std::io::ErrorKind::Interrupted {
				"Operation interrupted by user.".to_string()
			} else {
				format!("Error selecting platform: {}", e)
			}
		})
	}

	async fn loading() -> Result<(), String> {
		Spinner::new("Building the Package...")
			.style(&SpinnerStyle::line())
			.run(|_| {
				std::thread::sleep(std::time::Duration::from_secs(5));
			})
			.map_err(|_| "Error running spinner".to_string())
	}

	fn command_open_xcode(project_name: &str) -> Result<(), String> {
		let command = format!("cd {} && open Package.swift", project_name);
		let mut child = Command::new("sh")
			.arg("-c")
			.arg(&command)
			.spawn()
			.map_err(|e| format!("Failed to open Xcode: {}", e))?;

		child
			.wait()
			.map_err(|e| format!("Failed to wait for Xcode: {}", e))?;
		Ok(())
	}
}
