use crate::editor::highlight;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Cursor {
    pub row: usize,
    pub column: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Selection {
    pub start: Cursor,
    pub end: Cursor,
}

/// Basic text buffer implementation using lines
pub struct LineTextBuffer {
    pub lines: Vec<String>,
}

impl LineTextBuffer {
    /// Creates a new line based text buffer from the given initial text
    pub fn new(initial_text: String) -> Self {
        let mut lines: Vec<String> = initial_text.lines().map(String::from).collect();

        if initial_text.ends_with("\n") && !initial_text.ends_with("\\n") {
            lines.push("".into());
        }

        Self { lines }
    }

    /// Highlights the entire text (Language specific)
    /// TODO: Create a variant that updates the highlighting and passes only the changed lines
    pub fn get_highlighted_text(&self) -> highlight::HighlightedText {
        let mut highlighted_text = highlight::HighlightedText { text: vec![] };

        for line in self.lines.iter() {
            highlighted_text
                .text
                .push(vec![(highlight::HighlightType::None, line.clone())])
        }

        highlighted_text
    }

    /// Returns the column length of the given row
    pub fn get_row_length(&self, cursor: Cursor) -> usize {
        self.lines[cursor.row].len()
    }

    /// Insert text at cursor position and returns the updated cursor position
    pub fn insert_text(&mut self, text: String, cursor: Cursor) -> Cursor {
        let mut updated_cursor = cursor.clone();
        let current_line = self.lines[cursor.row].clone();
        let mut text_iter = text.split('\n');
        let (s1, s2) = current_line.split_at(cursor.column);
        let mut s1 = s1.to_string();
        let e = text_iter.next().unwrap();
        updated_cursor.column += e.len();
        s1.push_str(e);
        self.lines[cursor.row] = s1;
        for i in text_iter {
            updated_cursor.row += 1;
            updated_cursor.column = i.len();
            self.lines.insert(updated_cursor.row, i.to_owned());
        }
        let mut current_line = self.lines[updated_cursor.row].clone();
        current_line.push_str(s2);
        self.lines[updated_cursor.row] = current_line;
        updated_cursor
    }

    /// Remove the selected text and returns the updated cursor position
    /// and the deleted text
    pub fn remove_text(&mut self, selection: Selection) -> (String, Cursor) {
        if selection.start.row == selection.end.row {
            let current_line = self.lines[selection.start.row].clone();
            let (first, second) = current_line.split_at(selection.end.column);
            let (first, middle) = first.split_at(selection.start.column);
            self.lines[selection.start.row] = first.to_owned() + second;
            (middle.to_owned(), selection.start)
        } else {
            let mut buf = String::new();

            let current_line = self.lines[selection.end.row].clone();
            let (first, second) = current_line.split_at(selection.end.column);
            buf.insert_str(0, first);
            self.lines.remove(selection.end.row);

            for i in (selection.start.row + 1..selection.end.row).rev() {
                let current_line = self.lines.remove(i);
                buf.insert(0, '\n');
                buf.insert_str(0, &current_line);
            }

            let current_line = self.lines[selection.start.row].clone();
            let (first, middle) = current_line.split_at(selection.start.column);
            buf.insert(0, '\n');
            buf.insert_str(0, middle);
            self.lines[selection.start.row] = first.to_owned() + second;
            (buf, selection.start)
        }
    }
}
