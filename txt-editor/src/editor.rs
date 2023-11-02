use crate::Terminal;
use termion::event::Key;
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: CursorPosition,
}

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
            cursor_position: CursorPosition { x: 0, y: 0 },
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&CursorPosition { x: 0, y: 0 });
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

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_row();
            if row == height / 3 {
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
            Key::Up | Key::Down | Key::Right | Key::Left => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let CursorPosition { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            _ => (),
        }
        self.cursor_position = CursorPosition { x, y }
    }
}

fn die(_e: std::io::Error) {
    Terminal::clear_screen();
    std::process::exit(1)
}
