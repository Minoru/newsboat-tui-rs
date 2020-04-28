//! List of feeds.

use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{List, Paragraph, Text},
    Frame,
};

use crate::app::App;
use crate::form_action::FormAction;
use crate::stateful_list::StatefulList;

/// List of feeds.
pub struct FeedList {
    /// The state of the feedlist (what items it contains, what item is currently selected)
    pub state: StatefulList,
}

impl FeedList {
    /// Create example feedlist.
    pub fn new() -> FeedList {
        let mut state = StatefulList::new();

        state.items = vec![
            "   1    (14/532) Planet Debian".to_string(),
            "   2       (0/1) Интересное на ДОУ".to_string(),
            "   3 N (23/4558) Fabio Franchino’s blog".to_string(),
            "   4      (0/13) @prometheusmooc on Twitter".to_string(),
            "   5    (12/482) /dev/lawyer".to_string(),
            "   6 N   (3/148) non-O(n) musings".to_string(),
        ];
        state.state.select(Some(0));

        FeedList { state }
    }
}

impl<B: Backend> FormAction<B> for FeedList {
    fn draw(&mut self, frame: &mut Frame<B>) {
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
                self.state
                    .items
                    .iter()
                    .map(|text| Text::styled(text.to_string(), Style::default().fg(Color::Green))),
            )
            .highlight_style(Style::default().fg(Color::White).modifier(Modifier::BOLD));

            frame.render_stateful_widget(list, layout[1], &mut self.state.state);
        }

        {
            let hints = [Text::styled(
                "q:Quit UP:Previous DOWN:Next",
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .modifier(Modifier::BOLD),
            )];
            let paragraph = Paragraph::new(hints.iter()).wrap(true);
            frame.render_widget(paragraph, layout[2]);
        }
    }

    fn handle_key(&mut self, key: Key, _app: &mut App<B>) {
        match key {
            Key::Up => self.state.previous(),

            Key::Down => self.state.next(),

            _ => {}
        }
    }
}
