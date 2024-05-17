
use std::{fs::{self, File}, io::Write, path::Path};
use tabled::{builder::Builder, settings::{object::Rows, Alignment, Style}};
use crate::DEFAULT_HISTORY_LOCATION;


#[derive(Clone)]
pub(crate) struct History {
    location: String,
    commands: Vec<String>,
    idx: Option<usize>
}

/// used for accessing the history of all the commands that have been running
/// Up() used for accessing the previous command and down() used for 
/// accessing the next command. 
impl History {
    fn new(location: Option<&str>) -> Self {

        let mut commands = Vec::new();

        let file = match location {
            Some(loc) => loc,
            None => DEFAULT_HISTORY_LOCATION
        };

        if !Path::new(file).exists() {
            let _ = File::create(file);
        }

        fs::read_to_string(file)
            .expect("unable to open file")
            .split("\n")
            .into_iter()
            .for_each(|f| {
                if f.trim() != "" {
                    commands.push(f.to_string())
                }
            });


        Self {
            location: file.to_string(),
            commands,
            idx: None
        }
    }

    pub(crate) fn up(&mut self) -> Option<String> {
        
        let idx = match self.idx {
            Some(idx) => {
                if idx > 0 {
                    idx - 1
                } else {
                    0
                }
            },
            None => {
                if self.commands.is_empty() {
                    return None
                }
                self.commands.len() - 1
            }
        };
        self.idx = Some(idx);
        let command = self.commands.get(idx).unwrap();
        Some(command.clone())
    }

    pub(crate) fn down(&mut self) -> Option<String> {
        let idx = match self.idx {
            Some(idx) => {
                if idx as usize == self.commands.len() - 1 {
                    self.idx = None;
                    return None
                } else {
                    (idx as usize) + 1
                }
            } 
            None => {
                return None
            }
        };
        self.idx = Some(idx);
        let command = self.commands.get(idx).unwrap();
        Some(command.clone())
    }

    pub(crate) fn add(&mut self, command: &str) {

        let mut d_file = std::fs::OpenOptions::new()
            .append(true)
            .open(&self.location)
            .expect("cannot open file");

        d_file.write(format!("{command}\n").as_bytes())
            .expect("write failed");
        self.commands.push(String::from(command));
        self.idx = Some(self.commands.len());
        
    }

    pub(crate) fn pretty_print(&self) {

        let mut builder = Builder::default();
        let header = vec!["command"];
        builder.push_record(header);

        self.commands.iter().for_each(|cmd| {
            let mut row = vec![];
            let cmd = cmd.clone();
            row.push(&cmd);
            builder.push_record(row);
        });
        let table = builder.build()
            .with(Style::rounded())
            .modify(Rows::new(1..), Alignment::left())
            .to_string();
        println!("\nHistory of Commands\n{table}");
    }
    
}


impl Default for History {
    fn default() -> Self {
        Self::new(None)
    }
}