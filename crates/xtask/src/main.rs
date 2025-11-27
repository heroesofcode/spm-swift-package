use xshell::{cmd, Shell};
use demand::Input;

fn main() -> anyhow::Result<()> {
	header();
	let shell = Shell::new()?;
	input_option_validation(shell)?;

	Ok(())
}

fn header() {
	println!("1️⃣  Run spm-swift-package");
	println!("2️⃣  Run all testes");
	println!("3️⃣  Publish package");
	println!("4️⃣  Running cargo publish (dry-run)");
	println!("5️⃣  Preparing tar.gz to homebrew");
	println!();
}

fn input_option_validation(shell: Shell) -> anyhow::Result<()> {
	let validation_input = |s: &str| {
        if s.is_empty() {
            return Err("Input cannot be empty");
        }

        Ok(())
    };

    let option_input = Input::new("Choose an option: ")
        .prompt("Option: ")
        .validation(validation_input);

    let option = option_input.run().expect("error running input");

	match option.as_str() {
		"1" => {
			println!("🚀 Running spm-swift-package");
			println!();
			cmd!(shell, "cargo run").run()?;
		}
		"2" => {
			println!("✅ ❌ Running all testes");
			println!();
			cmd!(shell, "mise test").run()?;
		}
		"3" => {
			println!("📦 Publishing package");
			println!();
			cmd!(shell, "cargo publish").run()?;
		}
		"4" => {
			println!("📦 Running cargo publish (dry-run)");
			println!();
			cmd!(shell, "cargo publish --dry-run").run()?;
		}
		"5" => {
			println!("📦 Preparing tar.gz to homebrew");
			println!();
			cmd!(shell, "releasor --file-name spm-swift-package").run()?;
		}
		_ => {
			println!("Invalid option");
		}
	}

	Ok(())
}