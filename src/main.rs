use std::io::{stdin, stdout, Write};
use clap::{CommandFactory, Error, FromArgMatches, Parser, Subcommand};
use commands::operation;
use crossterm::{event::{read, Event, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode}};

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
        let event = read().unwrap();

        match mode {
            
            CliMode::Opr => {
                let input = get_input(">");
                println!("{:#?}", input.unwrap());

            }
            CliMode::Conf =>  {
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


fn get_input(bash: &str) -> std::io::Result<Input> {
    
    let mut input: String = String::new();
    enable_raw_mode()?;

    let event = read()?;
    loop {
        println!("{:#?}", event);
        match event { 
            Event::Key(e) => {
                if e.code == KeyCode::Char('?') {
                    let mut args = vec![String::new()];
                    input.split(" ").into_iter().for_each(|a| args.push(a.trim().to_string()));
                    disable_raw_mode()?;
                    return Ok(Input::Query(args))
                }
                
                if let KeyCode::Char(c) = e.code {
                    input.push(c);
                }

                if e.code == KeyCode::Enter {
                    let mut args = vec![String::new()];
                    input.split(" ").into_iter().for_each(|a| args.push(a.trim().to_string()));
                    disable_raw_mode()?;
                    return Ok(Input::CompletedCommand(args))
                }
            

            }
            _ => {}
        }
    }

    disable_raw_mode()?;


}

#[derive(Debug)]
enum Input {
    CompletedCommand(Vec<String>),
    Query(Vec<String>)
}

