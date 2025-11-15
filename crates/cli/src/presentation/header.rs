use clap::Command;
use colored::{Color, Colorize};

pub struct Header;

impl Header {
    const VERSION: &'static str = "0.8.1";

    pub fn show() -> String {
        Self::check_version();

        let orange = Color::TrueColor { r: 240, g: 81, b: 56 };

        format!(
            "\n{}\n\
             🚀 You can create your Swift Package via the command line 🔨\n\
             v{}\n",
            "SPM Swift Package".color(orange),
            Self::VERSION
        )
    }

    fn check_version() {
        let _ = Command::new("spm-swift-package")
            .version(Self::VERSION)
            .ignore_errors(true)
            .get_matches();
    }
}
