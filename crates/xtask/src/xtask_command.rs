use clap::Subcommand;
use xshell::{Shell, cmd};

#[derive(Subcommand)]
pub enum XtaskCommand {
	Run,
	Test,
	Build,
	UI,
	Publish,
	PublishDryRun,
	PrepareHomebrew,
}

impl XtaskCommand {
	pub fn execute(self) -> anyhow::Result<()> {
		let shell = Shell::new()?;

		match self {
			XtaskCommand::Run => {
				cmd!(shell, "cargo run").run()?;
			}
			XtaskCommand::Test => {
				cmd!(shell, "cargo test").run()?;
			}
			XtaskCommand::Build => {
				cmd!(shell, "cargo build").run()?;
			}
			XtaskCommand::UI => {
				cmd!(shell, "cargo run -- ui").run()?;
			}
			XtaskCommand::Publish => {
				cmd!(shell, "cargo publish").run()?;
			}
			XtaskCommand::PublishDryRun => {
				cmd!(shell, "cargo publish --dry-run").run()?;
			}
			XtaskCommand::PrepareHomebrew => {
				cmd!(shell, "releasor --file-name spm-swift-package").run()?;
			}
		}

		Ok(())
	}

	pub fn help() -> anyhow::Result<()> {
		let shell = Shell::new()?;
		cmd!(shell, "cargo run -p xtask -- --help").run()?;
		Ok(())
	}
}
