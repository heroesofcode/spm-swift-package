mod domain;
mod presentation;
mod ui;
mod utils;

use clap::{Parser, Subcommand};
use presentation::{CliController, Header};

/// Defines the CLI arguments accepted by the application
/// Uses Clap to support a subcommand-based interface
#[derive(Parser)]
struct Args {
	/// Optional subcommand that defines an alternative execution flow
	#[command(subcommand)]
	command: Option<Command>,
}

/// Represents the available CLI subcommands
/// UI triggers the graphical mode instead of the terminal flow
#[derive(Subcommand)]
enum Command {
	/// Runs the UI mode using the iced-based interface
	UI,
}

/// Entry point of the application
/// Uses Tokio runtime to support async operations in the CLI flow
#[tokio::main]
async fn main() {
	let args = Args::parse();

	if let Some(Command::UI) = args.command {
		// Runs the visual interface instead of the terminal builder
		let _ = ui::spm_view::run();
	} else {
		// Prints the header banner and runs the interactive CLI flow
		let header = Header::show();
		println!("{header}");

		let _ = CliController::execute_flow().await;
	}
}
