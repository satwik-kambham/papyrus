#[derive(Clone, serde::Serialize)]
struct Payload {
    output: String,
}

pub fn read_output(
    window: tauri::Window,
    mut reader: Box<dyn std::io::Read + Send>,
) -> Result<(), String> {
    let mut buf = [0; 1024];
    loop {
        let bytes_read = reader.read(&mut buf).map_err(|err| err.to_string())?;
        if bytes_read == 0 {
            break;
        }
        let _ = window
            .emit(
                "terminal_output",
                Payload {
                    output: String::from_utf8_lossy(&buf[..bytes_read]).to_string(),
                },
            )
            .map_err(|err| err.to_string())?;
    }
    Ok(())
}
