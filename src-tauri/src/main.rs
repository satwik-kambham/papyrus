// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use state::InitCell;
use std::sync::Arc;
use std::sync::Mutex;

pub mod commands;
pub mod editor;
pub mod editor_io;
pub mod terminal;

#[cfg(test)]
pub mod tests;

static EDITOR_STATE: InitCell<Arc<Mutex<editor::state::EditorState>>> = InitCell::new();

#[tokio::main]
async fn main() {
    EDITOR_STATE.set(Arc::new(Mutex::new(editor::state::EditorState::new())));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::init_pty,
            commands::send_to_pty,
            commands::resize_pty,
            commands::get_folder_content,
            commands::create_buffer_from_file_path,
            commands::get_highlighted_text,
            commands::insert_text,
            commands::remove_text,
            commands::get_row_length,
            commands::get_lines_length,
            commands::save_buffer,
            commands::save_buffer_to_new_file,
            commands::get_selected_text,
            commands::undo,
            commands::redo,
            commands::get_file_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
