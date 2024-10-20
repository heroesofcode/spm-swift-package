use std::fs;
use std::io::Write;

use crate::content::*;

pub struct Spm;

impl Spm {
	pub fn create_spm(project_name: &str) {
		Self::create_project(project_name);
        Self::create_test_folder(project_name);
	}

    fn create_project(project_name: &str) {
        let path = format!("{}/Sources/{}", project_name, project_name);
		fs::create_dir_all(&path).expect("Error creating project");

		let project_swift_content = Content::project_swift_content();

		let mut file_project_swift = fs::File::create(format!(
			"{}/Sources/{}/{}.swift",
			project_name, project_name, project_name
		))
		.expect("Error writing to file");

		file_project_swift
			.write_all(project_swift_content.as_bytes())
			.expect("Error creating file");

        println!("✅ Library {} created successfully", project_name);
    }

    fn create_test_folder(project_name: &str) {
        let path = format!("{}/Tests/{}Tests", project_name, project_name);
		fs::create_dir_all(&path).expect("Error creating test folder");

		let project_swift_content = Content::test_content(project_name);

		let mut file_test_swift = fs::File::create(format!(
			"{}/Tests/{}Tests/{}Tests.swift",
			project_name, project_name, project_name
		))
		.expect("Error creating file");

		file_test_swift
			.write_all(project_swift_content.as_bytes())
			.expect("Error writing to file");

        println!("✅ Test {} created successfully", project_name);
    }
}
