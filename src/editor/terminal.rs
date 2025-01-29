use crossterm::cursor::{MoveTo, Hide, Show, MoveRight, self};

use std::io::{self, stdout, Write};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{size, disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::Command;

// use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
//

pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16, // column
    pub y: u16, // line
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn initialize() -> io::Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{ x: 0, y: 0 })?;
        queue!(stdout(), Show)
    }

    pub fn terminate() -> io::Result<()> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> io::Result<()> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> io::Result<()> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn get_cursor_position() -> io::Result<Position> {
        let (x, y) = cursor::position()?;
        Ok(Position { x, y })
    }

    pub fn move_cursor_to(position: Position) -> io::Result<()> {
        Self::queue_command(MoveTo(position.x, position.y))
    }

    pub fn move_cursor_right(number: u16) -> io::Result<()> {
        Self::queue_command(MoveRight(number))
    }

    pub fn hide_cursor() -> io::Result<()> {
        Self::queue_command(Hide)
    }
    pub fn show_cursor() -> io::Result<()> {
        Self::queue_command(Show)
    }

    pub fn size() -> io::Result<Size> {
        let (width, height) = size()?;
        Ok(Size{ width, height })
    }

    pub fn print(string: &str) -> io::Result<()> {
        Self::queue_command(Print(string))
    }

    pub fn execute() -> io::Result<()> {
        stdout().flush()
    }

    fn queue_command<T: Command>(command: T) -> io::Result<()> {
        queue!(stdout(), command)
    }

}

impl Position {
    pub fn relative(&mut self, relative: Position) {
        self.x += relative.x;
        self.y += relative.y;
    }
}
