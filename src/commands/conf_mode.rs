
use clap::{Parser, Subcommand};

use super::ParsedOutput;


pub(crate) fn execute(input: ConfInput) -> std::io::Result<ParsedOutput> {

    if let Some(command) = input.command {
        match command {
            ConfCommand::Exit=> {
                return Ok(ParsedOutput::LevelDown)
            }
            ConfCommand::Edit => {
                return Ok(ParsedOutput::LevelUp)
            }
        }
    }
    Ok(ParsedOutput::Completed)

}

#[derive(Parser)]
pub(crate) struct ConfInput {
    #[command(subcommand)]
    command: Option<ConfCommand>
}

#[derive(Subcommand)]
enum ConfCommand {
    Exit,
    Edit
}
