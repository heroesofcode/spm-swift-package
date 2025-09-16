use spm_swift_package::presentation::cli_controller::CliController;
use spm_swift_package::presentation::header::Header;

#[tokio::main]
async fn main() {
	let header = Header::show();
	println!("{}", header);

	let _ = CliController::execute_flow().await;
}
