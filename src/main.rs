
mod commands;
mod base;
mod routing;
mod ifaces;
mod config;
mod mode;

use mode::{Cli, Mode};


struct Storage {
    mode: mode::Mode
}

fn main() {

    let mut storage = Storage{mode: Mode::default()};

    loop {
        
        match storage.mode {
            
            Mode::Operation(op) => {
                let output = op.run();
                storage = Storage {
                    mode: output.nextmode
                };
            },

            Mode::Configuration(conf) => {
                let output = conf.run();
                storage = Storage {
                    mode: output.nextmode
                }
            }
            _ => {}

        }
    }
}