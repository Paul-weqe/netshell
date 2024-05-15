use clap::{Parser, Subcommand};

use super::ClappedOutput;


pub(crate) fn execute(input: EditConfInput) -> std::io::Result<ClappedOutput> {
    if let Some(command) = input.command {
        match command {
            EditConfCommand::Up => {
                return Ok(ClappedOutput::LevelDown)
            }
            EditConfCommand::Logout => {
                return Ok(ClappedOutput::Logout)
            }
        }
    }
    Ok(ClappedOutput::Completed)
}


#[derive(Parser)]
pub(crate) struct EditConfInput {
    #[command(subcommand)]
    command: Option<EditConfCommand>
}

#[derive(Subcommand)]
enum EditConfCommand {
    Up,
    Logout
}