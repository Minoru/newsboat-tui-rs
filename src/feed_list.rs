//! List of feeds.

use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{List, Paragraph, Text},
    Frame,
};

use crate::app::App;
use crate::stateful_list::StatefulList;

/// List of feeds.
pub struct FeedList {
    /// The state of the feedlist (what items it contains, what item is currently selected)
    pub state: StatefulList,
}

impl FeedList {
    /// Create example feedlist
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

/// Draw list of feeds from application `app` to the window `frame`.
pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
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
                .state
                .items
                .iter()
                .map(|text| Text::styled(text.to_string(), Style::default().fg(Color::Green))),
        )
        .highlight_style(Style::default().fg(Color::White).modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, layout[1], &mut app.feeds.state.state);
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
