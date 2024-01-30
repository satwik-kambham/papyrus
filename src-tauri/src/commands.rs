use portable_pty::{CommandBuilder, PtySize};

use crate::editor::highlight;
use crate::editor::text_buffer::{Cursor, Language, LineTextBuffer, Selection};
use crate::editor_io::file_handling;
use crate::terminal::io::read_output;
use crate::EDITOR_STATE;

#[tauri::command]
pub fn init_pty(window: tauri::Window) -> Result<(), String> {
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
    let pty_system = portable_pty::native_pty_system();
    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|err| err.to_string())?;
    let cmd = CommandBuilder::new("pwsh.exe");
    let _child = pty_pair
        .slave
        .spawn_command(cmd)
        .map_err(|err| err.to_string())?;

    let reader = pty_pair
        .master
        .try_clone_reader()
        .map_err(|err| err.to_string())?;

    let writer = pty_pair
        .master
        .take_writer()
        .map_err(|err| err.to_string())?;

    std::thread::spawn(|| read_output(window, reader));

    editor_state.pty_writer = Some(writer);
    editor_state.pty_pair = Some(pty_pair);
    Ok(())
}

#[tauri::command]
pub fn send_to_pty(input: String) -> Result<(), String> {
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
    let writer = editor_state.pty_writer.as_mut().unwrap();
    write!(writer, "{}", input).map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn resize_pty(rows: u16, cols: u16) -> Result<(), String> {
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
    let pty_pair = editor_state.pty_pair.as_mut().unwrap();
    pty_pair
        .master
        .resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_folder_content(path: String) -> Result<Vec<file_handling::FolderEntry>, String> {
    let entries = file_handling::get_folder_content(&path).map_err(|err| err.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub fn get_file_info(path: String) -> file_handling::FileEntry {
    let entry = file_handling::FileEntry::new(path);
    entry
}

#[tauri::command]
pub fn create_buffer_from_file_path(path: String) -> Result<usize, String> {
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
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
pub fn delete_buffer(buffer_idx: usize) -> Result<String, String> {
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
    editor_state.text_buffers.remove(buffer_idx);
    Ok("Success".into())
}

#[tauri::command]
pub fn save_buffer(buffer_idx: usize, eol_sequence: String) -> Result<String, String> {
    let editor_state = EDITOR_STATE.get().lock().unwrap();
    let content = editor_state.text_buffers[buffer_idx].get_content(eol_sequence);
    let path = editor_state.text_buffers[buffer_idx]
        .file_path
        .as_ref()
        .unwrap();
    file_handling::override_file_content(path, content).map_err(|err| err.to_string())?;

    Ok("Success".into())
}

#[tauri::command]
pub fn save_buffer_to_new_file(
    buffer_idx: usize,
    path: String,
    eol_sequence: String,
) -> Result<String, String> {
    let editor_state = EDITOR_STATE.get().lock().unwrap();
    let content = editor_state.text_buffers[buffer_idx].get_content(eol_sequence);
    file_handling::override_file_content(&path, content).map_err(|err| err.to_string())?;

    Ok("Success".into())
}

#[tauri::command]
pub fn get_highlighted_text(buffer_idx: usize) -> highlight::HighlightedText {
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
    editor_state.text_buffers[buffer_idx].highlight_complete_text()
}

#[tauri::command]
pub fn get_row_length(buffer_idx: usize, row: usize) -> usize {
    let editor_state = EDITOR_STATE.get().lock().unwrap();
    let row_length = editor_state.text_buffers[buffer_idx].get_row_length(row);
    row_length
}

#[tauri::command]
pub fn get_lines_length(buffer_idx: usize) -> usize {
    let editor_state = EDITOR_STATE.get().lock().unwrap();
    let row_length = editor_state.text_buffers[buffer_idx].get_lines_length();
    row_length
}

#[tauri::command]
pub fn insert_text(
    buffer_idx: usize,
    text: String,
    cursor: Cursor,
) -> (highlight::HighlightedText, Cursor) {
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
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
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
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
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
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
    let mut editor_state = EDITOR_STATE.get().lock().unwrap();
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
    let editor_state = EDITOR_STATE.get().lock().unwrap();
    let selected_text = editor_state.text_buffers[buffer_idx].get_selected_text(selection);
    selected_text
}
