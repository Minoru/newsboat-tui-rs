use std::{error::Error, io};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{backend::TermionBackend, Terminal};

mod app;
mod feed_list;
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

    let mut app = App::new();

    let input = InputReader::new();
    loop {
        terminal.draw(|mut frame| app.draw(&mut frame))?;

        app.handle_key(input.next()?);

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
