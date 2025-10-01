// 声明子模块
pub mod error;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

// 重新导出常用模块，方便外部使用
pub use error::Result;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册插件
        .plugin(tauri_plugin_opener::init())
        // 注册命令处理器
        .invoke_handler(tauri::generate_handler![
            handlers::dog_api::get_random_dog_image,
            handlers::anime::get_random_anime_quote, // 新增动漫一言命令
            handlers::anime::get_multiple_anime_quotes  // 新增多条动漫一言命令
        ])
        // 运行
        .run(tauri::generate_context!())
        // 处理错误
        .expect("error while running tauri application");
}
