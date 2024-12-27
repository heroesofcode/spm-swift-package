use spm_swift_package::{Header, FieldsResults};

fn main() {
    let header = Header::show_header();
    println!("{}", header);

    FieldsResults::result();
}
