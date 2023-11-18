use ratatui::Terminal;
use std::{error::Error, io};

#[cfg(feature = "crossterm")]
use ratatui::backend::CrosstermBackend;
#[cfg(feature = "termion")]
use ratatui::backend::TermionBackend;

#[cfg(feature = "crossterm")]
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(feature = "termion")]
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
#[cfg(feature = "termion")]
fn setup_termion_terminal(
) -> Result<Terminal<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = stdout.into_alternate_screen()?;

    let backend = TermionBackend::new(stdout);

    Terminal::new(backend)
}

/// Setup a crossterm terminal with alternate screen enabled.
#[cfg(feature = "crossterm")]
fn setup_crossterm_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());

    Terminal::new(backend)
}

/// Return terminal to its original state.
#[cfg(feature = "crossterm")]
fn teardown_crossterm_terminal() -> Result<(), io::Error> {
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "termion")]
    let mut terminal = setup_termion_terminal()?;
    #[cfg(feature = "crossterm")]
    let mut terminal = setup_crossterm_terminal()?;

    #[cfg(feature = "termion")]
    let mut app: App<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>> = App::new();
    #[cfg(feature = "crossterm")]
    let mut app: App<CrosstermBackend<io::Stdout>> = App::new();

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

    #[cfg(feature = "crossterm")]
    teardown_crossterm_terminal()?;

    Ok(())
}
