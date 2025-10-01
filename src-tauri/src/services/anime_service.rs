//! 动漫一言服务 - 处理动漫一言相关的业务逻辑

use crate::{models::AnimeQuoteResponse, Result};

/// 动漫一言服务
pub struct AnimeService;

impl AnimeService {
    /// 获取随机动漫一言
    ///
    /// 这个API返回纯文本格式，所以我们需要直接处理字符串响应
    ///
    /// # 返回
    /// - `Ok(AnimeQuoteResponse)`: 包含动漫一言的响应
    /// - `Err(AppError)`: 错误信息
    ///
    /// # 示例
    /// ```rust
    /// let quote = AnimeService::get_random_quote().await?;
    /// ```
    pub async fn get_random_quote() -> Result<AnimeQuoteResponse> {
        // 动漫一言API端点 - 返回纯文本
        let url = "http://api.ziyi.site/anime-dailytxt";

        // 由于这个API返回纯文本而不是JSON，我们需要使用reqwest直接获取文本
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        // 检查HTTP状态码
        if !response.status().is_success() {
            return Err(crate::error::AppError {
                message: format!("动漫一言API请求失败: {}", response.status()),
            });
        }

        // 获取响应文本
        let text = response.text().await?;

        // 清理文本（移除可能的空白字符）
        let cleaned_text = text.trim().to_string();

        // 检查是否获取到有效内容
        if cleaned_text.is_empty() {
            return Err(crate::error::AppError {
                message: "获取到的动漫一言内容为空".to_string(),
            });
        }

        println!("成功获取动漫一言: {}", cleaned_text);
        Ok(AnimeQuoteResponse::from_text(cleaned_text))
    }

    /// 批量获取动漫一言
    ///
    /// # 参数
    /// - `count`: 要获取的条数
    ///
    /// # 返回
    /// - `Ok(Vec<AnimeQuoteResponse>)`: 动漫一言列表
    pub async fn get_multiple_quotes(count: usize) -> Result<Vec<AnimeQuoteResponse>> {
        let mut quotes = Vec::new();

        // 注意：实际使用时可能需要考虑API调用频率限制
        for _ in 0..count {
            match Self::get_random_quote().await {
                Ok(quote) => quotes.push(quote),
                Err(e) => eprintln!("获取动漫一言失败: {}", e),
                // 这里可以选择是遇到错误就终止还是继续获取
            }
        }

        if quotes.is_empty() {
            return Err(crate::error::AppError {
                message: "未能成功获取任何动漫一言".to_string(),
            });
        }

        Ok(quotes)
    }
}
