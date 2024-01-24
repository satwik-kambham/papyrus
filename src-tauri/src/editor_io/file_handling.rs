use std::{
    cmp::Ordering,
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FolderEntry {
    path: String,
    is_dir: bool,
    name: String,
    extension: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FileEntry {
    path: String,
    name: String,
    extension: String,
}

impl PartialOrd for FolderEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FolderEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize is_dir, then name
        other
            .is_dir
            .cmp(&self.is_dir)
            .then_with(|| self.name.to_lowercase().cmp(&other.name.to_lowercase()))
    }
}

impl FileEntry {
    pub fn new(path_str: String) -> Self {
        let path = Path::new(&path_str);

        Self {
            path: path_str.clone(),
            name: path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
            extension: path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

pub fn read_file_content(path: &str) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut buf = String::new();

    let _ = f.read_to_string(&mut buf)?;

    Ok(buf)
}

pub fn override_file_content(path: &str, buf: String) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(path)?;
    f.write_all(buf.as_bytes())?;

    Ok(())
}

pub fn get_folder_content(path: &str) -> Result<Vec<FolderEntry>, Box<dyn Error>> {
    let path = Path::new(path);
    assert!(path.is_dir());

    let mut entries: Vec<FolderEntry> = vec![];
    let entry_iter = fs::read_dir(path)?;
    for entry in entry_iter {
        let entry = entry?;
        let folder_entry = FolderEntry {
            path: entry.path().to_str().unwrap().to_string(),
            is_dir: entry.metadata()?.is_dir(),
            name: entry
                .path()
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
            extension: entry
                .path()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
        };
        entries.push(folder_entry);
    }
    entries.sort();

    Ok(entries)
}
