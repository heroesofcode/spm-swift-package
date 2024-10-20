use crate::structure::*;

pub struct Spm;

impl Spm {
	pub fn create_spm(project_name: &str, selected: Vec<&str>) {
		Structure::create_project(project_name);
        Structure::create_test_folder(project_name);
        Structure::create_package_swift(project_name);

        if selected.contains(&"Changelog") {
            Structure::create_changelog(project_name);
        } 

        if selected.contains(&"Readme") {
            Structure::create_readme(project_name);
        }
	}
}
