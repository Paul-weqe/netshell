
mod commands;
mod base;
mod routing;
mod ifaces;
mod config;
mod mode;

use mode::{Cli, Mode};


struct Context {
    mode: mode::Mode
}

pub(crate) struct Configuration {
    hostname: String
}

impl Default for Configuration {
    fn default() -> Self {
        Self { hostname: base::gethostname() }
    }
}

fn main() {

    let mut storage = Context{mode: Mode::default()};
    let mut config = Configuration::default();

    loop {
        
        match storage.mode {

            Mode::Operation(op) => {
                let output = op.run(&mut config);
                storage = Context {
                    mode: output.nextmode
                };
            },

            Mode::Configuration(conf) => {
                let output = conf.run(&mut config);
                storage = Context {
                    mode: output.nextmode
                }
            }

        }
    }
}