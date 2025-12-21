// axum 完整应用实战示例
// 运行命令: cargo run --example axum_complete --features full

use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    id: u32,
    title: String,
    author: String,
    isbn: String,
    available: bool,
}

#[derive(Debug, Deserialize)]
struct CreateBook {
    title: String,
    author: String,
    isbn: String,
}

#[derive(Debug, Deserialize)]
struct UpdateBook {
    title: Option<String>,
    author: Option<String>,
    available: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
    author: Option<String>,
    available: Option<bool>,
}

// ============================================================================
// 应用状态
// ============================================================================

#[derive(Clone)]
struct AppState {
    books: Arc<RwLock<HashMap<u32, Book>>>,
    next_id: Arc<RwLock<u32>>,
}

impl AppState {
    fn new() -> Self {
        let mut books = HashMap::new();
        
        // 初始化一些示例数据
        books.insert(1, Book {
            id: 1,
            title: "Rust 程序设计语言".to_string(),
            author: "Steve Klabnik".to_string(),
            isbn: "978-1-59327-828-1".to_string(),
            available: true,
        });
        
        books.insert(2, Book {
            id: 2,
            title: "Rust 编程之道".to_string(),
            author: "张汉东".to_string(),
            isbn: "978-7-121-35898-5".to_string(),
            available: true,
        });
        
        books.insert(3, Book {
            id: 3,
            title: "深入浅出 Rust".to_string(),
            author: "范长春".to_string(),
            isbn: "978-7-111-61733-8".to_string(),
            available: false,
        });
        
        AppState {
            books: Arc::new(RwLock::new(books)),
            next_id: Arc::new(RwLock::new(4)),
        }
    }
}

// ============================================================================
// 错误处理
// ============================================================================

#[derive(Debug)]
enum ApiError {
    NotFound,
    BadRequest(String),
    Conflict(String),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "资源未找到".to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        let body = Json(serde_json::json!({
            "error": message,
        }));
        
        (status, body).into_response()
    }
}

// ============================================================================
// API 处理器
// ============================================================================

// 根路由 - API 信息
async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "图书管理 API",
        "version": "1.0.0",
        "description": "使用 Rust + axum 构建的图书管理系统",
        "endpoints": {
            "GET /": "API 信息",
            "GET /health": "健康检查",
            "GET /books": "获取所有图书",
            "GET /books/:id": "获取单个图书",
            "POST /books": "创建新图书",
            "PUT /books/:id": "更新图书",
            "DELETE /books/:id": "删除图书",
            "GET /books/search": "搜索图书",
            "GET /stats": "统计信息",
        }
    }))
}

// 健康检查
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

// 获取所有图书
async fn list_books(
    State(state): State<AppState>,
) -> Result<Json<Vec<Book>>, ApiError> {
    println!("📚 GET /books - 获取图书列表");
    
    let books = state.books.read()
        .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
    
    let book_list: Vec<Book> = books.values().cloned().collect();
    
    println!("   ✓ 返回 {} 本图书", book_list.len());
    Ok(Json(book_list))
}

// 获取单个图书
async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Book>, ApiError> {
    println!("📖 GET /books/{} - 获取单个图书", id);
    
    let books = state.books.read()
        .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
    
    let book = books.get(&id)
        .ok_or(ApiError::NotFound)?;
    
    println!("   ✓ 找到图书: {}", book.title);
    Ok(Json(book.clone()))
}

// 创建新图书
async fn create_book(
    State(state): State<AppState>,
    Json(payload): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), ApiError> {
    println!("➕ POST /books - 创建新图书");
    
    // 验证输入
    if payload.title.trim().is_empty() {
        return Err(ApiError::BadRequest("标题不能为空".to_string()));
    }
    if payload.author.trim().is_empty() {
        return Err(ApiError::BadRequest("作者不能为空".to_string()));
    }
    if payload.isbn.trim().is_empty() {
        return Err(ApiError::BadRequest("ISBN 不能为空".to_string()));
    }
    
    // 检查 ISBN 是否已存在
    {
        let books = state.books.read()
            .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
        
        for book in books.values() {
            if book.isbn == payload.isbn {
                return Err(ApiError::Conflict(
                    format!("ISBN {} 已存在", payload.isbn)
                ));
            }
        }
    }
    
    // 生成新 ID
    let id = {
        let mut next_id = state.next_id.write()
            .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
        let id = *next_id;
        *next_id += 1;
        id
    };
    
    // 创建新图书
    let book = Book {
        id,
        title: payload.title,
        author: payload.author,
        isbn: payload.isbn,
        available: true,
    };
    
    // 添加到列表
    {
        let mut books = state.books.write()
            .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
        books.insert(id, book.clone());
    }
    
    println!("   ✓ 创建成功: {} (ID: {})", book.title, book.id);
    Ok((StatusCode::CREATED, Json(book)))
}

// 更新图书
async fn update_book(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, ApiError> {
    println!("✏️  PUT /books/{} - 更新图书", id);
    
    let mut books = state.books.write()
        .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
    
    let book = books.get_mut(&id)
        .ok_or(ApiError::NotFound)?;
    
    // 更新字段
    if let Some(title) = payload.title {
        if title.trim().is_empty() {
            return Err(ApiError::BadRequest("标题不能为空".to_string()));
        }
        book.title = title;
    }
    
    if let Some(author) = payload.author {
        if author.trim().is_empty() {
            return Err(ApiError::BadRequest("作者不能为空".to_string()));
        }
        book.author = author;
    }
    
    if let Some(available) = payload.available {
        book.available = available;
    }
    
    println!("   ✓ 更新成功: {}", book.title);
    Ok(Json(book.clone()))
}

// 删除图书
async fn delete_book(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, ApiError> {
    println!("🗑️  DELETE /books/{} - 删除图书", id);
    
    let mut books = state.books.write()
        .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
    
    let book = books.remove(&id)
        .ok_or(ApiError::NotFound)?;
    
    println!("   ✓ 删除成功: {}", book.title);
    Ok(StatusCode::NO_CONTENT)
}

// 搜索图书
async fn search_books(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<Book>>, ApiError> {
    println!("🔍 GET /books/search - 搜索图书");
    
    let books = state.books.read()
        .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
    
    let results: Vec<Book> = books.values()
        .filter(|book| {
            // 关键词搜索
            if let Some(ref q) = params.q {
                let q_lower = q.to_lowercase();
                if !book.title.to_lowercase().contains(&q_lower) &&
                   !book.author.to_lowercase().contains(&q_lower) {
                    return false;
                }
            }
            
            // 作者过滤
            if let Some(ref author) = params.author {
                if !book.author.to_lowercase().contains(&author.to_lowercase()) {
                    return false;
                }
            }
            
            // 可用性过滤
            if let Some(available) = params.available {
                if book.available != available {
                    return false;
                }
            }
            
            true
        })
        .cloned()
        .collect();
    
    println!("   ✓ 找到 {} 本图书", results.len());
    Ok(Json(results))
}

// 统计信息
async fn stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    println!("📊 GET /stats - 获取统计信息");
    
    let books = state.books.read()
        .map_err(|e| ApiError::InternalError(format!("锁错误: {}", e)))?;
    
    let total = books.len();
    let available = books.values().filter(|b| b.available).count();
    let borrowed = total - available;
    
    // 按作者统计
    let mut authors: HashMap<String, u32> = HashMap::new();
    for book in books.values() {
        *authors.entry(book.author.clone()).or_insert(0) += 1;
    }
    
    let stats = serde_json::json!({
        "total_books": total,
        "available": available,
        "borrowed": borrowed,
        "authors": authors,
    });
    
    println!("   ✓ 统计: 总数={}, 可借={}, 已借={}", total, available, borrowed);
    Ok(Json(stats))
}

// ============================================================================
// 路由配置
// ============================================================================

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/books", get(list_books).post(create_book))
        .route("/books/search", get(search_books))
        .route("/books/:id", 
            get(get_book)
            .put(update_book)
            .delete(delete_book)
        )
        .route("/stats", get(stats))
}

fn app() -> Router {
    let state = AppState::new();
    
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
    println!("║      axum 完整应用实战示例                   ║");
    println!("║         图书管理系统 API                     ║");
    println!("╚══════════════════════════════════════════════╝\n");
    
    let app = app();
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 服务器启动成功！");
    println!("📍 地址: http://{}", addr);
    println!("\n📚 可用的 API 端点:");
    println!("   GET    /api/              - API 信息");
    println!("   GET    /api/health        - 健康检查");
    println!("   GET    /api/books         - 获取所有图书");
    println!("   GET    /api/books/:id     - 获取单个图书");
    println!("   POST   /api/books         - 创建新图书");
    println!("   PUT    /api/books/:id     - 更新图书");
    println!("   DELETE /api/books/:id     - 删除图书");
    println!("   GET    /api/books/search  - 搜索图书");
    println!("   GET    /api/stats         - 统计信息");
    println!("\n💡 示例请求:");
    println!("   # 获取所有图书");
    println!("   curl http://localhost:3000/api/books");
    println!("\n   # 搜索图书");
    println!("   curl 'http://localhost:3000/api/books/search?q=Rust'");
    println!("\n   # 创建新图书");
    println!("   curl -X POST http://localhost:3000/api/books \\");
    println!("        -H 'Content-Type: application/json' \\");
    println!("        -d '{{\"title\":\"新书\",\"author\":\"作者\",\"isbn\":\"123-456\"}}'");
    println!("\n   # 更新图书");
    println!("   curl -X PUT http://localhost:3000/api/books/1 \\");
    println!("        -H 'Content-Type: application/json' \\");
    println!("        -d '{{\"available\":false}}'");
    println!("\n   # 删除图书");
    println!("   curl -X DELETE http://localhost:3000/api/books/1");
    println!("\n   # 获取统计信息");
    println!("   curl http://localhost:3000/api/stats");
    println!("\n按 Ctrl+C 停止服务器\n");
    println!("{}", "=".repeat(60));
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
