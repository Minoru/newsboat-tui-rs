use ratatui::{backend::TermionBackend, Terminal};
use std::{error::Error, io};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
};

mod events_source;

use events_source::EventsSource;
use ui::{
    app::App,
    event::{Event, Key},
};

/// Setup a termion terminal with alternate screen enabled.
fn setup_termion_terminal(
) -> Result<Terminal<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = stdout.into_alternate_screen()?;

    let backend = TermionBackend::new(stdout);

    Terminal::new(backend)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_termion_terminal()?;

    let mut app: App<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>> = App::new();

    let events = EventsSource::new();
    loop {
        terminal.draw(|mut frame| app.draw(&mut frame))?;

        match events.next()? {
            Event::Key(key) => match key {
                Key::Ctrl(ref c) => match **c {
                    Key::Char('v') => app.cycle_to_next_formaction(),
                    Key::Char('g') => app.cycle_to_previous_formaction(),

                    _ => app.handle_key(key),
                },

                _ => app.handle_key(key),
            },

            Event::TerminalResized => {
                // Do nothing. We'll redraw the UI on the next iteration anyway.
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
