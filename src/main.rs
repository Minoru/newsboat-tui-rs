use std::{error::Error, io};
use termion::{
    event::Key,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{backend::TermionBackend, Terminal};

mod app;
mod feedlist;
mod input_reader;
mod stateful_list;

use app::App;
use input_reader::InputReader;

/// Setup a termion terminal with alternate screen enabled.
fn setup_termion_terminal(
) -> Result<Terminal<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);

    let backend = TermionBackend::new(stdout);

    Terminal::new(backend)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_termion_terminal()?;
    terminal.hide_cursor()?;

    let mut app = App::default();

    let input = InputReader::new();
    loop {
        terminal.draw(|mut frame| feedlist::draw(&mut frame, &mut app))?;

        match input.next()? {
            Key::Char(c) => app.on_key(c),

            Key::Up => app.feeds.previous(),

            Key::Down => app.feeds.next(),

            _ => {}
        };

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
