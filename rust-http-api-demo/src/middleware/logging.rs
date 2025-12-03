use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

/// 日志中间件
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();
    
    tracing::info!(
        "收到请求: {} {}",
        method,
        uri
    );
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    tracing::info!(
        "响应完成: {} {} - {} ({:?})",
        method,
        uri,
        status.as_u16(),
        duration
    );
    
    response
}
