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
        let validation_empty = |s: &str| {
            if s.is_empty() {
                return Err("Library name cannot be empty");
            }
            Ok(())
        };

        let input = Input::new("Library name")
            .placeholder("Enter the library name")
            .prompt("Library: ")
            .validation(validation_empty);

        input.run().expect("error running input")
    }

    fn multi_select_input() -> Vec<&'static str> {
        let multi_select = MultiSelect::new("Add files")
            .description("Do you want to add some of these files?")
            .filterable(true)
            .option(DemandOption::new("Changelog"))
            .option(DemandOption::new("Swift Package Index"))
            .option(DemandOption::new("Readme"));

        let result = multi_select.run().expect("error running multi select");

        let selected: Vec<&str> = result
            .iter()
            .filter(|opt| !opt.is_empty())
            .copied()
            .collect();

        selected
    }

    fn loading() {
        Spinner::new("Building the Package...")
            .style(&SpinnerStyle::line())
            .run(|_| {
                sleep(Duration::from_secs(5));
            })
            .expect("error running spinner");
    }

    fn command_open_xcode(project_name: String) {
        let command = format!("cd {} && open Package.swift", project_name);
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .expect("Failed to open Xcode");
        
        child.wait().expect("Failed to wait on child");
    }    
}
