use crate::utils::theme_colors::*;
use clap::Command;
use colored::Colorize;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Header;

impl Header {
	pub fn show() -> String {
		Self::check_version();

		format!(
			"\n{}\n\
             🚀 You can create your Swift Package via the command line 🔨\n\
             v{}\n",
			"SPM Swift Package".color(ThemeColors::ORANGE_TERM),
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
