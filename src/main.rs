
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

fn main() {

    let mut storage = Context{mode: Mode::default()};

    loop {
        
        match storage.mode {

            Mode::Operation(op) => {
                let output = op.run();
                storage = Context {
                    mode: output.nextmode
                };
            },

            Mode::Configuration(conf) => {
                let output = conf.run();
                storage = Context {
                    mode: output.nextmode
                }
            }

            Mode::EditConfiguration(conf) => {
                let output = conf.run();
                storage = Context {
                    mode: output.nextmode
                }
            }

        }
    }
}