//! 狗狗图片相关数据模型

use serde::{Deserialize, Serialize};

/// 狗狗图片API响应结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DogImageResponse {
    /// 图片URL
    pub message: String,
    /// 请求状态
    pub status: String,
}

impl DogImageResponse {
    /// 创建新的响应实例
    pub fn new(message: String, status: String) -> Self {
        Self { message, status }
    }

    /// 检查请求是否成功
    pub fn is_success(&self) -> bool {
        self.status == "success"
    }
}
