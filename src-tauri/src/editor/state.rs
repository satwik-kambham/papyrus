use std::io::Write;

use crate::editor::text_buffer;

pub struct EditorState {
    pub text_buffers: Vec<text_buffer::LineTextBuffer>,
    pub pty_pair: Option<portable_pty::PtyPair>,
    pub pty_writer: Option<Box<dyn Write + Send>>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            text_buffers: vec![],
            pty_pair: None,
            pty_writer: None,
        }
    }
}
