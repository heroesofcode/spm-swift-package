use spm_swift_package::{Header, FieldsResults};

#[tokio::main]
async fn main() {
    let header = Header::show_header();
    println!("{}", header);

    FieldsResults::result().await;
}
