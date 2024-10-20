use demand::{Input, MultiSelect, DemandOption};

fn main() {
    println!("SPM Swift Package\n");

    let notempty_minlen = |s: &str| {
        if s.is_empty() {
            return Err("Name cannot be empty");
        }
        if s.len() < 8 {
            return Err("Name must be at least 8 characters");
        }
        Ok(())
    };

    let t = Input::new("Library name")
        .placeholder("Enter the library name")
        .prompt("Library: ")
        .validation(notempty_minlen);

    let i = t.run().expect("error running input");

    println!("The name is {}", i);
    print!("\n");

    let ms = MultiSelect::new("Toppings")
        .description("Select your files")
        .min(1)
        .filterable(true)
        .option(DemandOption::new("CHANGELOG"))
        .option(DemandOption::new("Swift Package Index"))
        .option(DemandOption::new("SwiftLint with mise"));
    
    let result = ms.run().expect("error running multi select");

    let selected: Vec<&str> = result
        .iter()
        .filter(|opt| !opt.is_empty()) 
        .copied() 
        .collect();

    println!("Toppings selecionados: {:?}", selected);
}
