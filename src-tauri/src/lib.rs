// 声明子模块
pub mod error;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册插件
        .plugin(tauri_plugin_opener::init())
        // 注册命令处理器
        .invoke_handler(tauri::generate_handler![])
        // 运行
        .run(tauri::generate_context!())
        // 处理错误
        .expect("error while running tauri application");
}
