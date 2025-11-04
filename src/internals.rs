/* 
This file is primarily meant for providing helper functions for 
other functions which are otherwise not meant to be exported.
*/

// Meant to streamline the process of getting the type of a directory entry
pub fn getentrytype(entry: std::fs::DirEntry) -> String {
    match entry.file_type() {
        Err(_) => "Unknown".to_string(),
        Ok(readentry) => {
            if readentry.is_dir() {
                "Dir".to_string()
            } else if readentry.is_file() {
                "File".to_string()
            } else {
                "Symlink".to_string()
            }
        }
    }
}