use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

/// 请求 ID 中间件
pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    
    // 将请求 ID 添加到请求扩展中
    request.extensions_mut().insert(request_id.clone());
    
    let mut response = next.run(request).await;
    
    // 将请求 ID 添加到响应头
    response.headers_mut().insert(
        "X-Request-ID",
        request_id.parse().unwrap(),
    );
    
    response
}
