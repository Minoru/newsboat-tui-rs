//! An input field for a single line of plain text.

use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};

#[derive(Debug, Clone)]
pub struct TextLineState {
    text: String,
    cursor_position: usize,
}

impl Default for TextLineState {
    fn default() -> TextLineState {
        TextLineState {
            text: String::new(),
            cursor_position: 0,
        }
    }
}

impl TextLineState {
    /// Currently entered text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Current position of the cursor.
    ///
    /// Note that counting starts from zero, and the cursor is always at the position where a new
    /// char will be inserted. E.g. for empty string, cursor is at position 0.
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Set the text and puts the cursor one char past the end.
    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
        self.cursor_position = self.text.len();
    }

    /// Clip cursor position to interval of [0; text_length].
    fn reestablish_cursor_position_invariants(&mut self) {
        self.cursor_position = self
            .cursor_position
            .max(0usize) // clip to 0 on the left
            .min(self.text.len()); // clip to text_length on the right
    }

    /// Move cursor to the given position.
    ///
    /// The position is clipped into the interval of [0; text_length].
    pub fn set_cursor_position(&mut self, new_position: usize) {
        self.cursor_position = new_position;
        self.reestablish_cursor_position_invariants();
    }

    /// Put given character at current cursor position and advance the cursor.
    pub fn put_char(&mut self, c: char) {
        self.text.insert(self.cursor_position, c);
        self.move_right(1);
    }

    /// Remove the character under cursor. Does nothing if there is no character under the cursor.
    pub fn delete_current_char(&mut self) {
        if self.cursor_position < self.text.len() {
            self.text.remove(self.cursor_position);
            self.reestablish_cursor_position_invariants();
        }
    }

    /// Remove the character to the left of cursor. Does nothing if there is no character there.
    pub fn delete_previous_char(&mut self) {
        if self.cursor_position != 0 {
            self.text.remove(self.cursor_position - 1);
            self.move_left(1);
        }
    }

    /// Move cursor to the left by `offset` characters. Stops at the beginning of the text.
    pub fn move_left(&mut self, offset: usize) {
        self.set_cursor_position(self.cursor_position.saturating_sub(offset));
    }

    /// Move cursor to the right by `offset` characters. Stop at the end of the text.
    pub fn move_right(&mut self, offset: usize) {
        self.set_cursor_position(self.cursor_position.saturating_add(offset));
    }
}

#[derive(Debug, Clone)]
pub struct TextLine {}

impl TextLine {
    pub fn new() -> TextLine {
        TextLine {}
    }
}

impl StatefulWidget for TextLine {
    type State = TextLineState;

    // TODO: make sure the cursor position is visible; skip some leading or trailing text if needed
    //      Will have to update feed_list::draw() to use the same algorithm
    //      Perhaps encapsulate the algorithm into a public function that can be used from outside
    //      to figure out where the cursor should be put
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_string(area.left(), area.top(), &state.text, Style::default());
    }
}
