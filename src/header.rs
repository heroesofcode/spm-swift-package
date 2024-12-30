use clap::Command;
use colored::{Color, Colorize};

pub struct Header;

impl Header {

    pub fn show_header() -> String {
        Self::check_version();

        let header = "SPM Swift Package";
        let orange = Color::TrueColor { r: 240, g: 81, b: 56 };

        let header = format!(
            "\n{}\n\
             🚀 You can create your Swift Package via the command line 🔨\n\
             v0.2.0\n",
            header.color(orange)
        );

        header
    }

    fn check_version() {
        let _app = Command::new("spm-swift-package")
            .version("0.4.0")
            .ignore_errors(true)
            .get_matches();
    }
}