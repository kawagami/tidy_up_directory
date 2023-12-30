use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::helper::get_file_type;
use crate::zip_fn::doit;

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
    // 現有的 zip files
    existed_zip_files: HashMap<String, String>,
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
            existed_zip_files: HashMap::new(),
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
        self.zip_files.push(path.clone());

        if let Some(file_name) = path.file_name() {
            if let Some(str) = file_name.to_str() {
                self.existed_zip_files
                    .insert(str.to_string(), "".to_string());
            }
        }
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
        self.directories.par_iter().for_each(|directory_path| {
            match count_files_in_folder(directory_path) {
                InnerType::Pics => {
                    self.handle_pics(directory_path);
                }
                InnerType::ZipFile => {
                    handle_zip_file(directory_path);
                }
                InnerType::Errors => {
                    println!("{:?}\nis\n{}\n", directory_path.to_str(), "Errors status")
                }
            }
        });
    }

    /**
     *
     */
    pub fn handle_pics(&self, path: &PathBuf) {
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                // 取得壓縮檔應該取的名稱
                let zip_name = format!("{}.zip", file_name_str);
                // 檢查是否在現有的 zip files 中存在
                if self.existed_zip_files.contains_key(&zip_name) {
                    println!("{} 壓縮檔案已經存在", file_name_str);
                } else {
                    // 執行壓縮
                    if let Some(path_str) = path.as_path().to_str() {
                        // 要壓縮的資料夾路徑 &str
                        let src_dir = path_str;
                        // 壓縮出來的檔案路徑
                        let dst_file = format!("{}.zip", path_str);

                        // println!("要壓縮的資料夾 {}", src_dir);

                        // doit(src_dir, &dst_file, zip::CompressionMethod::Stored).expect("壓縮失敗");
                        match doit(src_dir, &dst_file, zip::CompressionMethod::Stored) {
                            Ok(()) => {
                                // 壓縮成功後刪除資料夾
                                match std::fs::remove_dir_all(path_str) {
                                    Ok(()) => println!("{path_str} deleted successfully"),
                                    Err(err) => println!("Error deleting directory: {}", err),
                                }
                            }
                            Err(error) => {
                                println!("壓縮失敗\nError: {}", error);
                            }
                        }
                    } else {
                        println!("path.as_path().to_str() fail")
                    }
                }
            } else {
                println!("file_name.to_str() fail")
            }
        } else {
            println!("path.file_name() fail")
        }
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

fn handle_zip_file(path: &PathBuf) {
    println!("{:?}\nis\n{}\n", path.to_str(), "ZipFile status")
    // 取得 level 1 的名稱 A，將 level 2 重新命名為 A，將重新命名後的 level 2 往上移動檔案到 level 1 同層級，刪除空的 level 1 資料夾
}
