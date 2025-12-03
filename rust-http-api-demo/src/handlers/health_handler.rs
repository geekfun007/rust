use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

/// 健康检查端点
pub async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "服务运行正常"
        })),
    )
}
