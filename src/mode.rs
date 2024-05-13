use clap::{CommandFactory, Error, FromArgMatches, Parser};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};
use std::{io::{self, Write}, process};

use crate::commands::{self, ParsedOutput, conf_mode::ConfInput, opr_mode::OprInput};

pub(crate) struct OperationMode { 
    pub(crate) prompt: String 
}

pub(crate) struct ConfigMode { 
    pub(crate) prompt: String 
}

pub(crate) struct EditConfigMode { 
    pub(crate) prompt: String 
}

pub(crate) trait Cli {
    /*
    Gets input from the user. 

    initial_cmd is used for when a user had wanted the command to be completed e.g when a user 
        has pressed '?' they expect to be given the options that come with that command 
    */
    fn get_input(&self, prompt: &str, initial_cmd: Option<Vec<String>>) -> io::Result<UserRequest> {
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
                    line.split(' ').for_each(|a| args.push(String::from(a)));
                    return Ok(UserRequest::CanceledCommand(args))
                }

                // according the the hierarchy of modes, we have:
                // EditConfig > Config > Operation
                // when you press CTRL + D, you esentially go one level lower of where you were. 
                KeyCode::Char('d') if control => {
                    print!("\r{prompt} {line}^D");
                    disable_raw_mode()?;
                    line.split(' ').for_each(|a| args.push(String::from(a)));
                    return Ok(UserRequest::LevelDownInput);
                }

                KeyCode::Char('?') => {
                    print!("\r{prompt} {line}?");
                    disable_raw_mode()?;
                    line.split(' ').for_each(|a| args.push(String::from(a)));
                    return Ok(UserRequest::Query(args))
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Backspace => {
                    line.pop();
                    print!("\r{prompt} {line}{}\r{prompt} {line}", " ".to_string());
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
        line.split(' ').for_each(|a| args.push(String::from(a)));
        Ok(UserRequest::CompletedCommand(args))
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

    fn run(&self) -> CliOutput;
}

impl Cli for OperationMode {

    fn run(&self) -> CliOutput {
        if let Ok(user_request) = self.get_input(&self.prompt, None) {
            match user_request {

                UserRequest::CompletedCommand(args) => {

                    if let Ok(cli) = Self::netcli_parse::<OprInput>(&args) {
                        if let Ok(cmd_result) = commands::opr_mode::execute(cli){
                            match cmd_result {
                                ParsedOutput::LevelUp => {
                                    return CliOutput {
                                        nextmode: self.level_up()
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                },

                // todo()
                UserRequest::Query(_) => {

                }

                // basically when one presses CTRL + C
                UserRequest::CanceledCommand(_) => {
                    
                }

                UserRequest::LevelDownInput => {
                    return CliOutput {
                        nextmode: self.level_down()
                    }
                }

                UserRequest::LevelUpInput => {
                    return CliOutput {
                        nextmode: self.level_up()
                    }
                }

            }
        }

        CliOutput { 
            nextmode: Mode::Operation(
                OperationMode { prompt: String::from(">") }
            )
        }
    }

}


impl Cli for ConfigMode {
    fn run(&self) -> CliOutput {
        let input = self.get_input(&self.prompt, None);
        let request = match input {
            Ok(user_request) => user_request,
            Err(_) => panic!("No input")
        };
        match request {
            UserRequest::CompletedCommand(args) => {
                if let Ok(cli_input) = Self::netcli_parse::<ConfInput>(&args) {
                    if let Ok(output) = commands::conf_mode::execute(cli_input) {
                        match output {
                            ParsedOutput::LevelDown => {
                                return CliOutput {
                                    nextmode: self.level_down()
                                }
                            },
                            ParsedOutput::LevelUp => {
                                return CliOutput {
                                    nextmode: self.level_up()
                                }
                            }
                            _=> {}
                        }
                    }
                }
            }

            UserRequest::LevelDownInput => {
                return CliOutput {
                    nextmode: self.level_down()
                }
            }

            UserRequest::LevelUpInput => {
                return CliOutput {
                    nextmode: self.level_up()
                }
            }

            _ =>{}
        }
        CliOutput {
            nextmode: Mode::Configuration(
                ConfigMode { prompt: String::from("#") }
            )
        }
    }
}


pub(crate) struct CliOutput {
    pub(crate) nextmode: Mode
}

#[derive(Debug)]
pub(crate) enum UserRequest {
    CompletedCommand(Vec<String>),
    Query(Vec<String>),
    CanceledCommand(Vec<String>),
    LevelDownInput, 
    LevelUpInput
}

pub(crate) enum Mode{
    Operation(OperationMode),
    Configuration(ConfigMode),
    EditConfiguration(EditConfigMode)
}

impl Default for Mode {
    fn default() -> Self {
        Self::Operation(
            OperationMode{ prompt: String::from(">") }
        )
    }
}

// state for Mode FSM
pub(crate) trait State {

    // in each of the methods, I is the input type and O is the output type
    fn logout(&self) -> Mode {
        std::process::exit(1);
    }
    fn level_up(&self) -> Mode;
    fn level_down(&self) -> Mode;
}

trait Cool {
    fn cool<I>(&self) -> I where I: State;
}

impl State for OperationMode {
    fn level_up(&self) -> Mode {
        Mode::Configuration(
            ConfigMode{ prompt: String::from("#") }
        )
    }

    fn level_down(&self) -> Mode {
        process::exit(1);
    }

}

impl State for ConfigMode {
    fn level_up(&self) -> Mode {
        Mode::EditConfiguration(
            EditConfigMode { prompt: String::from("edit-config#") }
        )
    }

    fn level_down(&self) -> Mode{
        Mode::Operation(
            OperationMode { prompt: String::from(">") }
        )
    }

}


impl State for EditConfigMode {
    fn level_up(&self) -> Mode {
        Mode::EditConfiguration(
            EditConfigMode { prompt: String::from("edit-config#") }
        )
    }

    fn level_down(&self) -> Mode {
        Mode::Configuration(
            ConfigMode { prompt: String::from("#") }
        )
    }
}