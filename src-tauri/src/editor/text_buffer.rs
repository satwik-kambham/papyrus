use crate::editor::highlight;
use crate::editor::update;

/// To be implemented by different buffer data structures
pub trait TextBuffer {
    /// Returns highlighted code from start_line to end_line
    fn get_highlighted_code(
        &self,
    ) -> highlight::HighlightedCode;

    /// Update buffer data based on code changes
    fn update_code(u: update::Update);
}

/// Basic text buffer implementation using lines
pub struct LineTextBuffer {
    pub lines: Vec<String>,
}

impl LineTextBuffer {
    pub fn new(initial_code: String) -> Self {
        let mut lines: Vec<String> = initial_code.lines().map(String::from).collect();

        if initial_code.ends_with("\n") && !initial_code.ends_with("\\n") {
            lines.push("".into());
        }

        Self { lines }
    }
}

impl TextBuffer for LineTextBuffer {
    fn get_highlighted_code(
        &self,
    ) -> highlight::HighlightedCode {
        let mut highlighted_code = highlight::HighlightedCode { code: vec![] };

        for line in self.lines.iter() {
            highlighted_code
                .code
                .push(vec![(highlight::HighlightType::None, line.clone())])
        }

        highlighted_code
    }

    fn update_code(u: update::Update) {
        todo!()
    }
}
