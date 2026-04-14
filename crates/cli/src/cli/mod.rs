pub mod cli_controller;
pub use cli_controller::Cli;

use crate::core::error::SpmError;
use clap::{Parser, Subcommand, ValueEnum};

/// Defines the CLI arguments accepted by the application
#[derive(Parser)]
pub struct Args {
	/// Optional subcommand that defines an alternative execution flow
	#[command(subcommand)]
	pub command: Option<Command>,
}

/// Represents the available CLI subcommands
#[derive(Subcommand)]
pub enum Command {
	/// Runs the UI mode using the iced-based interface
	UI,
	/// Generates a Swift Package non-interactively (useful for CI)
	Generate {
		/// Name of the library/package
		#[arg(short, long)]
		name: String,
		/// Target platform
		#[arg(short, long)]
		platform: Platform,
		/// Test framework to use
		#[arg(short, long, default_value = "xctest")]
		test_framework: TestFramework,
		/// Optional files to include (can be repeated)
		#[arg(short, long)]
		files: Vec<OptionalFile>,
		/// Open the generated package in Xcode after creation
		#[arg(long)]
		open_xcode: bool,
	},
}

/// Dispatches CLI arguments to the appropriate execution path
pub async fn run(args: Args) -> Result<(), SpmError> {
	match args.command {
		Some(Command::UI) => {
			let _ = crate::ui::spm_view::run();
			Ok(())
		}
		Some(Command::Generate {
			name,
			platform,
			test_framework,
			files,
			open_xcode,
		}) => {
			crate::core::spm_builder::SpmBuilder::new(&name)
				.platform(platform.as_str())
				.test_framework(test_framework.as_str())
				.files(files.iter().map(|f| f.as_str()))
				.build()?;

			println!("Package '{}' generated.", name);

			if open_xcode {
				crate::utils::xcode::open_xcode(&name)?;
			}

			Ok(())
		}
		None => {
			let header = crate::header::Header::show();
			println!("{header}");
			Cli::execute_flow().await
		}
	}
}

#[derive(Clone, ValueEnum)]
pub enum Platform {
	Ios,
	Macos,
	Tvos,
	Watchos,
	Visionos,
}

impl Platform {
	pub fn as_str(&self) -> &'static str {
		match self {
			Platform::Ios => "iOS",
			Platform::Macos => "macOS",
			Platform::Tvos => "tvOS",
			Platform::Watchos => "watchOS",
			Platform::Visionos => "visionOS",
		}
	}
}

#[derive(Clone, ValueEnum)]
pub enum TestFramework {
	Xctest,
	SwiftTesting,
}

impl TestFramework {
	pub fn as_str(&self) -> &'static str {
		match self {
			TestFramework::Xctest => "XCTest",
			TestFramework::SwiftTesting => "Swift Testing",
		}
	}
}

#[derive(Clone, ValueEnum)]
pub enum OptionalFile {
	Changelog,
	Readme,
	Spi,
	Swiftlint,
}

impl OptionalFile {
	pub fn as_str(&self) -> &'static str {
		match self {
			OptionalFile::Changelog => "Changelog",
			OptionalFile::Readme => "Readme",
			OptionalFile::Spi => "Swift Package Index",
			OptionalFile::Swiftlint => "SwiftLint",
		}
	}
}

impl AsRef<str> for OptionalFile {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
