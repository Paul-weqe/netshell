use std::io::{self, Write};
use crossterm::{event::{self, Event, KeyCode, KeyEvent}, terminal::{disable_raw_mode, enable_raw_mode}};

pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    enable_raw_mode()?;
    print!("# ");
    std::io::stdout().flush().unwrap();

    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        
        match code {

            KeyCode::Char('?') => {
                break;
            }
            KeyCode::Enter => {
                break;
            }
            KeyCode::Backspace => {
                line.pop();
                print!("\r# {line}");
                std::io::stdout().flush().unwrap();
            }
            KeyCode::Char(c) => {
                line.push(c);
                print!("\r# {line}");
                std::io::stdout().flush().unwrap();
            }
            _ => {}
        }

    }

    disable_raw_mode()?;

    Ok(line)
}

fn main() {
    println!("read line:");
    println!("{:?}", read_line());
}

/*
use std::{io::{self, Write}, thread, time};

fn main() {
    let start = time::Instant::now();
    let one_second = time::Duration::from_secs(1);
    let mut counter = 0;
    while counter <= 4 {
        thread::sleep(one_second);
        print!("\rticking: {:.0}s", start.elapsed().as_secs_f32());
        std::io::stdout().flush().unwrap();
        counter += 1;
    }
}

 */