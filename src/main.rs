use easycurses;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CursesBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph, Text},
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

fn draw<B: Backend>(frame: &mut Frame<B>, _app: &mut App) {
    let layout = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(2),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(frame.size());

    {
        let title = [Text::styled(
            "Newsboat 2.20 (not really) - Your Feeds (0 unread, 0 total)",
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Blue)
                .modifier(Modifier::BOLD),
        )];
        let block = Block::default();
        let paragraph = Paragraph::new(title.iter()).block(block).wrap(true);
        frame.render_widget(paragraph, layout[0]);
    }

    {
        let hints = [Text::styled(
        "ESC,q:Quit ENTER:open n:Next Unread r:Reload R:Reload All A:Mark Read C:Mark All Read /:Search ?:Help",
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Blue)
            .modifier(Modifier::BOLD),
    )];
        let block = Block::default();
        let paragraph = Paragraph::new(hints.iter()).block(block).wrap(true);
        frame.render_widget(paragraph, layout[2]);
    }
}

fn setup_curses_terminal() -> Result<Terminal<CursesBackend>, io::Error> {
    let mut backend = CursesBackend::new().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Failed to initialize curses backend",
    ))?;
    let curses = backend.get_curses_mut();

    curses.set_echo(false);

    // The interface will be refreshed at least this often. Might be more often if user presses
    // something.
    const MAX_REFRESH_INTERVAL_MS: i32 = 500;
    curses.set_input_timeout(easycurses::TimeoutMode::WaitUpTo(MAX_REFRESH_INTERVAL_MS));

    curses.set_input_mode(easycurses::InputMode::RawCharacter);
    curses.set_keypad_enabled(true);

    Terminal::new(backend)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_curses_terminal()?;
    terminal.hide_cursor()?;

    let mut app = App::new();

    loop {
        terminal.draw(|mut frame| draw(&mut frame, &mut app))?;

        match terminal.backend_mut().get_curses_mut().get_input() {
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
