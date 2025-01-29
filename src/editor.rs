// {{{ mod and use
mod terminal;

use core::cmp::{min, max};
use std::io::{self};
use crossterm::event::{read, Event, Event::Key, KeyCode::{self, Char}, KeyEvent, KeyModifiers};

use terminal::{Terminal, Position};
// }}}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    cursor_location: Location,
    buffer: Buffer,
}

pub struct Buffer {
    content: Vec<String>,
}
// {{{
impl Buffer {
    pub fn render(&self) -> io::Result<()> {
        for line in self.content.clone() {
            Terminal::print(&line)?;
            Terminal::print("\r\n")?;
            Terminal::move_cursor_right(1)?;

        }
        Ok(())
    }
}
// }}}

// {{{ Location & impl
struct Location {
    line: usize,
    column: usize,
}
impl Location {
    fn to_position(&self) -> Position {
        Position{ x: self.column as u16, y: self.line as u16 }
    }
}
// }}}

impl Editor {
    // {{{ default
    pub fn default() -> Self {
        // Editor{ should_quit: false }
        Editor { should_quit: false, cursor_location: Location { line: 0, column: 1 }, buffer: Buffer
            { content: vec![String::from("sdaf"), String::from("line 2"), String::from("line 3")] }} // use 1 or 0 as the first?
    }
    // }}}

    // {{{ run and repl
    pub fn run(&mut self) -> io::Result<()> {
        Terminal::initialize()?;
        let result = self.repl();
        Terminal::terminate()?;

        result
    }

    pub fn repl(&mut self) -> io::Result<()> {
        loop {
            self.refresh_screen()?;
            // 下面两部分都是if should quit, 感觉可以合并
            // 非也，evaluate可以包含更多内容
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate(&event)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, event: &Event) -> io::Result<()> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            // self.move_cursor_to(Position{ x:1, y:0 })?;
            // Terminal::print(&format!("print!{code:?}"))?;
            // dbg!(code, modifiers, kind, state);
            // print!("{code:?}");
            let Location { mut line, mut column } = self.cursor_location;
            match *modifiers {
                KeyModifiers::CONTROL => {
                    if Char('q') == *code {
                        self.should_quit = true;
                    }
                },
                KeyModifiers::NONE => {
                    match *code {
                        KeyCode::Down => {
                            line = min(line+1, Terminal::size()?.height.saturating_sub(1) as usize);
                            self.cursor_location = Location { line, column };
                        },
                        KeyCode::Up => {
                            line = line.saturating_sub(1);
                            self.cursor_location = Location { line, column };
                        },
                        KeyCode::Right => {
                            column = min(column+1, Terminal::size()?.width.saturating_sub(1) as usize);
                            self.cursor_location = Location { line, column };
                        },
                        KeyCode::Left => {
                            column = max(1, column.saturating_sub(1));
                            self.cursor_location = Location { line, column };
                        },
                        Char(char) => {
                            Terminal::print(&char.to_string())?;
                            column = min(column+1, Terminal::size()?.width.saturating_sub(1) as usize);
                            self.cursor_location = Location { line, column };
                        }
                        KeyCode::Enter => {
                            line = min(line+1, Terminal::size()?.height.saturating_sub(1) as usize);
                            self.cursor_location = Location { line, column: 1 };
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
            // if Char(';') == *code && KeyModifiers::NONE == *modifiers {
                // execute!(stdout(), MoveRight(1))?;
            // }
        }
        Ok(())
    }
    //}}}

    // {{{ draw and refresh

    // Should I draw_buffer every time?

    fn draw_rows() -> io::Result<()> {
        let height = Terminal::size()?.height;
        for current_row in 0..height {
            Terminal::move_cursor_to(Position{ x:0, y:current_row })?;
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
        Ok(())
    }

    fn draw_welcome_message() -> io::Result<()> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding-1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;

        Ok(())
    }

    fn draw_empty_row() -> io::Result<()> {
        Terminal::print("~")
    }

    pub fn refresh_screen(& mut self) -> io::Result<()> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x:1,y:0})?;
            self.buffer.render()?;
            Terminal::move_cursor_to(self.cursor_location.to_position())?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    //}}}

    // {{{ cursor
    // fn move_cursor_to(&mut self, position: Position) -> io::Result<()> {
    //     Terminal::move_cursor_to(position)?;
    //     self.cursor_location = position;
    //     Ok(())
    // }

    // }}}
}
