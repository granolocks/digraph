use std::fs;
use std::path::Path;

/// Read a file into a byte vec
pub fn read_file_to_bytes(filepath: &Path) -> Option<Vec<u8>> {
    if let Ok(buffer) = fs::read(&filepath) {
        Some(buffer)
    } else {
        None
    }
}
