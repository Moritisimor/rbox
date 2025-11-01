use std::fs::DirEntry;

pub fn getentrytype(entry: DirEntry) -> String {
    match entry.file_type() {
        Ok(readentry) => {
            if readentry.is_dir() {
                "Dir".to_string()
            } else if readentry.is_file() {
                "File".to_string()
            } else {
                "Symlink".to_string()
            }
        }
        Err(_) => "Unknown".to_string()
    }
}