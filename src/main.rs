use std::{io::{stdin, stdout, Write}};
use clap::{CommandFactory, Error, FromArgMatches, Parser, Subcommand};

// type NetResult<T> = result::Result<T, Error>;

#[derive(Parser)]
struct OptCli {
    #[command(subcommand)]
    command: Option<OptCommand>
}

#[derive(Parser)]
struct ConfCli {
    #[command(subcommand)]
    command: Option<ConfCommand>
 }


#[derive(Subcommand)]
enum OptCommand {
    Ping {
        host: String
    }
}

#[derive(Subcommand)]
enum ConfCommand {
}

fn netcli_parse<P>(input: &[String]) -> Result<P, Error>  where P: Parser  {
    let mut matches = match <P as CommandFactory>::command().try_get_matches_from_mut(input.clone()){
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


fn main() {
    let mode = CliMode::default();

    'outer: loop {
        match mode {
            
            CliMode::Opr => {
                let input = get_input(">");

                let cli = match netcli_parse::<OptCli>(&input){
                    Ok(cli) => cli,
                    Err(_) => {
                        println!("invalid command: {:?}", input);
                        continue 'outer
                    }
                };
                
            }
            CliMode::Conf =>  {
                let mut input = get_input("#");

                let cli = match netcli_parse::<ConfCli>(&input){
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


#[derive(Default)]
/// this is the mode that the shell will be running in. 
/// When running in Operation mode, the configs will be read only
/// When running on Configuration mode, we will be able to edit the configuration
enum CliMode {
    // operation mode
    #[default]
    Opr,
    
    // configuration mode
    Conf
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