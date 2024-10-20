use demand::{Input, MultiSelect, DemandOption};

mod spm;
mod content;
mod structure;

use spm::Spm;

fn main() {
    println!("SPM Swift Package\n");

    let notempty_minlen = |s: &str| {
        if s.is_empty() {
            return Err("Library name cannot be empty");
        }
        Ok(())
    };

    let t = Input::new("Library name")
        .placeholder("Enter the library name")
        .prompt("Library: ")
        .validation(notempty_minlen);

    let project_name = t.run().expect("error running input");

    println!("The name is {}", project_name);
    print!("\n");

    let ms = MultiSelect::new("Toppings")
        .description("Select your files")
        .filterable(true)
        .option(DemandOption::new("Changelog"))
        .option(DemandOption::new("Readme"));
    
    let result = ms.run().expect("error running multi select");

    let selected: Vec<&str> = result
        .iter()
        .filter(|opt| !opt.is_empty()) 
        .copied() 
        .collect();

    print!("\n");

    Spm::create_spm(&project_name, selected);
}
