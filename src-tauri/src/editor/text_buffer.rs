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
    pub fn remove_text(&mut self, selection: Selection) -> Selection {
        todo!()
    }
}
