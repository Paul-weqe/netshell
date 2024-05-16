use std::{default, fs, io::Write};

use crate::DEFAULT_HISTORY_LOCATION;


struct History {
    location: String,
    commands: Vec<String>,
    idx: Option<usize>
}

impl History {
    fn new(mut location: Option<&str>) -> Self {

        let mut commands = Vec::new();
        if let None = location {
            location = Some(DEFAULT_HISTORY_LOCATION)
        }

        fs::read_to_string(location.unwrap())
            .expect("unable to open file")
            .split("\n")
            .into_iter()
            .for_each(|f| {
                if f.trim() != "" {
                    commands.push(f.to_string())
                }
            });

        Self {
            location: location.unwrap().to_string(),
            commands,
            idx: None
        }
    }

    fn up(&mut self) -> Option<String> {
        
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

    fn down(&mut self) -> Option<String> {
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

    fn add(&mut self, command: &str) {

        let mut d_file = std::fs::OpenOptions::new()
            .append(true)
            .open(&self.location)
            .expect("cannot open file");
        d_file.write(format!("{command}\n").as_bytes())
            .expect("write failed");
        self.commands.push(String::from(command));


        
    }
    
}


impl Default for History {
    fn default() -> Self {
        Self::new(None)
    }
}