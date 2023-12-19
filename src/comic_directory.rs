use std::path::PathBuf;

use crate::helper::get_file_type;

pub enum FileType {
    Directory,
    ZipFile,
    Other,
}

pub struct ComicDirectory {
    directories: Vec<PathBuf>,
    zip_files: Vec<PathBuf>,
    unexpect_files: Vec<PathBuf>,
}

impl ComicDirectory {
    pub fn new(path: &str) -> Self {
        let mut result = Self {
            directories: vec![],
            zip_files: vec![],
            unexpect_files: vec![],
        };

        let entries = std::fs::read_dir(path).expect("read dir fail");

        for entry in entries {
            if let Ok(entry) = entry {
                match get_file_type(entry.path().as_path()) {
                    FileType::Directory => result.add_one_directory(entry.path()),
                    FileType::ZipFile => result.add_one_zip_file(entry.path()),
                    FileType::Other => result.add_one_unexpect_files(entry.path()),
                }
            }
        }

        result
    }

    pub fn add_one_directory(&mut self, path: PathBuf) {
        self.directories.push(path);
    }

    pub fn add_one_zip_file(&mut self, path: PathBuf) {
        self.zip_files.push(path);
    }

    pub fn add_one_unexpect_files(&mut self, path: PathBuf) {
        self.unexpect_files.push(path);
    }

    pub fn show_directories(&self) {
        println!("");
        for dir_path in &self.directories {
            let path = dir_path.to_str().expect("取得 path str 失敗");
            println!("{}", path);
        }
        println!("");
    }

    pub fn show_zip_files(&self) {
        println!("");
        for dir_path in &self.zip_files {
            let path = dir_path.to_str().expect("取得 path str 失敗");
            println!("{}", path);
        }
        println!("");
    }

    pub fn _show_unexpect_files(&self) {
        println!("");
        for dir_path in &self.unexpect_files {
            let path = dir_path.to_str().expect("取得 path str 失敗");
            println!("{}", path);
        }
        println!("");
    }
}
