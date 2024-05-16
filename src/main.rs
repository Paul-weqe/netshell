
mod commands;
mod base;
mod routing;
mod ifaces;
mod config;
mod cli;
mod modes;


use std::io::Error;

use cli::{Cli, Mode};


pub const DEFAULT_HISTORY_LOCATION: &str = "/root/.netsh_history";

#[derive(Default, Clone)]
struct Context {
    mode: cli::Mode, 
    config: Configuration,
    history: Vec<String>
}

#[derive(Clone)]
pub(crate) struct Configuration {
    hostname: String
}

impl Default for Configuration {
    fn default() -> Self {
        Self { hostname: base::gethostname() }
    }
}

#[tokio::main]
async fn main() {

    let mut context = Context::default();

    loop {
        
        match context.clone().mode {
            Mode::Operation(ref op) => {
                op.run(&mut context);
            },

            Mode::Configuration(ref conf) => {
                conf.run(&mut context);
            }

        }
    }
}