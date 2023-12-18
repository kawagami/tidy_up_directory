use crate::comic_directory::FileType;
use std::path::Path;

pub fn get_file_type(path: &Path) -> FileType {
    if path.is_dir() {
        FileType::Directory
    } else if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            if ext_str.to_lowercase() == "zip" {
                FileType::ZipFile
            } else {
                FileType::Other
            }
        } else {
            FileType::Other
        }
    } else {
        FileType::Other
    }
}
