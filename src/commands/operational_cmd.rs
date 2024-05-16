use std::io::Result;
use clap::{Parser, Subcommand};
use pnet::datalink::interfaces;
use crate::{base::icmp, ifaces::draw_interface};
use super::ClappedOutput;

/// executes the command that has been run when on the operation mode 
pub(crate) fn execute(input: OprInput) -> Result<ClappedOutput> {
    
    if let Some(command) = input.command {
        match command {
            
            OprCommand::Configure | OprCommand::Edit => {
                println!("\nEntering configuration mode");
                return Ok(ClappedOutput::LevelDown)
            }

            OprCommand::Ping { host } => {
                icmp::ping(&host);
            }

            OprCommand::Show { item } => {
                match item {
                    Some(i) => i.run(),
                    None => ShowItems::default().run()
                }
            }

            OprCommand::Clear => {
                return Ok(ClappedOutput::ClearScreen)
            }
        }
    }

    Ok(ClappedOutput::Completed)
}

// this is what will handle the input from the user end, into clap. 
// should pass whatever input it gets to OprCommand
#[derive(Parser)]
pub(crate) struct OprInput {
    #[command(subcommand)]
    command: Option<OprCommand>
}


// OprCommand contains all the current commands that can be 
// run by the user while on the terminal. 
#[derive(Subcommand)]
enum OprCommand {
    Clear,
    Configure,
    Edit,
    Ping {
        host: String
    },
    Show {
        #[command(subcommand)]
        item: Option<ShowItems>
    }
}


#[derive(Default, Clone, Subcommand)]
enum ShowItems {
    #[default]
    Configuration,
    Interfaces {
        #[command(subcommand)]
        interface: Option<ShowInterface>
    }
}

#[derive(Debug, Clone, Subcommand)]
enum ShowInterface { Brief }


impl ShowItems {
    fn run(&self) {
        match self {
            Self::Configuration => {
                println!("\nAll Configuration");
            }
            Self::Interfaces{ interface: detailed } => {
                draw_interface(interfaces(), detailed.is_some());
            }
        }
    }
}
