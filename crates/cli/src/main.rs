mod domain;
mod presentation;
mod ui;

use clap::{Parser, Subcommand};
use presentation::cli_controller::CliController;
use presentation::header::Header;

#[derive(Parser)]
struct Args {
	#[command(subcommand)]
	command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
	UI,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	if let Some(Command::UI) = args.command {
		let _ = ui::spm_view::run();
	} else {
		let header = Header::show();
		println!("{header}");

		let _ = CliController::execute_flow().await;
	}
}
