fn main() {
    println!("Hello, world!");

    // 取得 comic 資料夾(level 0)屬於資料夾(level 1)的路徑
    // 在 level 1 中可能有兩種狀況加上未定義狀況 1. level 2 是複數圖片 2. level 2 是壓縮檔 3. 未定義狀況
    // 狀況 1 : 取得 level 1 的名稱 A，將所有圖片壓縮成同 A 的壓縮檔 B，將 B 往上移動檔案到 level 1 同層級，刪除空的 level 1 資料夾
    // 狀況 2 : 取得 level 1 的名稱 A，將 level 2 重新命名為 A，將重新命名後的 level 2 往上移動檔案到 level 1 同層級，刪除空的 level 1 資料夾
    // 狀況 3 : 跳出這次處理
}
