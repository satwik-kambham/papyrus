use crate::editor::text_buffer;

pub struct EditorState {
    pub text_buffers: Vec<text_buffer::LineTextBuffer>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            text_buffers: vec![],
        }
    }
}
