
mod commands;
mod base;
mod routing;
mod ifaces;
mod config;
mod cli;
mod modes;

use cli::{Cli, Mode};
use base::history::History;


pub const DEFAULT_HISTORY_LOCATION: &str = "/root/.netsh_history";

#[derive(Default, Clone)]
struct Context {
    mode: cli::Mode, 
    config: Configuration,
    history: History
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
                let _ = op.run(&mut context);
            },

            Mode::Configuration(ref conf) => {
                let _ = conf.run(&mut context);
            }

        }
    }
}