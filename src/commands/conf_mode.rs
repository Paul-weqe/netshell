
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct ConfCli {
    #[command(subcommand)]
    command: Option<CliCommand>
}

#[derive(Subcommand)]
enum CliCommand {

}