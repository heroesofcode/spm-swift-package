use clap::Command;
use colored::{Color, Colorize};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Header;

impl Header {
	pub fn show() -> String {
		Self::check_version();

		let orange = Color::TrueColor {
			r: 240,
			g: 81,
			b: 56,
		};

		format!(
			"\n{}\n\
             🚀 You can create your Swift Package via the command line 🔨\n\
             v{}\n",
			"SPM Swift Package".color(orange),
			VERSION
		)
	}

	fn check_version() {
		let _ = Command::new("spm-swift-package")
			.version(VERSION)
			.ignore_errors(true)
			.get_matches();
	}
}
