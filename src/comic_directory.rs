use std::path::PathBuf;

use crate::helper::get_file_type;

pub enum FileType {
    Directory,
    ZipFile,
    Other,
}

pub struct ComicDirectory {
    // 未整理的資料夾
    directories: Vec<PathBuf>,
    // 壓縮過的檔案
    zip_files: Vec<PathBuf>,
    // 預計之外的事物
    unexpect_files: Vec<PathBuf>,
}

enum InnerType {
    ZipFile,
    Pics,
    Errors,
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

    pub fn _show_directories(&self) {
        println!("");
        for dir_path in &self.directories {
            let path = dir_path.to_str().expect("取得 path str 失敗");
            println!("{}", path);
        }
        println!("");
    }

    pub fn _show_zip_files(&self) {
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

    pub fn classify(self) {
        self.directories.iter().for_each(|directory_path| {
            match count_files_in_folder(directory_path) {
                InnerType::Pics => {
                    if let Some(file_name) = directory_path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            println!("{}\nis\n{}\n", file_name_str, "Pics status");
                        }
                    }
                }
                InnerType::ZipFile => {
                    println!("{:?}\nis\n{}\n", directory_path.to_str(), "ZipFile status")
                }
                InnerType::Errors => {
                    println!("{:?}\nis\n{}\n", directory_path.to_str(), "Errors status")
                }
            }
        });
    }
}

fn count_files_in_folder(path: &PathBuf) -> InnerType {
    let mut file_count = 0;

    let entries = std::fs::read_dir(path).expect("read dir fail");
    for entry in entries {
        let entry = entry.expect("count_files_in_folder fail");
        let metadata = entry.metadata().expect("count_files_in_folder fail");
        if metadata.is_file() {
            file_count += 1;
        }
    }

    if file_count > 1 {
        InnerType::Pics
    } else if file_count == 1 {
        InnerType::ZipFile
    } else {
        InnerType::Errors
    }
}
