use easycurses;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CursesBackend},
    Frame, Terminal,
};

struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> App {
        App { should_quit: false }
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
        }
    }
}

fn draw<B: Backend>(_frame: &mut Frame<B>, _app: &mut App) {}

fn main() -> Result<(), Box<dyn Error>> {
    let mut backend = CursesBackend::new().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Failed to initialize curses backend",
    ))?;
    let curses = backend.get_curses_mut();

    curses.set_echo(false);
    curses.set_input_timeout(easycurses::TimeoutMode::WaitUpTo(60)); // milliseconds
    curses.set_input_mode(easycurses::InputMode::RawCharacter);
    curses.set_keypad_enabled(true);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new();

    loop {
        terminal.draw(|mut frame| draw(&mut frame, &mut app))?;

        match terminal.backend_mut().get_curses_mut().get_input() {
            // TODO: why not call `get_input()` on `curses`?
            Some(input) => match input {
                easycurses::Input::Character(c) => {
                    app.on_key(c);
                }

                _ => {}
            },

            _ => {}
        }

        terminal.backend_mut().get_curses_mut().flush_input();

        if app.should_quit {
            break;
        }
    }

    Ok(())
}