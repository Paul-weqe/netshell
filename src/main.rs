mod commands;
mod base;
mod routing;
mod ifaces;
mod config;

use std::io::{self, Write};
use clap::{CommandFactory, Error, FromArgMatches, Parser};
use commands::opr_mode::CommandOutput;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};


fn main() {
    let mut mode = CliMode::default();
    let mut initial_cmd: Option<Vec<String>> = None;

    'outer: loop {
        let prompt: &str;

        match mode {

            // when we are in operations mode. 
            CliMode::Opr => {
                prompt = ">";
                let input = get_input(&prompt, initial_cmd.clone());
                initial_cmd = None;
                match input {
                    Ok(i) => {
                        match i {


                            Input::CompletedCommand(args) => {
                                match netcli_parse::<commands::opr_mode::OprCli>(&args) {
                                    Ok(cli) => {

                                        match commands::opr_mode::execute(cli) {
                                            Ok(cmd_result) => {
                                                match cmd_result {
                                                    CommandOutput::LevelUp => {
                                                        mode = CliMode::Conf;
                                                        continue 'outer
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                        
                                    }
                                    Err(_) => continue 'outer
                                }
                            },

                            // todo()
                            Input::Query(incomplete_cmd) => {
                                initial_cmd = Some(incomplete_cmd);
                                continue 'outer
                            }

                            // basically when one presses CTRL + C
                            Input::CanceledCommand(_) => {
                                continue 'outer
                            }

                            Input::LevelDownInput => {
                                break 'outer
                            }

                            Input::LevelUpInput => {

                            }

                        }
                    },
                    Err(_) => {
                        
                    }
                }
            }


            // if we are if configuration mode. 
            CliMode::Conf =>  {
                prompt = "#";
                let input_result = get_input(&prompt, initial_cmd.clone());

                match input_result {

                    Ok(input) => {
                        match input {
                            Input::LevelUpInput => {

                            }
                            Input::LevelDownInput => {
                                mode = CliMode::Opr;
                                continue 'outer
                            }
                            _ => {}
                        }
                    }

                    Err(_) => {}
                }
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
    print!("{prompt} {}", line.trim());
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

            KeyCode::Char('d') if control => {
                print!("\r{prompt} {line}^D");
                disable_raw_mode()?;
                line.split(" ").into_iter().for_each(|a| args.push(String::from(a)));
                return Ok(Input::LevelDownInput);
            }

            KeyCode::Char('?') => {
                print!("\r{prompt} {line}?");
                disable_raw_mode()?;
                line.split(" ").into_iter().for_each(|a| args.push(String::from(a)));
                return Ok(Input::Query(args))
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
    CanceledCommand(Vec<String>),
    LevelDownInput, 
    LevelUpInput
}