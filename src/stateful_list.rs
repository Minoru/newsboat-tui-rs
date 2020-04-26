//! A list of `String`s, bundled with state from tui-rs.

use tui::widgets::ListState;

/// A list of `String`s, bundled with state from tui-rs.
pub struct StatefulList {
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
