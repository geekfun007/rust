# HTTP 客户端/服务器 速查表 (Cheatsheet)

## HTTP 客户端 (reqwest)

### 基础请求

```rust
// GET 请求
let response = reqwest::get("https://api.example.com/data").await?;
let text = response.text().await?;

// POST JSON
let client = reqwest::Client::new();
let response = client
    .post("https://api.example.com/users")
    .json(&user_data)
    .send()
    .await?;
```

### 客户端配置

```rust
use std::time::Duration;

let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))        // 总超时
    .connect_timeout(Duration::from_secs(10)) // 连接超时
    .pool_max_idle_per_host(25)              // 连接池
    .http2_prior_knowledge()                 // HTTP/2
    .gzip(true)                              // 压缩
    .cookie_store(true)                      // Cookie
    .build()?;
```

### 常用操作

```rust
// 查询参数
.query(&[("key", "value"), ("page", "1")])

// 请求头
.header("Authorization", "Bearer token")
.header("User-Agent", "MyApp/1.0")

// 表单提交
.form(&[("username", "alice"), ("password", "secret")])

// 文件上传
use reqwest::multipart;
let form = multipart::Form::new()
    .text("name", "file.jpg")
    .file("file", "/path/to/file").await?;
.multipart(form)

// 错误处理
.send().await?.error_for_status()?
```

### 并发请求

```rust
use futures::future::join_all;

let futures = urls.iter().map(|url| {
    client.get(url).send()
});
let results = join_all(futures).await;
```

---

## HTTP 服务器 (Axum)

### 基础服务器

```rust
use axum::{routing::get, Router};

let app = Router::new()
    .route("/", get(handler));

axum::Server::bind(&"0.0.0.0:3000".parse()?)
    .serve(app.into_make_service())
    .await?;
```

### 路由

```rust
use axum::{extract::Path, routing::{get, post}};

Router::new()
    // 基础路由
    .route("/", get(index))
    // 路径参数
    .route("/users/:id", get(get_user))
    // 多个 HTTP 方法
    .route("/todos", get(list).post(create))
    // 嵌套路由
    .nest("/api", api_routes())
```

### 提取器 (Extractors)

```rust
use axum::extract::{Path, Query, Json, State};

// 路径参数
async fn get_user(Path(id): Path<u32>) -> String { }

// 查询参数
#[derive(Deserialize)]
struct Params { page: Option<u32> }
async fn list(Query(params): Query<Params>) -> String { }

// JSON 请求
async fn create(Json(data): Json<User>) -> Json<User> { }

// 共享状态
async fn handler(State(state): State<Arc<AppState>>) -> String { }
```

### 响应

```rust
use axum::{http::StatusCode, Json};

// 纯文本
async fn handler() -> &'static str { "Hello" }

// JSON
async fn handler() -> Json<User> { Json(user) }

// 状态码 + JSON
async fn handler() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(user))
}

// 错误响应
async fn handler() -> Result<Json<User>, StatusCode> {
    Ok(Json(user))
}
```

### 状态共享

```rust
use std::sync::{Arc, Mutex};

struct AppState {
    data: Mutex<Vec<Item>>,
}

let state = Arc::new(AppState { 
    data: Mutex::new(Vec::new()) 
});

let app = Router::new()
    .route("/", get(handler))
    .with_state(state);
```

### 中间件

```rust
use axum::middleware::{self, Next};
use axum::http::Request;

async fn my_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    // 前置处理
    let response = next.run(req).await;
    // 后置处理
    response
}

let app = Router::new()
    .route("/", get(handler))
    .layer(middleware::from_fn(my_middleware));
```

### 错误处理

```rust
use axum::response::{IntoResponse, Response};

enum AppError {
    NotFound,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m.as_str()),
        };
        (status, Json(json!({"error": msg}))).into_response()
    }
}
```

---

## WebSocket

### 服务器端

```rust
use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // 接收消息
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    // 处理文本消息
                    socket.send(Message::Text(text)).await.ok();
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    }
}
```

### 广播系统

```rust
use tokio::sync::broadcast;

let (tx, _rx) = broadcast::channel(100);

// 发送消息
tx.send(message)?;

// 接收消息
let mut rx = tx.subscribe();
while let Ok(msg) = rx.recv().await {
    // 处理消息
}
```

---

## 完整示例

### REST API

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        todos: Mutex::new(Vec::new()),
    });
    
    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", 
            get(get_todo)
            .put(update_todo)
            .delete(delete_todo)
        )
        .with_state(state);
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_todos(
    State(state): State<Arc<AppState>>
) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    Json(todos.clone())
}

async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<Todo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = state.todos.lock().unwrap();
    todos.push(todo.clone());
    (StatusCode::CREATED, Json(todo))
}
```

---

## 常用命令

### 测试 API

```bash
# GET 请求
curl http://localhost:3000/api/todos

# POST 请求
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"学习 Rust","description":"深入学习"}'

# PUT 请求
curl -X PUT http://localhost:3000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed":true}'

# DELETE 请求
curl -X DELETE http://localhost:3000/api/todos/1

# 带认证头
curl -H "Authorization: Bearer token123" \
  http://localhost:3000/api/protected
```

---

## 依赖配置

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = { version = "0.7", features = ["ws", "multipart"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower-http = { version = "0.5", features = ["trace", "cors"] }
```

---

## 最佳实践

### 客户端

✅ **DO**:
- 复用 Client 实例
- 设置超时时间
- 使用连接池
- 处理所有错误情况
- 使用 error_for_status()

❌ **DON'T**:
- 为每个请求创建新 Client
- 忽略错误处理
- 硬编码 URL
- 忽略超时设置

### 服务器

✅ **DO**:
- 使用类型安全的提取器
- 实现统一的错误处理
- 使用中间件处理横切关注点
- 验证所有输入
- 使用 Arc 共享状态

❌ **DON'T**:
- 使用全局可变状态
- 在处理器中 panic
- 忽略并发安全
- 返回详细的错误信息给客户端

---

## 调试技巧

```rust
// 打印请求信息
println!("请求 URL: {}", response.url());
println!("状态码: {}", response.status());
println!("响应头: {:?}", response.headers());

// 启用日志
use tower_http::trace::TraceLayer;
let app = Router::new()
    .layer(TraceLayer::new_for_http());

// 错误详情
match result {
    Err(e) if e.is_timeout() => println!("超时"),
    Err(e) if e.is_connect() => println!("连接失败"),
    Err(e) => println!("错误: {}", e),
    _ => {}
}
```
