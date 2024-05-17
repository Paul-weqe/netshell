use clap::{Parser, Subcommand};
use pnet::datalink::interfaces;
use crate::{base, ifaces, Context};
use super::ClappedOutput;


pub(crate) fn execute(input: ConfInput, context: &mut Context) -> std::io::Result<ClappedOutput> {

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


            ConfCommand::Clear => {
                return Ok(ClappedOutput::ClearScreen)
            }

            // {username}# set [item]
            ConfCommand::Set { item } => {
                return execute_set_command(item, context)
            }

            ConfCommand::Show { item } => {
                match item {
                    ShowItem::Interfaces => {
                        ifaces::draw_interface(interfaces(), true);
                    }
                }
            }

        }
    }
    Ok(ClappedOutput::Completed)

}

fn execute_set_command(item: SetItem, context: &mut Context) -> std::io::Result<ClappedOutput>{
    match item {

        // config-mode# set system [param]
        SetItem::System { param } => {
            match param {

                // config-mode# set system host-name [hostname]
                System::HostName { hostname } => {
                    if base::sethostname(&hostname) >= 0 {
                        context.config.hostname = hostname.clone();
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
        item: ShowItem
    }, 
    Set {
        #[command(subcommand)]
        item: SetItem
    }
}


#[derive(Subcommand, Debug)]
enum SetItem {
    // set system [param]
    System {
        #[command(subcommand)]
        param: System
    }
}

#[derive(Subcommand, Debug)]
enum System {

    // set system host-name [host-name]
    HostName {
        hostname: String
    }
}


#[derive(Subcommand, Debug)]
enum ShowItem {
    // show interfaces
    Interfaces 
}