//! HTTP客户端工具 - 封装HTTP请求逻辑

use crate::Result;
use serde::de::DeserializeOwned;

/// HTTP客户端
pub struct HttpClient;

impl HttpClient {
    /// 发送GET请求并返回解析后的JSON数据
    ///
    /// # 类型参数
    /// - `T`: 响应数据的类型，必须实现Deserialize trait
    ///
    /// # 参数
    /// - `url`: 请求的URL
    ///
    /// # 返回
    /// - `Ok(T)`: 解析后的数据
    /// - `Err(AppError)`: 请求或解析错误
    pub async fn get<T>(url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // 创建HTTP客户端
        let client = reqwest::Client::new();

        // 发送GET请求
        let response = client.get(url).send().await?;

        // 检查HTTP状态码
        if !response.status().is_success() {
            return Err(crate::error::AppError {
                message: format!("HTTP请求失败: {}", response.status()),
            });
        }

        // 解析JSON响应
        let data: T = response.json().await?;

        Ok(data)
    }

    /// 发送GET请求并返回纯文本数据
    ///
    /// # 参数
    /// - `url`: 请求的URL
    ///
    /// # 返回
    /// - `Ok(String)`: 响应文本
    /// - `Err(AppError)`: 请求错误
    pub async fn get_text(url: &str) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(crate::error::AppError {
                message: format!("HTTP请求失败: {}", response.status()),
            });
        }

        let text = response.text().await?;
        Ok(text.trim().to_string())
    }
}
