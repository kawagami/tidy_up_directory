use std::path::PathBuf;

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
    pub fn new() -> Self {
        Self {
            directories: vec![],
            zip_files: vec![],
            unexpect_files: vec![],
        }
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

    pub fn show_unexpect_files(&self) {
        println!("");
        for dir_path in &self.unexpect_files {
            let path = dir_path.to_str().expect("取得 path str 失敗");
            println!("{}", path);
        }
        println!("");
    }
}
