//! 动漫一言相关数据模型

use serde::Serialize;

/// 动漫一言响应
/// 由于这个API返回纯文本，我们只需要处理字符串
#[derive(Debug, Serialize, Clone)]
pub struct AnimeQuoteResponse {
    /// 动漫一言文本内容
    pub quote: String,
}

impl AnimeQuoteResponse {
    /// 创建新的动漫一言响应
    pub fn new(quote: String) -> Self {
        Self { quote }
    }

    /// 从原始文本创建响应
    pub fn from_text(text: String) -> Self {
        Self { quote: text }
    }
}
