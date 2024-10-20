use std::fs;
use std::io::Write;

use crate::content::*;

pub struct Structure;

impl Structure {
	pub fn create_project(project_name: &str) {
		let path = format!("{}/Sources/{}", project_name, project_name);
		fs::create_dir_all(&path).expect("Error creating project");

		let content = Content::project_swift_content();

		let mut file = fs::File::create(format!(
			"{}/Sources/{}/{}.swift",
			project_name, project_name, project_name
		))
		.expect("Error writing to file");

		file
			.write_all(content.as_bytes())
			.expect("Error creating file");

		println!("✅ Library {} created successfully", project_name);
	}

	pub fn create_test_folder(project_name: &str) {
		let path = format!("{}/Tests/{}Tests", project_name, project_name);
		fs::create_dir_all(&path).expect("Error creating test folder");

		let content = Content::test_content(project_name);

		let mut file = fs::File::create(format!(
			"{}/Tests/{}Tests/{}Tests.swift",
			project_name, project_name, project_name
		))
		.expect("Error creating file");

		file
			.write_all(content.as_bytes())
			.expect("Error writing to file");

		println!("✅ Test created successfully");
	}

	pub fn create_package_swift(project_name: &str) {
		let content = Content::package_swift_content(project_name);
		Self::base_root_project(project_name, "Package.swift", content);

		println!("✅ Package.swift created successfully");
	}

	pub fn create_changelog(project_name: &str) {
		let content = Content::changelog_content();
		Self::base_root_project(project_name, "CHANGELOG.md", content);

		println!("✅ Changelog created successfully");
	}

	pub fn create_readme(project_name: &str) {
		let content = Content::readme_content(project_name);
		Self::base_root_project(project_name, "README.md", content);

		println!("✅ Readme created successfully");
	}

	fn base_root_project(project_name: &str, name_file: &str, content: String) {
		let path = format!("{}", project_name);
		fs::create_dir_all(&path).expect("Error creating test folder");

		let mut file =
			fs::File::create(format!("{}/{}", project_name, name_file))
            .expect("Error creating file");

		file
			.write_all(content.as_bytes())
			.expect("Error writing to file");
	}
}
