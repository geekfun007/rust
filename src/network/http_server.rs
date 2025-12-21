// HTTP 服务器模块
//
// 本模块提供 HTTP 服务器功能的说明和示例
// 完整的实战代码请参考: examples/http_server_practical.rs 和 examples/axum_complete.rs

/// HTTP 服务器概述
///
/// Rust 生态系统中主要的 Web 框架包括:
/// - axum: 现代、类型安全、基于 tokio
/// - actix-web: 高性能
/// - rocket: 易用性
/// - warp: 函数式风格
pub mod overview {
    pub const DESCRIPTION: &str = r#"
axum 是基于 tokio 的现代 Web 框架

核心特性:
- 类型安全的提取器
- 灵活的路由系统
- 中间件支持
- WebSocket 支持
- 基于 hyper 的高性能
- 优秀的编译时错误检查
"#;
}

/// 基础用法示例
pub mod basics {
    /// 最简单的服务器
    pub const BASIC_SERVER: &str = r#"
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
"#;

    /// 路由示例
    pub const ROUTING_EXAMPLE: &str = r#"
use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};

let app = Router::new()
    .route("/", get(index))
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(get_user));

async fn index() -> &'static str {
    "Home"
}

async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}
"#;
}

/// 提取器使用
pub mod extractors {
    /// 常用提取器示例
    pub const EXTRACTORS_EXAMPLE: &str = r#"
use axum::{
    extract::{Path, Query, Json, State},
    routing::get,
};
use serde::{Deserialize, Serialize};

// 路径参数
async fn handler1(Path(id): Path<u32>) {}

// 查询参数
#[derive(Deserialize)]
struct Params {
    page: Option<u32>,
}
async fn handler2(Query(params): Query<Params>) {}

// JSON 请求体
#[derive(Deserialize)]
struct CreateUser {
    name: String,
}
async fn handler3(Json(user): Json<CreateUser>) {}

// 应用状态
async fn handler4(State(state): State<Arc<AppState>>) {}
"#;
}

/// JSON 处理
pub mod json {
    /// JSON 请求/响应示例
    pub const JSON_EXAMPLE: &str = r#"
use axum::{
    extract::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 接收 JSON
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    
    (StatusCode::CREATED, Json(user))
}

// 返回 JSON
async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    })
}
"#;
}

/// 状态管理
pub mod state {
    /// 状态共享示例
    pub const STATE_EXAMPLE: &str = r#"
use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<u32>>,
}

async fn increment(State(state): State<AppState>) -> String {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    format!("Counter: {}", counter)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };
    
    let app = Router::new()
        .route("/increment", get(increment))
        .with_state(state);
    
    // 启动服务器...
}
"#;
}

/// 中间件
pub mod middleware {
    /// 中间件示例
    pub const MIDDLEWARE_EXAMPLE: &str = r#"
use axum::{
    middleware::{self, Next},
    http::Request,
    response::Response,
};

// 自定义中间件
async fn log_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    println!("Request: {} {}", request.method(), request.uri());
    let response = next.run(request).await;
    println!("Response: {}", response.status());
    response
}

// 应用中间件
let app = Router::new()
    .route("/", get(handler))
    .layer(middleware::from_fn(log_middleware));
"#;

    /// Tower 中间件
    pub const TOWER_MIDDLEWARE: &str = r#"
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};

let app = Router::new()
    .route("/", get(handler))
    // 日志
    .layer(TraceLayer::new_for_http())
    // CORS
    .layer(CorsLayer::permissive());
"#;
}

/// 错误处理
pub mod error_handling {
    /// 错误处理示例
    pub const ERROR_EXAMPLE: &str = r#"
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// 自定义错误类型
enum ApiError {
    NotFound,
    BadRequest(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };
        
        (status, Json(serde_json::json!({
            "error": message
        }))).into_response()
    }
}

// 使用自定义错误
async fn handler() -> Result<Json<User>, ApiError> {
    let user = find_user(1)
        .ok_or(ApiError::NotFound)?;
    
    Ok(Json(user))
}
"#;
}

/// WebSocket 支持
pub mod websocket {
    /// WebSocket 示例
    pub const WEBSOCKET_EXAMPLE: &str = r#"
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    // 处理文本消息
                    if socket.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    }
}

let app = Router::new()
    .route("/ws", get(ws_handler));
"#;
}

/// 运行示例和文档
pub fn print_documentation() {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║       Rust HTTP 服务器 (axum)             ║");
    println!("╚════════════════════════════════════════════╝\n");
    
    println!("{}", overview::DESCRIPTION);
    
    println!("\n📖 详细文档:");
    println!("   docs/HTTP_PRACTICAL_GUIDE.md");
    println!("   docs/REQWEST_AXUM_COMPLETE.md");
    println!("   docs/HTTP_CHEATSHEET.md");
    
    println!("\n🚀 完整示例:");
    println!("   examples/http_server_practical.rs");
    println!("   examples/axum_complete.rs");
    println!("   examples/websocket_practical.rs");
    
    println!("\n▶️  运行示例:");
    println!("   cargo run --example http_server_practical --features full");
    println!("   cargo run --example axum_complete --features full");
    println!("   cargo run --example websocket_practical --features full");
    
    println!("\n💡 快速开始:");
    println!("{}", basics::BASIC_SERVER);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_documentation_exists() {
        assert!(!overview::DESCRIPTION.is_empty());
        assert!(!basics::BASIC_SERVER.is_empty());
        assert!(!basics::ROUTING_EXAMPLE.is_empty());
    }
}
