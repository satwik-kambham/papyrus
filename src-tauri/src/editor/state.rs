use crate::editor::text_buffer;

pub struct EditorState {
    pub text_buffer: text_buffer::LineTextBuffer,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            text_buffer: text_buffer::LineTextBuffer::new("".into()),
        }
    }
}
