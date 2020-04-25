use std::{error::Error, io, thread, time::Duration};
use termion::{
    async_stdin,
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
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
            "Newsboat 2.20 (ну, почти) - Your Feeds (0 unread, 0 total)",
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

fn setup_termion_terminal() -> Result<
    Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>>,
    io::Error,
> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);

    let backend = TermionBackend::new(stdout);

    Terminal::new(backend)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_termion_terminal()?;
    terminal.hide_cursor()?;

    let mut app = App::new();

    let mut stdin = async_stdin().keys();
    loop {
        terminal.draw(|mut frame| draw(&mut frame, &mut app))?;

        if let Some(event) = stdin.next() {
            match event {
                Ok(key) => match key {
                    Key::Char(c) => app.on_key(c),

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
