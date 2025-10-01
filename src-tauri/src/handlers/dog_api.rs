//! 狗狗图片API相关的Tauri命令

use crate::services::DogService;

/// 获取随机狗狗图片的Tauri命令
///
/// 这个命令会被前端Vue调用，返回一个随机的狗狗图片URL
///
/// # 返回
/// - `Ok(String)`: 成功的狗狗图片URL
/// - `Err(String)`: 错误信息字符串（Tauri要求错误类型为String）
///
/// # 前端调用示例 (Vue)
/// ```typescript
/// import { invoke } from "@tauri-apps/api/core";
///
/// const dogImageUrl = await invoke<string>('get_random_dog_image');
/// ```
#[tauri::command]
pub async fn get_random_dog_image() -> std::result::Result<String, String> {
    // 调用服务层获取狗狗图片
    match DogService::get_random_dog_image().await {
        Ok(image_url) => {
            println!("成功获取狗狗图片: {}", image_url);
            Ok(image_url)
        }
        Err(e) => {
            // 记录错误日志
            eprintln!("获取狗狗图片失败: {}", e);
            // 返回错误信息给前端
            Err(e.to_string())
        }
    }
}
