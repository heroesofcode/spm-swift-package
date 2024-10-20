use demand::{DemandOption, Input, MultiSelect, Spinner, SpinnerStyle};

use std::process::Command;
use std::{thread::sleep, time::Duration};

use crate::spm::*;

pub struct FieldsResults;

impl FieldsResults {
	pub fn result() {
		let project_name = Self::project_name_input();
		let selected = Self::multi_select_input();

		Self::loading();

		Spm::create_spm(&project_name, selected);

		Self::command_open_xcode(project_name);
	}

	fn project_name_input() -> String {
		let notempty_minlen = |s: &str| {
			if s.is_empty() {
				return Err("Library name cannot be empty");
			}
			Ok(())
		};

		let input = Input::new("Library name")
			.placeholder("Enter the library name")
			.prompt("Library: ")
			.validation(notempty_minlen);

		let project_name = input.run().expect("error running input");
		return project_name;
	}

	fn multi_select_input() -> Vec<&'static str> {
		let multi_select = MultiSelect::new("Toppings")
			.description("Select your files")
			.filterable(true)
			.option(DemandOption::new("Changelog"))
			.option(DemandOption::new("Readme"));

		let result = multi_select.run().expect("error running multi select");

		let selected: Vec<&str> = result
			.iter()
			.filter(|opt| !opt.is_empty())
			.copied()
			.collect();

		return selected;
	}

	fn loading() {
		Spinner::new("\nBuilding the Package...")
			.style(&SpinnerStyle::line())
			.run(|_| {
				sleep(Duration::from_secs(3));
			})
			.expect("error running spinner");
	}

	fn command_open_xcode(project_name: String) {
		let command = format!("cd {} && open Package.swift", project_name);
		Command::new("sh")
			.arg("-c")
			.arg(command)
			.spawn()
			.expect("Failed to open Xcode");
	}
}
