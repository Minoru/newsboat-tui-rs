//! List of feeds.

use std::{cell::RefCell, rc::Rc};
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{List, Paragraph, Text},
    Frame,
};

use crate::app::App;
use crate::form_action::FormAction;
use crate::item_list::ItemList;
use crate::stateful_list::StatefulList;
use crate::widgets::text_line;

/// Which widget should process input?
pub enum Focus {
    /// Input goes to the feedlist.
    Dialog,

    /// Input goes to the command line.
    ///
    /// Command line has a state that's only useful when the focus is on the command line, so we
    /// keep it here rather than in `FeedList`.
    CommandLine(text_line::TextLineState),
}

/// List of feeds.
pub struct FeedList {
    /// The state of the feedlist (what items it contains, what item is currently selected)
    list_state: StatefulList,

    /// Which widget should process input?
    focus: Focus,
}

impl FeedList {
    /// Create example feedlist.
    pub fn new() -> FeedList {
        let mut list_state = StatefulList::new();

        list_state.items = vec![
            "   1    (14/532) Planet Debian".to_string(),
            "   2       (0/1) Интересное на ДОУ".to_string(),
            "   3 N (23/4558) Fabio Franchino’s blog".to_string(),
            "   4      (0/13) @prometheusmooc on Twitter".to_string(),
            "   5    (12/482) /dev/lawyer".to_string(),
            "   6 N   (3/148) non-O(n) musings".to_string(),
        ];
        list_state.state.select(Some(0));

        FeedList {
            list_state,
            focus: Focus::Dialog,
        }
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
            let paragraph = Paragraph::new(title.iter()).style(Style::default().bg(Color::Blue));
            frame.render_widget(paragraph, layout[0]);
        }

        {
            let list = List::new(
                self.list_state
                    .items
                    .iter()
                    .map(|text| Text::styled(text.to_string(), Style::default().fg(Color::Green))),
            )
            .highlight_style(Style::default().fg(Color::White).modifier(Modifier::BOLD));

            frame.render_stateful_widget(list, layout[1], &mut self.list_state.state);
        }

        {
            let hints = [Text::styled(
                "q:Quit UP:Previous DOWN:Next ENTER:Open",
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .modifier(Modifier::BOLD),
            )];
            let paragraph = Paragraph::new(hints.iter()).style(Style::default().bg(Color::Blue));
            frame.render_widget(paragraph, layout[2]);
        }

        {
            if let Focus::CommandLine(ref mut cli_state) = self.focus {
                let line_layout = Layout::default()
                    .constraints(
                        [
                            Constraint::Length(1), // Colon
                            Constraint::Min(0),    // Text input
                        ]
                        .as_ref(),
                    )
                    .direction(Direction::Horizontal)
                    .split(layout[3]);

                let colon = [Text::raw(":")];
                let paragraph = Paragraph::new(colon.iter());
                frame.render_widget(paragraph, line_layout[0]);

                let command_line = text_line::TextLine::new();
                frame.render_stateful_widget(command_line, line_layout[1], cli_state);
                frame.set_desired_cursor_position(
                    line_layout[1]
                        // x+cursor_offset, with careful type conversions:
                        // - cursor_offset is usize, so we limit it to u16::MAX
                        // - u16+u16 won't fit into u16. Since we just want the cursor at the end of
                        //   the text, we're using saturating addition to put cursor as far right as we
                        //   possibly can
                        .x
                        .saturating_add(
                            cli_state.cursor_display_offset().min(u16::MAX as usize) as u16
                        ),
                    line_layout[1].y,
                );
            } else {
                frame.hide_cursor();
            }
        }
    }

    fn handle_key(&mut self, key: Key, app: &mut App<B>) {
        match self.focus {
            Focus::Dialog => match key {
                Key::Char(c) => match c {
                    'q' => app.should_quit = true,

                    ':' => self.focus = Focus::CommandLine(text_line::TextLineState::default()),

                    '\n' => app.add_formaction(Rc::new(RefCell::new(ItemList::new()))),

                    _ => {}
                },

                Key::Up => self.list_state.previous(),

                Key::Down => self.list_state.next(),

                _ => {}
            },

            Focus::CommandLine(ref mut cli_state) => match key {
                Key::Char(c) => match c {
                    '\n' => {
                        if cli_state.text() == "quit" {
                            app.should_quit = true;
                        } else {
                            self.focus = Focus::Dialog;
                        }
                    }

                    _ => cli_state.put_char(c),
                },

                _ => {}
            },
        }
    }
}
