mod content;
mod fields_results;
mod spm;
mod structure;

use clap::Command;
use colored::{Color, Colorize};
use fields_results::FieldsResults;

fn main() {
	check_version();

	let header = format!(
		r#"
   _____ ____  __  ___   _____         _ ______     ____             __                  
   / ___// __ \/  |/  /  / ___/      __(_) __/ /_   / __ \____ ______/ /______ _____ ____ 
   \__ \/ /_/ / /|_/ /   \__ \ | /| / / / /_/ __/  / /_/ / __ `/ ___/ //_/ __ `/ __ `/ _ \
  ___/ / ____/ /  / /   ___/ / |/ |/ / / __/ /_   / ____/ /_/ / /__/ ,< / /_/ / /_/ /  __/
 /____/_/   /_/  /_/   /____/|__/|__/_/_/  \__/  /_/    \__,_/\___/_/|_|\__,_/\__, /\___/ 
                                                                             /____/        (0.1.0)
     "#
	);

	let orange = Color::TrueColor {r: 240, g: 81, b: 56};
	println!("\n{}", header.color(orange));
	println!("🚀 You can create your Swift Package via the command line 🔨\n");

	FieldsResults::result();
}

fn check_version() {
	let _app = Command::new("spm-swift-package")
		.version("0.1.0")
		.ignore_errors(true)
		.get_matches();
}
