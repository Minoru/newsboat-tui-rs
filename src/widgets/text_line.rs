//! An input field for a single line of plain text.

use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};

#[derive(Debug, Clone)]
pub struct TextLineState {
    text: String,

    // The screen might be too narrow to display the whole text, but the cursor should always be
    // visible. To achieve this, the widget keeps track of two values:
    // 1. viewport_offset, which is the number of characters at the front of the string that are
    //    outside the displayed area;
    // 2. cursor_position, which is cursor's offset inside the text.
    //
    // Visually it looks something like this:
    //
    //                                        TextLine
    //                     +------------------------------------------------+
    // Quick brown fox jumps over the lazy dog.                             |
    //                     +------------------------------------------------+
    // `-------------------'            ^
    //    viewport_offset                `-- cursor is here
    // `--------------------------------'
    //            cursor_position
    //
    // The widget always keeps the cursor in sight, i.e. cursor_position is in [viewport_offset;
    // viewport_offset + viewport_width). When cursor_position reaches its viewport_width, each call to put_char
    // increases viewport_offset, moving more text out of sight to make space for the new character.
    viewport_offset: usize,
    cursor_position: usize,
}

impl Default for TextLineState {
    fn default() -> TextLineState {
        TextLineState {
            text: String::new(),
            viewport_offset: 0,
            cursor_position: 0,
        }
    }
}

impl TextLineState {
    // This API is incomplete, e.g. it lacks full cursor control. For this demo, we simply don't
    // care; all of those functions are trivial to add and don't have an impact on our evaluation
    // of tui-rs.

    /// Currently entered text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Current offset inside the display area (at the time of the last `render()` call).
    pub fn cursor_display_offset(&self) -> usize {
        self.cursor_position - self.viewport_offset
    }

    /// Clip cursor position to interval of [0; text_length].
    fn reestablish_cursor_position_invariants(&mut self) {
        self.cursor_position = self.cursor_position.min(self.text.len()); // clip to text_length on the right

        if self.viewport_offset > self.cursor_position {
            self.viewport_offset = self.cursor_position;
        }
    }

    /// Move cursor to the given position in text.
    ///
    /// The position is clipped into the interval of [0; text_length].
    fn set_cursor_position(&mut self, new_position: usize) {
        self.cursor_position = new_position;
        self.reestablish_cursor_position_invariants();
    }

    /// Put given character at current cursor position and advance the cursor.
    pub fn put_char(&mut self, c: char) {
        self.text.insert(self.cursor_position, c);
        self.move_right(1);
    }

    /// Move cursor to the right by `offset` characters. Stop at the end of the text.
    fn move_right(&mut self, offset: usize) {
        self.set_cursor_position(self.cursor_position.saturating_add(offset));
    }

    fn make_cursor_visible(&mut self, viewport_width: usize) {
        let diff = self
            .cursor_position
            .saturating_sub(self.viewport_offset)
            // We have to subtract 1 to reserve a space for cursor.
            .saturating_sub(viewport_width.saturating_sub(1));
        if diff > 0 {
            // Cursor is too far to the right, out of the viewport. Shift viewport to put cursor
            // at the end of the display area.
            self.viewport_offset = self.viewport_offset + diff;
        }
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

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.make_cursor_visible(area.width as usize);
        buf.set_string(
            area.left(),
            area.top(),
            &state.text[state.viewport_offset..],
            Style::default(),
        );
    }
}
