use spm_swift_package::presentation::header::Header;
use spm_swift_package::presentation::cli_controller::CliController;

#[tokio::main]
async fn main() {
    let header = Header::show();
    println!("{}", header);

    let _ = CliController::execute_flow().await;
}
