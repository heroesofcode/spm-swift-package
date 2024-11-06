use clap::Command;
use colored::{Color, Colorize};

pub struct Header;

impl Header {

    pub fn show_header() {
        Self::check_version();

        let header = "SPM Swift Package";

        let orange = Color::TrueColor {r: 240, g: 81, b: 56};
        println!("\n{}", header.color(orange));
        println!("🚀You can create your Swift Package via the command line 🔨");
        println!("v0.2.0\n");
    }

    fn check_version() {
        let _app = Command::new("spm-swift-package")
            .version("0.2.0")
            .ignore_errors(true)
            .get_matches();
    }
}