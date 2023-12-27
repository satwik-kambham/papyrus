use std::{fs::File, io::Read};

#[tauri::command]
pub fn read_file_content(path: String) -> Result<String, String> {
    let mut f = File::open(path).map_err(|err| err.to_string())?;
    let mut buf = String::new();

    let _ = f.read_to_string(&mut buf).map_err(|err| err.to_string())?;

    Ok(buf)
}
