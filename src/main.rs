
mod commands;
mod base;
mod routing;
mod ifaces;
mod config;

use std::io::{self, Write};
use clap::{CommandFactory, Error, FromArgMatches, Parser};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};


fn main() {
    let mode = CliMode::default();
    
    'outer: loop {
        let prompt: &str;

        match mode {
            CliMode::Opr => {
                prompt = ">";
                let input = get_input(&prompt, None);
                match input {
                    Ok(i) => {
                        match i {


                            Input::CompletedCommand(args) => {
                                match netcli_parse::<commands::opr_mode::OprCli>(&args) {
                                    Ok(cli) => {
                                        let _ = commands::opr_mode::execute(cli);
                                    }
                                    Err(_) => continue 'outer
                                }
                            },

                            // todo()
                            Input::Query(args) => {

                            }
                            Input::CanceledCommand(_) => {
                                break 'outer
                            }

                        }
                    },
                    Err(_) => {

                    }
                }
            }

            CliMode::Conf =>  {

            }
        }
    }
}

#[derive(Default)]
enum CliMode {
    // operation mode - only allows reading and viewing of certain configs
    #[default]
    Opr,    
    
    // configuration mode - allows editing of configs
    Conf
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


/*
Gets input from the user. 

initial_cmd is used for when a user had wanted the command to be completed e.g when a user 
    has pressed '?' they expect to be given the options that come with that command 
 */
fn get_input(prompt: &str, initial_cmd: Option<Vec<String>>) -> io::Result<Input> {
    let mut line = String::new();
    if let Some(cmd) = initial_cmd {
        line += &cmd.join(" ");
    }
    println!();

    enable_raw_mode()?;
    let mut args = vec![String::new()];
    print!("{prompt} ");
    std::io::stdout().flush().unwrap();

    while let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
        let control = modifiers.contains(KeyModifiers::CONTROL);
        match code {

            KeyCode::Char('c') if control => {
                print!("\r{prompt} {line}^C");
                disable_raw_mode()?;
                line.split(" ").into_iter().for_each(|a| args.push(String::from(a)));
                return Ok(Input::CanceledCommand(args))
            }

            KeyCode::Char('?') => {
                break;
            }
            KeyCode::Enter => {
                break;
            }
            KeyCode::Backspace => {
                line.pop();
                print!("\r{prompt} {line}{}\r{prompt} {line}", " ".repeat(1));
                std::io::stdout().flush().unwrap();
            }
            KeyCode::Char(c) => {
                line.push(c);
                print!("\r{prompt} {line}");
                std::io::stdout().flush().unwrap();
            }

            _ => {}

        }

    }

    disable_raw_mode()?;
    line.split(" ").into_iter().for_each(|a| args.push(String::from(a)));
    Ok(Input::CompletedCommand(args))
}


#[derive(Debug)]
enum Input {
    CompletedCommand(Vec<String>),
    Query(Vec<String>),
    CanceledCommand(Vec<String>)
}