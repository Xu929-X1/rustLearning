use crate::Document;
use crate::Row;
use crate::Terminal;
use termion::event::Key;
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: CursorPosition,
    document: Document,
}
#[derive(Default)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            } else {
                self.draw_rows();
                Terminal::cursor_position(&self.cursor_position);
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
            cursor_position: CursorPosition::default(),
            document: Document::open(),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&CursorPosition::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_row();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("BO Editor -- version {}\r", VERSION);
        let terminal_width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = terminal_width.saturating_sub(len) / 2;
        let space = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", space, welcome_message);
        welcome_message.truncate(terminal_width);
        println!("{}\r", welcome_message);
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Right
            | Key::Left
            | Key::PageDown
            | Key::PageUp
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let CursorPosition { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1);
        let width = size.width.saturating_sub(1);
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x.saturating_add(1);
                }
            }
            Key::PageDown => y = height,
            Key::PageUp => y = 0,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = CursorPosition { x, y }
    }
}

fn die(_e: std::io::Error) {
    Terminal::clear_screen();
    std::process::exit(1)
}
