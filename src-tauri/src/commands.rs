use crate::editor::highlight;
use crate::editor::text_buffer::{Cursor, LineTextBuffer};
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
pub fn get_highlighted_text() -> highlight::HighlightedText {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    editor_state.text_buffer.get_highlighted_text()
}

#[tauri::command]
pub fn insert_text(text: String, cursor: Cursor) -> (highlight::HighlightedText, Cursor) {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let updated_cursor = editor_state.text_buffer.insert_text(text, cursor);
    (
        editor_state.text_buffer.get_highlighted_text(),
        updated_cursor,
    )
}
