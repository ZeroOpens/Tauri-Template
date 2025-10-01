//! 动漫一言相关的Tauri命令

use crate::{models::AnimeQuoteResponse, services::AnimeService};

/// 获取随机动漫一言的Tauri命令
///
/// 这个命令会被前端Vue调用，返回一个随机的动漫一言文本
///
/// # 返回
/// - `Ok(AnimeQuoteResponse)`: 包含动漫一言的响应
/// - `Err(String)`: 错误信息字符串
///
/// # 前端调用示例 (Vue)
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const quoteData = await invoke<{ quote: string }>('get_random_anime_quote');
/// console.log(quoteData.quote);
/// ```
#[tauri::command]
pub async fn get_random_anime_quote() -> std::result::Result<AnimeQuoteResponse, String> {
    // 调用服务层获取动漫一言
    match AnimeService::get_random_quote().await {
        Ok(quote_response) => {
            println!("成功获取动漫一言: {}", quote_response.quote);
            Ok(quote_response)
        }
        Err(e) => {
            // 记录错误日志
            eprintln!("获取动漫一言失败: {}", e);
            // 返回错误信息给前端
            Err(e.to_string())
        }
    }
}

/// 获取多条动漫一言的Tauri命令
///
/// # 参数
/// - `count`: 要获取的条数
///
/// # 返回
/// - `Ok(Vec<AnimeQuoteResponse>)`: 动漫一言列表
/// - `Err(String)`: 错误信息
#[tauri::command]
pub async fn get_multiple_anime_quotes(
    count: usize,
) -> std::result::Result<Vec<AnimeQuoteResponse>, String> {
    // 限制最大数量，防止滥用
    let actual_count = count.min(10); // 最多10条

    match AnimeService::get_multiple_quotes(actual_count).await {
        Ok(quotes) => {
            println!("成功获取 {} 条动漫一言", quotes.len());
            Ok(quotes)
        }
        Err(e) => {
            eprintln!("获取多条动漫一言失败: {}", e);
            Err(e.to_string())
        }
    }
}
