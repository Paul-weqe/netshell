use clap::{CommandFactory, Error, FromArgMatches, Parser};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};
use std::{io::{self, Write}, process};

use crate::commands::{self, conf_commands::ConfInput, edit_conf_commands::EditConfInput, opr_commands::OprInput, ClappedOutput};

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
                        if let Ok(cmd_result) = commands::opr_commands::execute(cli){
                            match cmd_result {
                                ClappedOutput::LevelUp => {
                                    return CliOutput {
                                        nextmode: self.level_up()
                                    }
                                }
                                ClappedOutput::Logout => {
                                    return CliOutput {
                                        nextmode: self.logout()
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
            Err(_) => panic!("[config] No input")
        };
        match request {
            UserRequest::CompletedCommand(args) => {
                if let Ok(cli_input) = Self::netcli_parse::<ConfInput>(&args) {
                    if let Ok(output) = commands::conf_commands::execute(cli_input) {
                        match output {
                            ClappedOutput::LevelDown => {
                                return CliOutput {
                                    nextmode: self.level_down()
                                }
                            },
                            ClappedOutput::LevelUp => {
                                return CliOutput {
                                    nextmode: self.level_up()
                                }
                            }
                            ClappedOutput::Logout => {
                                return CliOutput {
                                    nextmode: self.logout()
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


impl Cli for EditConfigMode {
    fn run(&self) -> CliOutput {
        let input = self.get_input(&self.prompt, None);
        let request = match input {
            Ok(user_request) => user_request,
            Err(_) => panic!("[edit-config] No input")
        };
        match request {
            UserRequest::CompletedCommand(args) => {

                if let Ok(cli_input) = Self::netcli_parse::<EditConfInput>(&args) {
                    if let Ok(output) = commands::edit_conf_commands::execute(cli_input) {
                        match output{
                            ClappedOutput::LevelDown => {
                                return CliOutput {
                                    nextmode: self.level_down()
                                }
                            }
                            ClappedOutput::LevelUp => {
                                return CliOutput {
                                    nextmode: self.level_up()
                                }
                            }
                            ClappedOutput::Logout => {
                                return CliOutput {
                                    nextmode: self.logout()
                                }
                            }
                            _ => {}
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

            _ => {}

        };
        return CliOutput {
            nextmode: Mode::EditConfiguration(
                EditConfigMode { prompt: String::from("edit-config# ") }
            ) 
        }
    }
}

pub(crate) struct CliOutput {
    pub(crate) nextmode: Mode
}


///
///  `UserRequest` takes the input that the user has given (e.g show ip interface)
/// and categorizes the input. Apart from when the user presses `ENTER`, there 
/// are other user actions that are taken into consideration e.g `CRTL + C`, 
/// `CTRL + D` etc... 
/// 
#[derive(Debug)]
pub(crate) enum UserRequest {

    /// when a user is sure that is the command they would 
    /// like to run. Happens when a user presses 'ETER' or 'RETURN'
    /// (whichever your keyboard uses)
    CompletedCommand(Vec<String>),

    /// when a user is asking for a completion of 
    /// the command or when they ask for the possibilities
    /// usually when '?' is pressed when running a command
    Query(Vec<String>),

    /// this happens when a user presses `CTRL + C``
    CanceledCommand(Vec<String>),

    /// when the mode is changed from a higher mode
    /// to a lower mode e.g from a edit configuration 
    /// mode to configuration mode. Mostly happens when `CRTL + D`
    LevelDownInput, 

    /// when someone is changing to a higher level mode
    /// e.g from operation mode to configuration mode. 
    LevelUpInput
}


/// Anytime we are on the network device terminal, we will be 
/// in one mode depending on what you are working on. 
/// 
/// 
/// 
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