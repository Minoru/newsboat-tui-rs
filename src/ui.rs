//! Code for drawing application's user interface.

use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{List, Paragraph, Text},
    Frame,
};

use crate::app::App;

/// Draw the application `app` to the screen `frame`.
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
