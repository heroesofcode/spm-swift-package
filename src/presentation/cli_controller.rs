use demand::{
    DemandOption, 
    Input, 
    MultiSelect, 
    Spinner, 
    SpinnerStyle
};
use std::process::{exit, Command};
use std::{thread::sleep, time::Duration};
use colored::Colorize;

use crate::domain::usecase::usecase::*;

pub struct CliController;

impl CliController {

    pub async fn execute_flow() {
        let project_name = Self::project_name_input();
        let file_selected = Self::multiselect_files();
        let platform_selected = Self::multiselect_platform();

        Self::loading();
        SpmUseCase::execute(&project_name, file_selected, platform_selected).await;
        Self::command_open_xcode(project_name);
    }

    // Internal functions

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

        input.run().unwrap_or_else(|e| {
            if e.kind() == std::io::ErrorKind::Interrupted {
                println!("{}", e);
                exit(0)
            } else {
                panic!("Error: {}", e);
            }
        })
    }

    fn multiselect_options(
        prompt: &str,
        description: &str,
        options: &[&'static str],
    ) -> Vec<&'static str> {
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
                        println!("Operation interrupted.");
                        exit(0);
                    } else {
                        panic!("Error: {}", e);
                    }
                }
            };

            let selected: Vec<&str> = result
                .iter()
                .filter(|opt| !opt.is_empty())
                .copied()
                .collect();

            if selected.is_empty() {
                println!("{}", "You need to choose in order to follow".yellow());
                continue;
            }

            return selected;
        }
    }

    fn multiselect_files() -> Vec<&'static str> {
        Self::multiselect_options(
            "Add files",
            "Do you want to add some of these files?",
            &["Changelog", "Swift Package Index", "Readme", "SwiftLint with mise"]
        )
    }

    fn multiselect_platform() -> Vec<&'static str> {
        Self::multiselect_options(
            "Choose platform",
            "Which platform do you want to choose?",
            &["iOS", "macOS", "tvOS", "watchOS", "visionOS"]
        )
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
