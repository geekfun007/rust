use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// 应用错误类型
#[derive(Debug)]
pub enum AppError {
    /// 数据库错误
    DatabaseError(sqlx::Error),
    /// 未找到资源
    NotFound(String),
    /// 验证错误
    ValidationError(String),
    /// 内部错误
    InternalError(String),
    /// 请求错误
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "数据库错误: {}", e),
            AppError::NotFound(msg) => write!(f, "未找到: {}", msg),
            AppError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            AppError::InternalError(msg) => write!(f, "内部错误: {}", msg),
            AppError::BadRequest(msg) => write!(f, "请求错误: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::NotFound("记录未找到".to_string()),
            _ => AppError::DatabaseError(error),
        }
    }
}

// 将 AppError 转换为 HTTP 响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(ref e) => {
                tracing::error!("数据库错误: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误")
            }
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::ValidationError(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalError(ref msg) => {
                tracing::error!("内部错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str())
            }
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };

        let body = Json(json!({
            "error": error_message,
            "details": self.to_string(),
        }));

        (status, body).into_response()
    }
}

/// 应用 Result 类型别名
pub type AppResult<T> = Result<T, AppError>;
