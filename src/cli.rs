use clap::{CommandFactory, Error, FromArgMatches, Parser};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, 
    terminal::{self, disable_raw_mode, enable_raw_mode}, 
    ExecutableCommand
};
use std::{io::{self, stdout, Write}, process};
use crate::{
    commands::{
        configuration_cmd::{self, ConfInput}, 
        operational_cmd::{self, OprInput}, 
        ClappedOutput
    }, 
    modes::{ConfigMode, OperationMode}, Context
};

fn clear_screen() {
    let _ = stdout().execute(terminal::Clear(terminal::ClearType::All));
    let _ = stdout().execute(crossterm::cursor::MoveTo(0, 0));
}


pub trait Cli {
    /*
    Gets input from the user. 

    initial_cmd is used for when a user had wanted the command to be completed e.g when a user 
        has pressed '?' they expect to be given the options that come with that command 
    */
    fn get_input(&self, prompt: &str, initial_cmd: Option<Vec<String>>, context: &mut Context) -> io::Result<UserRequest> {
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

                // keycode is used for getting the previous command that has been input. 
                KeyCode::Up => {
                    let mut space = String::new();
                    if let Some(value) = context.history.up() {

                        // when we press the up button, sometimes we find the previous is longer 
                        // e.g if command 56 was `configure` and 57 was `show interfaces`
                        // and you are scrolling up, naturally if we are pressing the up button, we will 
                        // first be shown the 
                        if line.len() > value.len() {
                            (0..(line.len() - value.len())).for_each(|_| space = format!("{} ", space) );
                        }

                        line = value;
                        print!("\r{prompt} {line}{}\r{prompt} {line}", space);
                        std::io::stdout().flush().unwrap();
                    }
                }

                // keycode used for getting the next command in the history that 
                // have been input
                KeyCode::Down => {
                    let mut space = String::new();
                    if let Some(value) = context.history.down() {
                        // when we press the up button, sometimes we find the previous is longer 
                        // e.g if command 56 was `configure` and 57 was `show interfaces`
                        // and you are scrolling up, naturally if we are pressing the up button, we will 
                        // first be shown the 
                        if line.len() > value.len() {
                            (0..(line.len() - value.len())).for_each(|_| space = format!("{} ", space) );
                        }
                        
                        line = value;
                        print!("\r{prompt} {line}{}\r{prompt} {line}", space);
                        std::io::stdout().flush().unwrap();
                    }
                }

                // when CTRL +C is pressed
                KeyCode::Char('c') if control => {
                    print!("\r{prompt} {line}^C");
                    disable_raw_mode()?;
                    line.trim().split(' ').for_each(|a| args.push(String::from(a)));
                    return Ok(UserRequest::CanceledCommand(args))
                }

                // according the the hierarchy of modes, we have:
                // EditConfig > Config > Operation
                // when you press CTRL + D, you esentially go one level lower of where you were. 
                KeyCode::Char('d') if control => {
                    print!("\r{prompt} {line}^D");
                    disable_raw_mode()?;
                    line.trim().split(' ').for_each(|a| args.push(String::from(a)));
                    return Ok(UserRequest::LevelUpInput);
                }

                KeyCode::Char('l') if control => {
                    disable_raw_mode()?;
                    line.trim().split(' ').for_each(|a| args.push(String::from(a)));
                    return Ok(UserRequest::ClearScreen)
                }

                KeyCode::Char('?') => {
                    print!("\r{prompt} {line}?");
                    disable_raw_mode()?;
                    line.trim().split(' ').for_each(|a| args.push(String::from(a)));
                    return Ok(UserRequest::Query(args))
                }
                KeyCode::Enter => {
                    if !&line.trim().is_empty() { context.history.add(&line) };
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
        line.trim().split(' ').for_each(|a| args.push(String::from(a)));
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

    fn run(&self, context: &mut Context) -> io::Result<()>;
}


impl Cli for OperationMode {

    fn run(&self, context: &mut Context) -> io::Result<()> {
        if let Ok(user_request) = self.get_input(&self.prompt, None, context) {
            match user_request {

                UserRequest::CompletedCommand(args) => {

                    if let Ok(cli) = Self::netcli_parse::<OprInput>(&args) {

                        if let Ok(cmd_result) = operational_cmd::execute(cli, context){
                            match cmd_result {
                                ClappedOutput::LevelUp => {
                                    context.mode = self.level_up();
                                    return Ok(())
                                }
                                ClappedOutput::LevelDown => {
                                    context.mode = self.level_down();
                                    return Ok(())
                                }
                                ClappedOutput::Logout => {
                                    context.mode = self.logout();
                                    return Ok(())
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

                UserRequest::LevelUpInput => {
                    context.mode = self.level_up();
                    return Ok(())
                }

                UserRequest::ClearScreen => {
                    let _ = stdout().execute(terminal::Clear(terminal::ClearType::All));
                    let _ = stdout().execute(crossterm::cursor::MoveTo(0, 0));
                }

            }
        }
        context.mode = Mode::Operation( OperationMode::default() );
        return Ok(())
    }

}


impl Cli for ConfigMode {
    fn run(&self, context: &mut Context) -> io::Result<()> {

        let input = self.get_input(&self.prompt, None, context);
        let request = match input {
            Ok(user_request) => user_request,
            Err(_) => panic!("[config] No input")
        };

        match request {
            UserRequest::CompletedCommand(args) => {
                if let Ok(cli_input) = Self::netcli_parse::<ConfInput>(&args) {
                    if let Ok(output) = configuration_cmd::execute(cli_input, context) {
                        match output {
                            ClappedOutput::LevelDown => {
                                context.mode = self.level_down();
                                return Ok(())
                            },
                            ClappedOutput::LevelUp => {
                                context.mode = self.level_up();
                                return Ok(())
                            }
                            ClappedOutput::Logout => {
                                context.mode = self.logout();
                                return Ok(())
                            }
                            ClappedOutput::ClearScreen => {
                                clear_screen();
                            }
                            _=> {}
                        }
                    }
                } 
            }

            UserRequest::LevelUpInput => {
                context.mode = self.level_up();
                return Ok(())
            }

            UserRequest::ClearScreen => {
                clear_screen();
            }

            _ =>{}
        }
        context.mode = Mode::Configuration( ConfigMode::default() );
        return Ok(())
    }
}


///  `UserRequest` takes the input that the user has given (e.g show ip interface)
/// and categorizes the input. Apart from when the user presses `ENTER`, there 
/// are other user actions that are taken into consideration e.g `CRTL + C`, 
/// `CTRL + D` etc... 
#[derive(Debug)]
pub enum UserRequest {

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
    LevelUpInput, 

    // Clears the whole screen to get new input. 
    // when `CRTL + L` is pressed
    ClearScreen

}


/// Anytime we are on the network device terminal, we will be 
/// in one mode depending on what you are working on. 
#[derive(Clone)]
pub enum Mode{
    Operation(OperationMode),
    Configuration(ConfigMode)
}

impl Default for Mode {
    fn default() -> Self {
        Self::Operation(OperationMode::default())
    }
}

// state for Mode FSM
pub(crate) trait State {

    // in each of the methods, I is the input type and O is the output type
    fn logout(&self) -> Mode {
        std::process::exit(1);
    }

    // There are three modes: Operation Mode, Configuration Mode and Edit Configuration Mode
    // Operation is the highest Mode followed by COnfiguration followed by Edit Configuration Mode. 
    // The lower you go, the more specific you are as to what you are viewing and also the more power you have. 
    // level_up() takes you to a higher Mode and level_down() takes you to a lower mode.
    fn level_up(&self) -> Mode;
    fn level_down(&self) -> Mode;
}


impl State for OperationMode {

    fn level_up(&self) -> Mode {
        process::exit(1);
    }

    fn level_down(&self) -> Mode {
        Mode::Configuration(ConfigMode::default())
    }

}

impl State for ConfigMode {
    fn level_up(&self) -> Mode {
        Mode::Operation(OperationMode::default())
    }

    fn level_down(&self) -> Mode {
        Mode::Configuration(ConfigMode::default())
    }

}
