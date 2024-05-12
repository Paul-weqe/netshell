use std::io::Result;
use clap::{Parser, Subcommand};
use pnet::datalink::interfaces;
use crate::{base::icmp, ifaces::draw_interface};


/// executes the command that has been run when on the operation mode 
pub(crate) fn execute(cli: OprCli) -> Result<()> {
    match cli.command {
        Some(command) => {
            match command {
                OprCommand::Ping { host } => {
                    icmp::ping(&host);
                },
                OprCommand::Show { item } => {
                    match item {
                        Some(i) => i.run(),

                        // by defaut the command will not show only one item, it will show multiple items  
                        None => ShowItems::default().run()
                    }
                }
            }
        }
        None => { }
    }
    Ok(())
}

#[derive(Parser)]
pub(crate) struct OprCli {
    #[command(subcommand)]
    command: Option<OprCommand>
}

#[derive(Subcommand)]
enum OprCommand {
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
    Interface {
        #[command(subcommand)]
        detailed: Option<InterfaceBrief>
    }
}

#[derive(Debug, Clone, Subcommand)]
enum InterfaceBrief { Brief }


impl ShowItems {
    fn run(&self) {
        match self {
            Self::Configuration => {
                println!("\nAll Configuration");
            }
            Self::Interface{ detailed } => {
                draw_interface(interfaces(), detailed.is_some());
            }
        }
    }
}
