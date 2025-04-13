use spm_swift_package::presentation::header::Header;
use spm_swift_package::presentation::fields_results::FieldsResults;

#[tokio::main]
async fn main() {
    let header = Header::show_header();
    println!("{}", header);

    FieldsResults::result().await;
}
