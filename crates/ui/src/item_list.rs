//! List of feeds.

use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::{cell::RefCell, rc::Rc};

use crate::app::App;
use crate::event::Key;
use crate::form_action::FormAction;
use crate::item_view::ItemView;
use crate::stateful_list::StatefulList;

/// List of items.
pub struct ItemList {
    /// The state of the itemlist (what items it contains, what item is currently selected)
    state: StatefulList,
}

impl ItemList {
    /// Create example itemlist.
    pub fn new() -> ItemList {
        let mut state = StatefulList::new();

        state.items = vec![
            "   1    Apr 28   3.9K  NVidia acquires Mellanox".to_string(),
            "   2    Apr 28    591  [$] Dumping kernel data structure with BPF".to_string(),
            "   3    Apr 28    971  Wooden server rack".to_string(),
            "   4    Apr 28   2.2K  Trouble fully setting up baremetal homelab".to_string(),
            "   5    Apr 28    548  Looking for a very small server with 2 plus hot swap 3.5 inch driver I can install linux on.".to_string(),
            "   6    Apr 28   1.7K  VLAN and iOT devices".to_string(),
        ];
        state.state.select(Some(0));

        ItemList { state }
    }
}

impl<B: Backend> FormAction<B> for ItemList {
    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .constraints(
                [
                    Constraint::Length(1), // title
                    Constraint::Min(0),    // itemlist
                    Constraint::Length(1), // hints
                    Constraint::Length(1), // command line (TODO: implement)
                ]
                .as_ref(),
            )
            .split(frame.size());

        {
            let title = Span::styled(
                "Newsboat 2.20 (ну, почти) - Example Feed (0 unread, 0 total)",
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );
            let paragraph = Paragraph::new(title)
                .wrap(Wrap { trim: false })
                .style(Style::default().bg(Color::Blue));
            frame.render_widget(paragraph, layout[0]);
        }

        {
            let list = List::new(
                self.state
                    .items
                    .iter()
                    .map(|text| {
                        ListItem::new(Span::styled(
                            text.to_string(),
                            Style::default().fg(Color::Green),
                        ))
                    })
                    .collect::<Vec<_>>(),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );

            frame.render_stateful_widget(list, layout[1], &mut self.state.state);
        }

        {
            let hints = Span::styled(
                "q:Quit UP:Previous DOWN:Next ENTER:Open",
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );
            let paragraph = Paragraph::new(hints)
                .wrap(Wrap { trim: false })
                .style(Style::default().bg(Color::Blue));
            frame.render_widget(paragraph, layout[2]);
        }
    }

    fn handle_key(&mut self, key: Key, app: &mut App<B>) {
        match key {
            Key::Char(c) => match c {
                'q' => app.quit_current_formaction(),

                '\n' => app.add_formaction(Rc::new(RefCell::new(ItemView::new()))),

                _ => {}
            },

            Key::Up => self.state.previous(),

            Key::Down => self.state.next(),

            _ => {}
        }
    }
}
