use std::io::Result;
use clap::{Parser, Subcommand};
use pnet::datalink::interfaces;
use crate::{base::icmp, ifaces::draw_interface};
use super::ParsedOutput;

/// executes the command that has been run when on the operation mode 
pub(crate) fn execute(input: OprInput) -> Result<ParsedOutput> {

    match input.command {
        Some(command) => {
            match command {
                OprCommand::Configure => {
                    return Ok(ParsedOutput::LevelUp)
                }
                OprCommand::Ping { host } => {
                    icmp::ping(&host);
                },
                OprCommand::Show { item } => {
                    match item {
                        Some(i) => i.run(),
                        None => ShowItems::default().run()
                    }
                }
            }
        }
        None => {

        }
    }

    Ok(ParsedOutput::Completed)
}

#[derive(Parser)]
pub(crate) struct OprInput {
    #[command(subcommand)]
    command: Option<OprCommand>
}

#[derive(Subcommand)]
enum OprCommand {
    Configure,
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
