mod content;
mod fields_results;
mod spm;
mod structure;
mod header;

use fields_results::FieldsResults;
use crate::header::Header;

fn main() {
	Header::show_header();
	FieldsResults::result();
}
