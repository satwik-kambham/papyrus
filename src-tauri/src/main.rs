// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::RwLock;

use state::InitCell;

pub mod commands;
pub mod editor;
pub mod editor_io;

#[cfg(test)]
pub mod tests;

static EDITOR_STATE: InitCell<RwLock<editor::state::EditorState>> = InitCell::new();

fn main() {
    EDITOR_STATE.set(RwLock::new(editor::state::EditorState::new()));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
