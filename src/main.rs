use std::{error::Error, io, thread, time::Duration};
use termion::{
    async_stdin,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{backend::TermionBackend, Terminal};

mod app;
mod statefullist;
mod ui;

use app::App;

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

    app.feeds.items = vec![
        "   1    (14/532) Planet Debian".to_string(),
        "   2       (0/1) Интересное на ДОУ".to_string(),
        "   3 N (23/4558) Fabio Franchino’s blog".to_string(),
        "   4      (0/13) @prometheusmooc on Twitter".to_string(),
        "   5    (12/482) /dev/lawyer".to_string(),
        "   6 N   (3/148) non-O(n) musings".to_string(),
    ];
    app.feeds.state.select(Some(0));

    let mut stdin = async_stdin().keys();
    loop {
        terminal.draw(|mut frame| ui::draw(&mut frame, &mut app))?;

        if let Some(event) = stdin.next() {
            match event {
                Ok(key) => match key {
                    Key::Char(c) => app.on_key(c),

                    Key::Up => app.feeds.previous(),

                    Key::Down => app.feeds.next(),

                    _ => {}
                },

                Err(_) => {}
            }

            if app.should_quit {
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
