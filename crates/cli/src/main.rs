mod domain;
mod presentation;

use presentation::cli_controller::CliController;
use presentation::header::Header;

#[tokio::main]
async fn main() {
	let header = Header::show();
	println!("{header}");

	let _ = CliController::execute_flow().await;
}