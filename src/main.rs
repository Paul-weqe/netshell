use std::io::{stdin, stdout, Write};
use clap::{CommandFactory, Error, FromArgMatches, Parser, Subcommand};
use commands::operation;

mod base;


#[derive(Default)]
enum CliMode {
    // operation mode - only allows reading and viewing of certain configs
    #[default]
    Opr,    
    // configuration mode - allows editing of configs
    Conf
}


fn main() {
    let mode = CliMode::default();

    'outer: loop {
        match mode {
            
            CliMode::Opr => {
                let input = get_input(">");

                let _ = match netcli_parse::<commands::operation::OptCli>(&input){
                    Ok(cli) => {
                        operation::execute(cli)
                    },
                    Err(_) => {
                        println!("invalid command: {:?}", input);
                        continue 'outer
                    }
                };

            }
            CliMode::Conf =>  {
                let input = get_input("#");

                match netcli_parse::<commands::configuration::ConfCli>(&input){
                    Ok(cli) => cli,
                    Err(_) => {
                        
                        println!("invalid command: {:?}", input.as_slice());
                        continue 'outer
                    }
                };
            }
        }
    }
}

mod commands {
    pub(crate) mod operation {
        use std::io::Result;

        // holds and executes commands when on operation mode
        use clap::{Parser, Subcommand};

        use crate::base::icmp;

        #[derive(Parser)]
        pub(crate) struct OptCli {
            #[command(subcommand)]
            command: Option<OptCommand>
        }

        #[derive(Subcommand)]
        enum OptCommand {
            Ping {
                host: String
            }
        }

        pub(crate) fn execute(cli: OptCli) -> Result<()> {

            match cli.command {
                Some(command) => {
                    match command {
                        OptCommand::Ping { host } => {
                            icmp::ping(&host);
                            Ok(())
                        }
                    }
                }
                None => Ok(())
            }
        }


    }

    // todo
    pub(crate) mod configuration {  
        use clap::{Parser, Subcommand};

        #[derive(Parser)]
        pub(crate) struct ConfCli {
            #[command(subcommand)]
            command: Option<CliCommand>
        }

        #[derive(Subcommand)]
        enum CliCommand {

        }

        
    }
}

fn netcli_parse<P>(input: &[String]) -> Result<P, Error>  where P: Parser  {
    let mut matches = match <P as CommandFactory>::command().try_get_matches_from_mut(input){
        Ok(m) => {
            m
        }
        Err(err) => return Err(err)
    };
    let res = match <P as FromArgMatches>::from_arg_matches_mut(&mut matches) {
        Ok(res) => res,
        Err(err) => return Err(err)
    };
    Ok(res)
}






fn get_input(bash: &str) -> Vec<String> {

    let mut input: String = String::new();

    print!("{bash} ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Unable to read command");
    let mut args = vec![String::new()];
    input.split(" ").into_iter().for_each(|a| args.push(a.trim().to_string()));
    args
}

