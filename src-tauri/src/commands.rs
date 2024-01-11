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
pub fn create_buffer_from_file_path(path: String) -> Result<usize, String> {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    for (idx, buffer) in editor_state.text_buffers.iter().enumerate() {
        if let Some(file_path) = &buffer.file_path {
            if path.contains(file_path) {
                return Ok(idx);
            }
        }
    }

    let buf = file_handling::read_file_content(&path).map_err(|err| err.to_string())?;
    editor_state
        .text_buffers
        .push(LineTextBuffer::from_file(buf, path));
    let buffer_idx = editor_state.text_buffers.len() - 1;
    editor_state.text_buffers[buffer_idx].language = Language::Python;

    Ok(buffer_idx)
}

#[tauri::command]
pub fn save_buffer(buffer_idx: usize) -> Result<String, String> {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let content = editor_state.text_buffers[buffer_idx].get_content();
    let path = editor_state.text_buffers[buffer_idx]
        .file_path
        .as_ref()
        .unwrap();
    file_handling::override_file_content(path, content).map_err(|err| err.to_string())?;

    Ok("Success".into())
}

#[tauri::command]
pub fn save_buffer_to_new_file(buffer_idx: usize, path: String) -> Result<String, String> {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let content = editor_state.text_buffers[buffer_idx].get_content();
    file_handling::override_file_content(&path, content).map_err(|err| err.to_string())?;

    Ok("Success".into())
}

#[tauri::command]
pub fn get_highlighted_text(buffer_idx: usize) -> highlight::HighlightedText {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    editor_state.text_buffers[buffer_idx].highlight_complete_text()
}

#[tauri::command]
pub fn get_row_length(buffer_idx: usize, row: usize) -> usize {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let row_length = editor_state.text_buffers[buffer_idx].get_row_length(row);
    row_length
}

#[tauri::command]
pub fn get_lines_length(buffer_idx: usize) -> usize {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let row_length = editor_state.text_buffers[buffer_idx].get_lines_length();
    row_length
}

#[tauri::command]
pub fn insert_text(
    buffer_idx: usize,
    text: String,
    cursor: Cursor,
) -> (highlight::HighlightedText, Cursor) {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let updated_cursor = editor_state.text_buffers[buffer_idx].insert_text(text, cursor);
    (
        editor_state.text_buffers[buffer_idx].highlight_complete_text(),
        updated_cursor,
    )
}

#[tauri::command]
pub fn remove_text(
    buffer_idx: usize,
    selection: Selection,
) -> (highlight::HighlightedText, String, Cursor) {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let (removed_text, updated_cursor) =
        editor_state.text_buffers[buffer_idx].remove_text(selection);
    (
        editor_state.text_buffers[buffer_idx].highlight_complete_text(),
        removed_text,
        updated_cursor,
    )
}

#[tauri::command]
pub fn undo(buffer_idx: usize) -> Option<(highlight::HighlightedText, Cursor)> {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let updated_cursor = editor_state.text_buffers[buffer_idx].undo();
    match updated_cursor {
        Some(cursor) => {
            return Some((
                editor_state.text_buffers[buffer_idx].highlight_complete_text(),
                cursor,
            ));
        }
        None => return None,
    };
}

#[tauri::command]
pub fn redo(buffer_idx: usize) -> Option<(highlight::HighlightedText, Cursor)> {
    let mut editor_state = EDITOR_STATE.get().write().unwrap();
    let updated_cursor = editor_state.text_buffers[buffer_idx].redo();
    match updated_cursor {
        Some(cursor) => {
            return Some((
                editor_state.text_buffers[buffer_idx].highlight_complete_text(),
                cursor,
            ));
        }
        None => return None,
    };
}

#[tauri::command]
pub fn get_selected_text(buffer_idx: usize, selection: Selection) -> String {
    let editor_state = EDITOR_STATE.get().read().unwrap();
    let selected_text = editor_state.text_buffers[buffer_idx].get_selected_text(selection);
    selected_text
}
