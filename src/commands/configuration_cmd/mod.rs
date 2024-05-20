mod set;
mod show;

use clap::{Parser, Subcommand};
use super::ClappedOutput;


pub(crate) fn execute(input: ConfInput, context: &mut crate::Context) -> std::io::Result<ClappedOutput> {

    if let Some(command) = input.command {
        match command {

            // {username}# up
            ConfCommand::Up | ConfCommand::Exit => {
                return Ok(ClappedOutput::LevelUp)
            }

            // {username}# edit
            ConfCommand::Edit => {
                return Ok(ClappedOutput::LevelDown)
            }

            ConfCommand::History => {
                context.history.pretty_print();
                return Ok(ClappedOutput::Completed)
            }

            // clear
            ConfCommand::Clear => {
                return Ok(ClappedOutput::ClearScreen)
            }

            // set [item]
            ConfCommand::Set { item } => {
                return set::execute(item, context)
            }

            // show [item]
            ConfCommand::Show { item } => {
                return show::execute(item, context)
            }
        }
    }
    Ok(ClappedOutput::Completed)

}



#[derive(Parser, Debug)]
pub(crate) struct ConfInput {
    #[command(subcommand)]
    command: Option<ConfCommand>
}

#[derive(Subcommand, Debug)]
enum ConfCommand {
    Clear,
    Up,
    History,
    Exit,
    Edit,
    Show {
        #[command(subcommand)]
        item: show::ShowItem
    }, 
    Set {
        #[command(subcommand)]
        item: set::SetItem
    }
}

