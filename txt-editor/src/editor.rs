use crate::Terminal;
use std::io::{self, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            } else {
                self.draw_rows();
                print!("{}", termion::cursor::Goto(1, 1));
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    //this is static method, lack of the &self indictor
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Goodbye. \r");
        }
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r");
        }
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().keys().next() {
            return key;
        }
    }
}
fn die(_e: std::io::Error) {
    print!("{}", termion::clear::All);
    std::process::exit(1)
}
