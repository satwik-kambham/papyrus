use std::{error::Error, fs::File, io::Read};

pub fn read_file_content(path: String) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut buf = String::new();

    let _ = f.read_to_string(&mut buf)?;

    Ok(buf)
}
