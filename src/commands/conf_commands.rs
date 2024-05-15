
use clap::{Parser, Subcommand};

use crate::{base, Configuration};

use super::ClappedOutput;


pub(crate) fn execute(input: ConfInput, config: &mut Configuration) -> std::io::Result<ClappedOutput> {

    if let Some(command) = input.command {
        match command {

            // {username}# up
            ConfCommand::Up=> {
                return Ok(ClappedOutput::LevelDown)
            }

            // {username}# edit
            ConfCommand::Edit => {
                return Ok(ClappedOutput::LevelUp)
            },


            ConfCommand::Clear => {
                return Ok(ClappedOutput::ClearScreen)
            }

            // {username}# set [item]
            ConfCommand::Set { item } => {
                return execute_set_command(item, config)
            }

        }
    }
    Ok(ClappedOutput::Completed)

}

fn execute_set_command(item: SetItem, config: &mut Configuration) -> std::io::Result<ClappedOutput>{
    match item {

        // config-mode# set system [param]
        SetItem::System { param } => {
            match param {

                // config-mode# set system host-name [hostname]
                System::HostName { hostname } => {
                    if base::sethostname(&hostname) >= 0 {
                        config.hostname = hostname.clone();
                        return Ok(ClappedOutput::Completed);
                    } 
                    else { 
                        return Ok(ClappedOutput::Completed);
                    }
                }
                
            }
        }
    }
}


#[derive(Parser)]
pub(crate) struct ConfInput {
    #[command(subcommand)]
    command: Option<ConfCommand>
}

#[derive(Subcommand, Debug)]
enum ConfCommand {
    Clear,
    Up,
    Edit,
    Set {
        #[command(subcommand)]
        item: SetItem
    }
}


#[derive(Subcommand, Debug)]
enum SetItem {
    // HostName {
    //     hostname: String
    // },
    System {
        #[command(subcommand)]
        param: System
    }
}

#[derive(Subcommand, Debug)]
enum System {
    HostName {
        hostname: String
    }
}
