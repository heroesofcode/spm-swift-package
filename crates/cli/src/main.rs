mod cli;
mod core;
mod header;
mod ui;
mod utils;

use clap::Parser;
use cli::Args;

/// Entry point of the application
#[tokio::main]
async fn main() {
	let args = Args::parse();
	if let Err(e) = cli::run(args).await {
		eprintln!("Error: {e}");
		std::process::exit(1);
	}
}
