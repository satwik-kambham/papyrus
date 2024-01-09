use crate::editor::highlight;
use crate::editor::text_buffer::{Cursor, Language, LineTextBuffer, Selection};
use crate::editor_io::file_handling;
use crate::EDITOR_STATE;

#[tauri::command]
pub fn get_folder_content(path: String) -> Result<Vec<file_handling::FolderEntry>, String> {
    let entries = file_handling::get_folder_content(&path).map_err(|err| err.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub fn create_buffer_from_file_path(path: String) -> Result<String, String> {
    let buf = file_handling::read_file_content(&path).map_err(|err| err.to_string())?;

    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    editor_state.text_buffer = LineTextBuffer::from_file(buf, path);
    editor_state.text_buffer.language = Language::Python;

    Ok("Success".into())
}

#[tauri::command]
pub fn save_buffer() -> Result<String, String> {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let content = editor_state.text_buffer.get_content();
    let path = editor_state.text_buffer.file_path.as_ref().unwrap();
    file_handling::override_file_content(path, content).map_err(|err| err.to_string())?;

    Ok("Success".into())
}

#[tauri::command]
pub fn save_buffer_to_new_file(path: String) -> Result<String, String> {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let content = editor_state.text_buffer.get_content();
    file_handling::override_file_content(&path, content).map_err(|err| err.to_string())?;

    Ok("Success".into())
}

#[tauri::command]
pub fn get_highlighted_text() -> highlight::HighlightedText {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    editor_state.text_buffer.highlight_complete_text()
}

#[tauri::command]
pub fn get_row_length(row: usize) -> usize {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let row_length = editor_state.text_buffer.get_row_length(row);
    row_length
}

#[tauri::command]
pub fn get_lines_length() -> usize {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let row_length = editor_state.text_buffer.get_lines_length();
    row_length
}

#[tauri::command]
pub fn insert_text(text: String, cursor: Cursor) -> (highlight::HighlightedText, Cursor) {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let updated_cursor = editor_state.text_buffer.insert_text(text, cursor);
    (
        editor_state.text_buffer.highlight_complete_text(),
        updated_cursor,
    )
}

#[tauri::command]
pub fn remove_text(selection: Selection) -> (highlight::HighlightedText, String, Cursor) {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let (removed_text, updated_cursor) = editor_state.text_buffer.remove_text(selection);
    (
        editor_state.text_buffer.highlight_complete_text(),
        removed_text,
        updated_cursor,
    )
}

#[tauri::command]
pub fn get_selected_text(selection: Selection) -> String {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let selected_text = editor_state.text_buffer.get_selected_text(selection);
    selected_text
}
