//! 狗狗图片服务 - 处理狗狗图片相关的业务逻辑

use crate::{models::DogImageResponse, utils::http_client::HttpClient, Result};

/// 狗狗图片服务
pub struct DogService;

impl DogService {
    /// 获取随机狗狗图片URL
    ///
    /// # 返回
    /// - `Ok(String)`: 图片URL
    /// - `Err(AppError)`: 错误信息
    ///
    /// # 示例
    /// ```rust
    /// let image_url = DogService::get_random_dog_image().await?;
    /// ```
    pub async fn get_random_dog_image() -> Result<String> {
        // 狗狗图片API端点
        let url = "https://dog.ceo/api/breeds/image/random";

        // 发送HTTP GET请求
        let response: DogImageResponse = HttpClient::get(url).await?;

        // 检查API响应状态
        if response.is_success() {
            Ok(response.message)
        } else {
            Err(crate::error::AppError {
                message: "获取狗狗图片失败".to_string(),
            })
        }
    }
}
