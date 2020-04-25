use std::{error::Error, io, thread, time::Duration};
use termion::{
    async_stdin,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{List, ListState, Paragraph, Text},
    Frame, Terminal,
};

/// A list of `String`s, bundled with state from tui-rs.
struct StatefulList {
    /// List state (from tui-rs crate).
    pub state: ListState,

    /// List of items to display.
    pub items: Vec<String>,
}

impl StatefulList {
    /// Create new, empty list.
    pub fn new() -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    /// Move to the next item. If already at the last one, stay there.
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            None => 0,

            Some(i) => {
                if i >= self.items.len() - 1 {
                    self.items.len() - 1
                } else {
                    i + 1
                }
            }
        };

        self.state.select(Some(i));
    }

    /// Move to the previous item. If already at the first one, stay there.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            None => 0,

            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
        };

        self.state.select(Some(i));
    }
}

/// State of our application.
struct App {
    /// Should we quit on the next iteration of the event loop?
    pub should_quit: bool,

    /// Feedlist.
    pub feeds: StatefulList,
}

impl App {
    /// Create new, empty app.
    pub fn new() -> App {
        App {
            should_quit: false,
            feeds: StatefulList::new(),
        }
    }

    /// Handle key `c` pressed by the user.
    pub fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
        }
    }
}

/// Draw the application `app` to the screen `frame`.
fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let layout = Layout::default()
        .constraints(
            [
                Constraint::Length(1), // title
                Constraint::Min(0),    // feedlist
                Constraint::Length(1), // hints
                Constraint::Length(1), // command line (TODO: implement)
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
        let paragraph = Paragraph::new(title.iter()).wrap(true);
        frame.render_widget(paragraph, layout[0]);
    }

    {
        let list = List::new(
            app.feeds
                .items
                .iter()
                .map(|text| Text::styled(text.to_string(), Style::default().fg(Color::Green))),
        )
        .highlight_style(Style::default().fg(Color::White).modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, layout[1], &mut app.feeds.state);
    }

    {
        let hints = [Text::styled(
        "ESC,q:Quit ENTER:open n:Next Unread r:Reload R:Reload All A:Mark Read C:Mark All Read /:Search ?:Help",
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Blue)
            .modifier(Modifier::BOLD),
        )];
        let paragraph = Paragraph::new(hints.iter()).wrap(true);
        frame.render_widget(paragraph, layout[2]);
    }
}

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
        terminal.draw(|mut frame| draw(&mut frame, &mut app))?;

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
