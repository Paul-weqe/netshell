use netshell::{cli::{Cli, Mode}, Context};


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
