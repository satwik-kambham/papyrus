use crate::editor::highlight;
use crate::editor::text_buffer::{Cursor, LineTextBuffer, Selection};
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
pub fn get_row_length(cursor: Cursor) -> usize {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let row_length = editor_state.text_buffer.get_row_length(cursor);
    row_length
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

#[tauri::command]
pub fn remove_text(selection: Selection) -> (highlight::HighlightedText, String, Cursor) {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let (removed_text, updated_cursor) = editor_state.text_buffer.remove_text(selection);
    (
        editor_state.text_buffer.get_highlighted_text(),
        removed_text,
        updated_cursor,
    )
}
