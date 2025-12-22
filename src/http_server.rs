// Rust HTTP Server 详解 - 使用 Axum 框架
// 
// Axum 是基于 tokio 的高性能异步 web 框架

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;

// ============================================
// 数据模型
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    page: Option<u32>,
    limit: Option<u32>,
}

// 应用状态（共享数据）
#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<u32, User>>>,
    next_id: Arc<Mutex<u32>>,
}

// ============================================
// 主函数
// ============================================

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    println!("=== Rust HTTP Server 实战（Axum） ===\n");
    
    // 创建共享状态
    let state = AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
        next_id: Arc::new(Mutex::new(1)),
    };
    
    // 添加一些初始数据
    {
        let mut users = state.users.lock().unwrap();
        users.insert(1, User {
            id: 1,
            name: "张三".to_string(),
            email: "zhangsan@example.com".to_string(),
            age: 25,
        });
        users.insert(2, User {
            id: 2,
            name: "李四".to_string(),
            email: "lisi@example.com".to_string(),
            age: 30,
        });
        *state.next_id.lock().unwrap() = 3;
    }
    
    // 构建路由
    let app = Router::new()
        // 基础路由
        .route("/", get(root))
        .route("/health", get(health_check))
        
        // RESTful API
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", get(get_user).delete(delete_user))
        
        // 查询参数示例
        .route("/api/search", get(search_users))
        
        // JSON 请求体示例
        .route("/api/users/:id/update", post(update_user))
        
        // 错误处理示例
        .route("/api/error", get(error_example))
        
        // 添加状态
        .with_state(state)
        
        // 添加 CORS 中间件
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );
    
    // 启动服务器
    let addr = "127.0.0.1:3000";
    println!("🚀 服务器启动成功！");
    println!("📡 监听地址: http://{}", addr);
    println!("\n可用端点:");
    println!("  GET    /                    - 首页");
    println!("  GET    /health              - 健康检查");
    println!("  GET    /api/users           - 获取所有用户");
    println!("  GET    /api/users/:id       - 获取单个用户");
    println!("  POST   /api/users           - 创建用户");
    println!("  DELETE /api/users/:id       - 删除用户");
    println!("  POST   /api/users/:id/update - 更新用户");
    println!("  GET    /api/search          - 搜索用户（带查询参数）");
    println!("  GET    /api/error           - 错误示例");
    println!("\n按 Ctrl+C 停止服务器\n");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ============================================
// 路由处理函数
// ============================================

// 1. 基础路由 - 返回 HTML
async fn root() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Rust HTTP Server</title>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            max-width: 800px; 
            margin: 50px auto; 
            padding: 20px;
            background: #f5f5f5;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 { color: #333; }
        .endpoint { 
            background: #f0f0f0; 
            padding: 10px; 
            margin: 10px 0; 
            border-radius: 5px;
            font-family: monospace;
        }
        .method { 
            color: white; 
            padding: 3px 8px; 
            border-radius: 3px; 
            font-size: 12px;
            font-weight: bold;
        }
        .get { background: #61affe; }
        .post { background: #49cc90; }
        .delete { background: #f93e3e; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🦀 Rust HTTP Server (Axum)</h1>
        <p>欢迎使用基于 Axum 的高性能 HTTP 服务器！</p>
        
        <h2>📋 可用端点：</h2>
        <div class="endpoint">
            <span class="method get">GET</span> /health - 健康检查
        </div>
        <div class="endpoint">
            <span class="method get">GET</span> /api/users - 获取所有用户
        </div>
        <div class="endpoint">
            <span class="method get">GET</span> /api/users/:id - 获取单个用户
        </div>
        <div class="endpoint">
            <span class="method post">POST</span> /api/users - 创建新用户
        </div>
        <div class="endpoint">
            <span class="method delete">DELETE</span> /api/users/:id - 删除用户
        </div>
        <div class="endpoint">
            <span class="method post">POST</span> /api/users/:id/update - 更新用户
        </div>
        <div class="endpoint">
            <span class="method get">GET</span> /api/search?page=1&limit=10 - 搜索
        </div>
        
        <h3>🧪 测试命令：</h3>
        <pre>
# 获取所有用户
curl http://localhost:3000/api/users

# 创建用户
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"王五","email":"wangwu@example.com","age":28}'

# 获取单个用户
curl http://localhost:3000/api/users/1

# 删除用户
curl -X DELETE http://localhost:3000/api/users/1
        </pre>
    </div>
</body>
</html>
    "#;
    
    (StatusCode::OK, [("Content-Type", "text/html")], html)
}

// 2. 健康检查
async fn health_check() -> impl IntoResponse {
    #[derive(Serialize)]
    struct HealthResponse {
        status: String,
        message: String,
    }
    
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "服务器运行正常".to_string(),
    })
}

// 3. 获取所有用户 - State 提取器
async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.lock().unwrap();
    let user_list: Vec<User> = users.values().cloned().collect();
    
    Json(user_list)
}

// 4. 获取单个用户 - Path 参数提取器
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let users = state.users.lock().unwrap();
    
    match users.get(&id) {
        Some(user) => (StatusCode::OK, Json(user.clone())).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "用户未找到",
                "id": id
            }))
        ).into_response(),
    }
}

// 5. 创建用户 - JSON 请求体提取器
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // 生成新 ID
    let id = {
        let mut next_id = state.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    };
    
    // 创建用户
    let user = User {
        id,
        name: payload.name,
        email: payload.email,
        age: payload.age,
    };
    
    // 保存到状态
    {
        let mut users = state.users.lock().unwrap();
        users.insert(id, user.clone());
    }
    
    (StatusCode::CREATED, Json(user))
}

// 6. 删除用户
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut users = state.users.lock().unwrap();
    
    match users.remove(&id) {
        Some(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "用户已删除",
                "id": id
            }))
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "用户未找到",
                "id": id
            }))
        ).into_response(),
    }
}

// 7. 更新用户
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let mut users = state.users.lock().unwrap();
    
    match users.get_mut(&id) {
        Some(user) => {
            user.name = payload.name;
            user.email = payload.email;
            user.age = payload.age;
            
            (StatusCode::OK, Json(user.clone())).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "用户未找到",
                "id": id
            }))
        ).into_response(),
    }
}

// 8. 查询参数示例
async fn search_users(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    
    let users = state.users.lock().unwrap();
    let user_list: Vec<User> = users.values().cloned().collect();
    
    let total = user_list.len();
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(total);
    
    let paginated = user_list.get(start..end).unwrap_or(&[]).to_vec();
    
    Json(serde_json::json!({
        "data": paginated,
        "page": page,
        "limit": limit,
        "total": total,
    }))
}

// 9. 错误处理示例
async fn error_example() -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // 模拟错误
    let success = false;
    
    if success {
        Ok(Json(serde_json::json!({
            "message": "成功"
        })))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "内部服务器错误",
                "code": 500
            }))
        ))
    }
}

// ============================================
// 高级示例说明
// ============================================

/*
更多 Axum 特性：

1. 中间件（Middleware）
   - tower-http 提供了很多实用中间件
   - 日志、CORS、压缩、限流等

2. 提取器（Extractors）
   - Path: 路径参数
   - Query: 查询参数
   - Json: JSON 请求体
   - State: 应用状态
   - Extension: 扩展数据
   - Header: HTTP 头部

3. 响应类型
   - Json: JSON 响应
   - Html: HTML 响应
   - StatusCode: 状态码
   - Redirect: 重定向
   - Stream: 流式响应

4. 错误处理
   - 自定义错误类型
   - IntoResponse trait
   - 中间件错误处理

5. WebSocket 支持
   use axum::extract::ws::{WebSocket, WebSocketUpgrade};

6. 文件上传
   use axum::extract::Multipart;

7. SSE (Server-Sent Events)
   use axum::response::sse::Event;

8. 测试
   - axum::http::Request
   - tower::ServiceExt

运行示例：
  cargo run --bin http_server

测试命令：
  # 获取所有用户
  curl http://localhost:3000/api/users
  
  # 创建用户
  curl -X POST http://localhost:3000/api/users \
    -H "Content-Type: application/json" \
    -d '{"name":"Test","email":"test@example.com","age":25}'
  
  # 获取单个用户
  curl http://localhost:3000/api/users/1
  
  # 带查询参数
  curl "http://localhost:3000/api/search?page=1&limit=5"
*/
