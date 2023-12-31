use crate::editor::highlight;
use crate::editor::text_buffer::{LineTextBuffer, TextBuffer};
use crate::editor_io::file_handling;
use crate::EDITOR_STATE;

#[tauri::command]
pub fn create_buffer_from_file_path(path: String) -> Result<String, String> {
    let buf = file_handling::read_file_content(path).map_err(|err| err.to_string())?;

    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    editor_state.text_buffer = LineTextBuffer::new(buf);

    Ok("Success".into())
}

#[tauri::command]
pub fn get_highlighted_code() -> highlight::HighlightedCode {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    editor_state.text_buffer.get_highlighted_code()
}
