use std::io::Error;
use std::io::{self, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::CursorPosition;
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: termion::raw::RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    //getter for size
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All)
    }

    pub fn cursor_position(position: &CursorPosition) {
        let CursorPosition { mut x, mut y } = position;
        //prevent overflow
        x = x.saturating_add(1) as u16;
        y = y.saturating_add(1) as u16;

        println!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_row() {
        print!("{}", termion::clear::CurrentLine);
    }
}
