// HTTP 服务器实战示例 - 完整的 REST API
// 运行命令: cargo run --example http_server_practical --features full

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tokio;

// ============================================================================
// 数据结构
// ============================================================================

/// Todo 项
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

/// 创建 Todo 请求
#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
    description: String,
}

/// 更新 Todo 请求
#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

/// 查询参数
#[derive(Debug, Deserialize)]
struct ListQuery {
    completed: Option<bool>,
    limit: Option<usize>,
}

/// 应用状态
#[derive(Debug)]
struct AppState {
    todos: Mutex<Vec<Todo>>,
    next_id: Mutex<u32>,
}

impl AppState {
    fn new() -> Self {
        // 初始化一些示例数据
        let todos = vec![
            Todo {
                id: 1,
                title: "学习 Rust".to_string(),
                description: "深入学习 Rust 编程语言".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "构建 Web API".to_string(),
                description: "使用 Axum 构建 REST API".to_string(),
                completed: false,
            },
            Todo {
                id: 3,
                title: "部署应用".to_string(),
                description: "将应用部署到生产环境".to_string(),
                completed: false,
            },
        ];
        
        AppState {
            todos: Mutex::new(todos),
            next_id: Mutex::new(4),
        }
    }
}

// ============================================================================
// 错误处理
// ============================================================================

/// 自定义错误类型
#[derive(Debug)]
enum ApiError {
    NotFound,
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "资源未找到".to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        let body = Json(serde_json::json!({
            "error": error_message,
        }));
        
        (status, body).into_response()
    }
}

// 为常见错误类型实现 From trait
impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, Vec<Todo>>>> for ApiError {
    fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, Vec<Todo>>>) -> Self {
        ApiError::InternalError("互斥锁中毒".to_string())
    }
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, u32>>> for ApiError {
    fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, u32>>) -> Self {
        ApiError::InternalError("互斥锁中毒".to_string())
    }
}

// ============================================================================
// API 处理器
// ============================================================================

/// 根路由 - API 信息
async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Todo REST API",
        "version": "1.0.0",
        "description": "使用 Rust + Axum 构建的 REST API 示例",
        "endpoints": {
            "GET /todos": "获取所有 Todo",
            "GET /todos/:id": "获取单个 Todo",
            "POST /todos": "创建新 Todo",
            "PUT /todos/:id": "更新 Todo",
            "DELETE /todos/:id": "删除 Todo",
        }
    }))
}

/// 健康检查
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// 获取所有 Todo
async fn list_todos(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListQuery>,
) -> Result<Json<Vec<Todo>>, ApiError> {
    println!("📋 GET /todos - 获取 Todo 列表");
    
    let todos = state.todos.lock()?;
    
    // 根据查询参数过滤
    let mut filtered: Vec<Todo> = todos
        .iter()
        .filter(|todo| {
            if let Some(completed) = params.completed {
                todo.completed == completed
            } else {
                true
            }
        })
        .cloned()
        .collect();
    
    // 限制返回数量
    if let Some(limit) = params.limit {
        filtered.truncate(limit);
    }
    
    println!("   ✓ 返回 {} 个 Todo", filtered.len());
    Ok(Json(filtered))
}

/// 获取单个 Todo
async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, ApiError> {
    println!("📄 GET /todos/{} - 获取单个 Todo", id);
    
    let todos = state.todos.lock()?;
    
    let todo = todos
        .iter()
        .find(|t| t.id == id)
        .ok_or(ApiError::NotFound)?;
    
    println!("   ✓ 找到 Todo: {}", todo.title);
    Ok(Json(todo.clone()))
}

/// 创建新 Todo
async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    println!("➕ POST /todos - 创建新 Todo");
    
    // 验证输入
    if payload.title.trim().is_empty() {
        return Err(ApiError::BadRequest("标题不能为空".to_string()));
    }
    
    // 生成新 ID
    let mut next_id = state.next_id.lock()?;
    let id = *next_id;
    *next_id += 1;
    drop(next_id); // 释放锁
    
    // 创建新 Todo
    let todo = Todo {
        id,
        title: payload.title,
        description: payload.description,
        completed: false,
    };
    
    // 添加到列表
    let mut todos = state.todos.lock()?;
    todos.push(todo.clone());
    
    println!("   ✓ 创建成功: {} (ID: {})", todo.title, todo.id);
    Ok((StatusCode::CREATED, Json(todo)))
}

/// 更新 Todo
async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    println!("✏️  PUT /todos/{} - 更新 Todo", id);
    
    let mut todos = state.todos.lock()?;
    
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(ApiError::NotFound)?;
    
    // 更新字段
    if let Some(title) = payload.title {
        if title.trim().is_empty() {
            return Err(ApiError::BadRequest("标题不能为空".to_string()));
        }
        todo.title = title;
    }
    
    if let Some(description) = payload.description {
        todo.description = description;
    }
    
    if let Some(completed) = payload.completed {
        todo.completed = completed;
    }
    
    println!("   ✓ 更新成功: {}", todo.title);
    Ok(Json(todo.clone()))
}

/// 删除 Todo
async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<StatusCode, ApiError> {
    println!("🗑️  DELETE /todos/{} - 删除 Todo", id);
    
    let mut todos = state.todos.lock()?;
    
    let index = todos
        .iter()
        .position(|t| t.id == id)
        .ok_or(ApiError::NotFound)?;
    
    let removed = todos.remove(index);
    
    println!("   ✓ 删除成功: {}", removed.title);
    Ok(StatusCode::NO_CONTENT)
}

/// 统计信息
async fn stats(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, ApiError> {
    println!("📊 GET /stats - 获取统计信息");
    
    let todos = state.todos.lock()?;
    
    let total = todos.len();
    let completed = todos.iter().filter(|t| t.completed).count();
    let pending = total - completed;
    
    let stats = serde_json::json!({
        "total": total,
        "completed": completed,
        "pending": pending,
        "completion_rate": if total > 0 {
            (completed as f64 / total as f64 * 100.0).round()
        } else {
            0.0
        }
    });
    
    println!("   ✓ 统计: 总数={}, 完成={}, 待办={}", total, completed, pending);
    Ok(Json(stats))
}

// ============================================================================
// 路由配置
// ============================================================================

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .route("/stats", get(stats))
}

fn app() -> Router {
    // 创建共享状态
    let state = Arc::new(AppState::new());
    
    Router::new()
        .nest("/api", api_routes())
        .with_state(state)
}

// ============================================================================
// 主函数
// ============================================================================

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║      Rust HTTP 服务器实战示例               ║");
    println!("║         完整的 REST API                      ║");
    println!("╚══════════════════════════════════════════════╝\n");
    
    let app = app();
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 服务器启动成功！");
    println!("📍 地址: http://{}", addr);
    println!("\n📚 可用的 API 端点:");
    println!("   GET    /api/              - API 信息");
    println!("   GET    /api/health        - 健康检查");
    println!("   GET    /api/todos         - 获取所有 Todo");
    println!("   GET    /api/todos/:id     - 获取单个 Todo");
    println!("   POST   /api/todos         - 创建新 Todo");
    println!("   PUT    /api/todos/:id     - 更新 Todo");
    println!("   DELETE /api/todos/:id     - 删除 Todo");
    println!("   GET    /api/stats         - 统计信息");
    println!("\n💡 示例请求:");
    println!("   curl http://localhost:3000/api/");
    println!("   curl http://localhost:3000/api/todos");
    println!("   curl -X POST http://localhost:3000/api/todos \\");
    println!("        -H 'Content-Type: application/json' \\");
    println!("        -d '{{\"title\":\"新任务\",\"description\":\"描述\"}}'");
    println!("\n按 Ctrl+C 停止服务器\n");
    println!("{}", "=".repeat(60));
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`
    use serde_json::json;
    
    #[tokio::test]
    async fn test_root() {
        let app = app();
        
        let response = app
            .oneshot(Request::builder().uri("/api/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let app = app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_list_todos() {
        let app = app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/todos")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_get_todo() {
        let app = app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/todos/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_get_todo_not_found() {
        let app = app();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/todos/9999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    
    #[tokio::test]
    async fn test_create_todo() {
        let app = app();
        
        let new_todo = json!({
            "title": "测试任务",
            "description": "这是一个测试任务"
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/todos")
                    .header("content-type", "application/json")
                    .body(Body::from(new_todo.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
