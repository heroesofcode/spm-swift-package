use colored::Colorize;
use demand::{Confirm, DemandOption, Input, MultiSelect, Select, Spinner, SpinnerStyle};

use crate::core::error::SpmError;
use crate::core::spm_builder::SpmBuilder;
use crate::utils::xcode;

/// Controls all CLI interactions and orchestrates the project creation flow
pub struct Cli;

impl Cli {
	/// Executes the complete flow: prompts user input, builds the project, and opens it in Xcode
	pub async fn execute_flow() -> Result<(), SpmError> {
		let project_name = Self::project_name_input()?;
		let file_selected = Self::multiselect_files()?;
		let platform_selected = Self::select_platform()?;
		let test_framework = Self::select_test_framework()?;

		Self::loading().await?;

		SpmBuilder::new(&project_name)
			.platform(platform_selected)
			.test_framework(test_framework)
			.files(file_selected.iter().copied())
			.build()?;

		Self::confirm_open_xcode(project_name)?;

		Ok(())
	}

	// Internal functions

	/// Prompts the user to input the library or module name
	/// Validates that the name is not empty
	fn project_name_input() -> Result<String, SpmError> {
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
				SpmError::Interrupted
			} else {
				SpmError::Io(e)
			}
		})
	}

	/// Creates a generic multiselect component with a prompt, description, and list of options
	/// Ensures at least one option is selected before continuing
	fn multiselect_options(
		prompt: &str,
		description: &str,
		options: &[&'static str],
	) -> Result<Vec<&'static str>, SpmError> {
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
						return Err(SpmError::Interrupted);
					} else {
						return Err(SpmError::Io(e));
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

	/// Defines the file options available for selection (Changelog, SPI, README, SwiftLint)
	fn multiselect_files() -> Result<Vec<&'static str>, SpmError> {
		Self::multiselect_options(
			"Add files",
			"Do you want to add some of these files?",
			&["Changelog", "Swift Package Index", "Readme", "SwiftLint"],
		)
	}

	/// Displays a select input for platform choice
	/// The result is always a single selected platform
	fn select_platform() -> Result<&'static str, SpmError> {
		let mut select = Select::new("Choose platform")
			.description("Which platform do you want to choose?")
			.filterable(true);

		for option in ["iOS", "macOS", "tvOS", "watchOS", "visionOS"].iter() {
			select = select.option(DemandOption::new(*option));
		}

		select.run().map_err(|e| {
			if e.kind() == std::io::ErrorKind::Interrupted {
				SpmError::Interrupted
			} else {
				SpmError::Io(e)
			}
		})
	}

	/// Displays a select input for test framework choice
	fn select_test_framework() -> Result<&'static str, SpmError> {
		let mut select = Select::new("Choose test framework")
			.description("Which test framework do you want to use?")
			.filterable(true);

		for option in ["XCTest", "Swift Testing"].iter() {
			select = select.option(DemandOption::new(*option));
		}

		select.run().map_err(|e| {
			if e.kind() == std::io::ErrorKind::Interrupted {
				SpmError::Interrupted
			} else {
				SpmError::Io(e)
			}
		})
	}

	/// Shows a loading spinner while running a simulated build step
	/// Uses a 5 second delay for effect
	async fn loading() -> Result<(), SpmError> {
		Spinner::new("Building the Package...")
			.style(&SpinnerStyle::line())
			.run(|_| {
				std::thread::sleep(std::time::Duration::from_secs(5));
			})
			.map_err(SpmError::Io)
	}

	/// Asks the user whether to open the generated package in Xcode
	/// Opens Xcode if confirmed, otherwise returns without opening Xcode
	fn confirm_open_xcode(project_name: String) -> Result<(), SpmError> {
		let is_yes = Confirm::new("Do you want to open the package in Xcode?")
			.affirmative("Yes")
			.negative("No")
			.run()
			.map_err(|e| {
				if e.kind() == std::io::ErrorKind::Interrupted {
					SpmError::Interrupted
				} else {
					SpmError::Io(e)
				}
			})?;

		if is_yes {
			xcode::open_xcode(&project_name)?;
		}

		Ok(())
	}
}
