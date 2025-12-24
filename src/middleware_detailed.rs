// Axum 中间件 & IntoResponse 详解
//
// 本教程涵盖 Axum 中间件系统和响应类型的所有核心概念

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    println!("=== Axum 中间件 & IntoResponse 详解 ===\n");
    
    // 1. 中间件基础
    demo_middleware_basics();
    
    // 2. IntoResponse 详解
    demo_into_response();
    
    // 3. 自定义响应类型
    demo_custom_response();
    
    // 4. 中间件实战
    demo_middleware_examples();
    
    // 5. Tower 中间件
    demo_tower_middleware();
    
    println!("\n启动服务器示例...");
    println!("访问 http://localhost:3000 测试中间件");
    
    // 启动示例服务器
    run_example_server().await;
}

// ============================================
// 1. 中间件基础
// ============================================
fn demo_middleware_basics() {
    println!("--- 1. 中间件基础 ---\n");
    
    println!("什么是中间件？");
    println!("  - 在请求到达处理器之前/之后执行的代码");
    println!("  - 可以修改请求和响应");
    println!("  - 可以短路请求处理");
    println!("  - 形成处理链\n");
    
    println!("中间件类型:");
    println!("  📌 Axum 中间件    - axum::middleware");
    println!("  📌 Tower 中间件   - tower::ServiceBuilder");
    println!("  📌 Layer 中间件   - 可组合的层\n");
    
    println!("使用场景:");
    println!("  ✓ 日志记录");
    println!("  ✓ 认证授权");
    println!("  ✓ 错误处理");
    println!("  ✓ CORS");
    println!("  ✓ 请求限流");
    println!("  ✓ 响应压缩\n");
}

// ============================================
// 2. IntoResponse 详解
// ============================================
fn demo_into_response() {
    println!("--- 2. IntoResponse 详解 ---\n");
    
    println!("什么是 IntoResponse？");
    println!("  - Trait，将类型转换为 HTTP 响应");
    println!("  - Axum 处理器的返回类型必须实现它");
    println!("  - 提供灵活的响应方式\n");
    
    println!("标准实现:");
    println!("  &str             - 纯文本");
    println!("  String           - 字符串");
    println!("  Html<T>          - HTML");
    println!("  Json<T>          - JSON");
    println!("  (StatusCode, T)  - 状态码 + 内容");
    println!("  (HeaderMap, T)   - 请求头 + 内容");
    println!("  Response         - 完整响应");
    println!("  Result<T, E>     - 错误处理\n");
}

// ============================================
// 3. 自定义响应类型
// ============================================
fn demo_custom_response() {
    println!("--- 3. 自定义响应类型 ---\n");
    
    println!("实现 IntoResponse:");
    println!("  1. 定义响应类型");
    println!("  2. 实现 IntoResponse trait");
    println!("  3. 在处理器中使用\n");
    
    println!("示例代码:");
    println!("  struct ApiResponse {{ ... }}");
    println!("  impl IntoResponse for ApiResponse {{ ... }}");
    println!();
}

// ============================================
// 4. 中间件实战
// ============================================
fn demo_middleware_examples() {
    println!("--- 4. 中间件实战 ---\n");
    
    println!("常用中间件:");
    println!("  1. 日志中间件");
    println!("  2. 认证中间件");
    println!("  3. 错误处理中间件");
    println!("  4. CORS 中间件");
    println!("  5. 超时中间件\n");
}

// ============================================
// 5. Tower 中间件
// ============================================
fn demo_tower_middleware() {
    println!("--- 5. Tower 中间件 ---\n");
    
    println!("Tower ServiceBuilder:");
    println!("  - 组合多个中间件");
    println!("  - 按顺序执行");
    println!("  - 类型安全\n");
}

// ============================================
// 实战示例：完整的中间件系统
// ============================================

// 自定义响应类型
#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    timestamp: u64,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

// 实现 IntoResponse
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        
        (status, Json(self)).into_response()
    }
}

// 自定义错误类型
enum AppError {
    NotFound(String),
    Unauthorized,
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            ),
        };
        
        let body = ApiResponse::<()>::error(message);
        (status, Json(body)).into_response()
    }
}

// 中间件 1: 日志中间件
async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    println!("[LOG] {} {}", method, uri);
    
    let start = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    println!("[LOG] {} {} - {:?}", method, uri, duration);
    
    response
}

// 中间件 2: 认证中间件
async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 检查 Authorization header
    match headers.get("authorization") {
        Some(auth_header) => {
            let auth_str = auth_header.to_str().unwrap_or("");
            
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                
                // 验证 token（简化示例）
                if token == "valid-token-123" {
                    Ok(next.run(request).await)
                } else {
                    Err(AppError::Unauthorized)
                }
            } else {
                Err(AppError::BadRequest(
                    "Invalid authorization format".to_string(),
                ))
            }
        }
        None => Err(AppError::Unauthorized),
    }
}

// 中间件 3: 请求 ID 中间件
async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // 生成请求 ID
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // 添加到请求扩展
    request.extensions_mut().insert(RequestId(request_id.clone()));
    
    // 执行下一个中间件
    let mut response = next.run(request).await;
    
    // 添加到响应头
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    response
}

// 请求 ID 类型
#[derive(Clone)]
struct RequestId(String);

// 中间件 4: 错误处理中间件
async fn error_handling_middleware(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    // 检查响应状态
    if response.status().is_server_error() {
        println!("[ERROR] Server error: {}", response.status());
    }
    
    response
}

// 中间件 5: 超时中间件
async fn timeout_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let timeout = Duration::from_secs(30);
    
    match tokio::time::timeout(timeout, next.run(request)).await {
        Ok(response) => Ok(response),
        Err(_) => Err(AppError::InternalError(
            "Request timeout".to_string(),
        )),
    }
}

// ============================================
// 处理器示例
// ============================================

// 基础处理器
async fn root_handler() -> &'static str {
    "Welcome to Middleware Demo!"
}

// JSON 响应处理器
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn get_user() -> impl IntoResponse {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    ApiResponse::success(user)
}

// 错误处理器
async fn error_handler() -> Result<String, AppError> {
    Err(AppError::NotFound("Resource not found".to_string()))
}

// 受保护的路由
async fn protected_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "This is a protected route"
    }))
}

// 多种响应类型示例
async fn multi_response_handler() -> impl IntoResponse {
    // 可以返回不同的响应类型
    (
        StatusCode::CREATED,
        [("X-Custom-Header", "custom-value")],
        Json(serde_json::json!({
            "status": "created"
        })),
    )
}

// ============================================
// 服务器配置
// ============================================

async fn run_example_server() {
    // 公开路由（不需要认证）
    let public_routes = Router::new()
        .route("/", get(root_handler))
        .route("/user", get(get_user))
        .route("/error", get(error_handler));
    
    // 受保护路由（需要认证）
    let protected_routes = Router::new()
        .route("/protected", get(protected_route))
        .route("/multi", get(multi_response_handler))
        .layer(middleware::from_fn(auth_middleware));
    
    // 组合所有路由
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // 应用中间件（按顺序执行）
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn(logging_middleware))
        .layer(middleware::from_fn(error_handling_middleware))
        // Tower 中间件
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new().allow_origin(Any))
        );
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("服务器运行在 http://127.0.0.1:3000");
    println!();
    println!("测试端点:");
    println!("  GET  /              - 根路径");
    println!("  GET  /user          - 获取用户");
    println!("  GET  /error         - 错误示例");
    println!("  GET  /protected     - 受保护路由（需要 token）");
    println!("  GET  /multi         - 多响应类型");
    println!();
    println!("测试命令:");
    println!("  curl http://localhost:3000/");
    println!("  curl http://localhost:3000/user");
    println!("  curl http://localhost:3000/protected -H 'Authorization: Bearer valid-token-123'");
    println!();
    
    axum::serve(listener, app).await.unwrap();
}

// 用于生成 UUID 的简化实现
mod uuid {
    pub struct Uuid;
    
    impl Uuid {
        pub fn new_v4() -> Self {
            Uuid
        }
        
        pub fn to_string(&self) -> String {
            format!("req-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis())
        }
    }
}

/*
=== 总结 ===

1. 中间件核心概念:

   执行顺序:
   Request → Middleware 1 → Middleware 2 → Handler
          ← Response 1    ← Response 2    ← Response
   
   类型:
   - Axum 中间件: from_fn()
   - Tower 中间件: ServiceBuilder
   - Layer 中间件: 可组合

2. IntoResponse:

   标准类型:
   - &str, String          - 文本
   - Json<T>               - JSON
   - Html<T>               - HTML
   - (StatusCode, T)       - 状态码
   - (HeaderMap, T)        - 请求头
   - Response              - 完整响应
   
   自定义:
   - 实现 IntoResponse trait
   - 灵活的响应格式
   - 错误类型转换

3. 中间件模式:

   日志:
   - 记录请求/响应
   - 性能监控
   
   认证:
   - Token 验证
   - 权限检查
   
   错误处理:
   - 统一错误格式
   - 错误恢复
   
   CORS:
   - 跨域支持
   - 安全配置

4. 最佳实践:

   DO:
   ✓ 使用类型安全的中间件
   ✓ 合理的中间件顺序
   ✓ 实现自定义响应类型
   ✓ 统一错误处理
   
   DON'T:
   ✗ 在中间件中阻塞
   ✗ 过多的中间件层
   ✗ 忽略错误处理

运行示例:
  cargo run --bin middleware_detailed
  
  然后在另一个终端测试:
  curl http://localhost:3000/
  curl http://localhost:3000/user
  curl http://localhost:3000/protected \
    -H 'Authorization: Bearer valid-token-123'
*/
