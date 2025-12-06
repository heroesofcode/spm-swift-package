use clap::Parser;

mod xtask_command;
use xtask_command::XtaskCommand;

#[derive(Parser)]
struct Args {
	#[command(subcommand)]
	command: Option<XtaskCommand>,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	if let Some(command) = args.command {
		command.execute()?;
	} else {
		XtaskCommand::help()?;
	}

	Ok(())
}
