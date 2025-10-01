//! 统一错误处理模块

use serde::Serialize;

/// 应用错误类型
#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub message: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

// 从其他错误类型转换
impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError {
            message: format!("网络请求错误: {}", error),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError {
            message: format!("JSON解析错误: {}", error),
        }
    }
}

/// 应用结果类型别名
pub type Result<T> = std::result::Result<T, AppError>;
