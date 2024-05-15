
use clap::{Parser, Subcommand};

use super::ClappedOutput;


pub(crate) fn execute(input: ConfInput) -> std::io::Result<ClappedOutput> {

    if let Some(command) = input.command {
        match command {
            ConfCommand::Up=> {
                return Ok(ClappedOutput::LevelDown)
            }
            ConfCommand::Edit => {
                return Ok(ClappedOutput::LevelUp)
            }
        }
    }
    Ok(ClappedOutput::Completed)

}

#[derive(Parser)]
pub(crate) struct ConfInput {
    #[command(subcommand)]
    command: Option<ConfCommand>
}

#[derive(Subcommand)]
enum ConfCommand {
    Up,
    Edit
}
