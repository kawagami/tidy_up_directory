mod comic_directory;
mod helper;

use comic_directory::{ComicDirectory, FileType};
use helper::get_file_type;

fn main() {
    // 取得 comic 資料夾(level 0)屬於資料夾(level 1)的路徑
    // 在 level 1 中可能有兩種狀況加上未定義狀況 1. level 2 是複數圖片 2. level 2 是壓縮檔 3. 未定義狀況
    // 狀況 1 : 取得 level 1 的名稱 A，將所有圖片壓縮成同 A 的壓縮檔 B，將 B 往上移動檔案到 level 1 同層級，刪除空的 level 1 資料夾
    // 狀況 2 : 取得 level 1 的名稱 A，將 level 2 重新命名為 A，將重新命名後的 level 2 往上移動檔案到 level 1 同層級，刪除空的 level 1 資料夾
    // 狀況 3 : 跳出這次處理

    let path = "C:/for_test"; // 指定要查看的路径

    if let Ok(entries) = std::fs::read_dir(path) {
        let mut comic_directory = ComicDirectory::new();

        for entry in entries {
            if let Ok(entry) = entry {
                match get_file_type(entry.path().as_path()) {
                    FileType::Directory => comic_directory.add_one_directory(entry.path()),
                    FileType::ZipFile => comic_directory.add_one_zip_file(entry.path()),
                    FileType::Other => comic_directory.add_one_unexpect_files(entry.path()),
                }
            }
        }

        comic_directory.show_directories();

        comic_directory.show_zip_files();

        comic_directory.show_unexpect_files();
    } else {
        println!("無法讀取目錄");
    }
}
