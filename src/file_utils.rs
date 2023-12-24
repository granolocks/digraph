use std::fs;
use std::path::Path;

/// Read a file into a byte vec
pub fn read_file_to_bytes(filepath: &Path) -> Option<(String, Vec<u8>)> {
    if let Ok(buffer) = fs::read(&filepath) {
        let filename = filepath.file_name().unwrap().to_str().unwrap();
        let filename = String::from(filename);
        Some((filename, buffer))
    } else {
        None
    }
}


