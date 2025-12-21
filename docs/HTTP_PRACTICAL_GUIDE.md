# Rust HTTP 客户端/服务器 详解与实战

## 目录

1. [HTTP 客户端详解](#http-客户端详解)
2. [HTTP 服务器详解](#http-服务器详解)
3. [实战项目](#实战项目)
4. [最佳实践](#最佳实践)

---

## HTTP 客户端详解

### 1. 基础概念

HTTP 客户端用于发送 HTTP 请求并接收响应。Rust 生态系统中最流行的 HTTP 客户端库是 `reqwest`。

#### 核心特性：
- **异步支持**：基于 tokio 的异步运行时
- **连接池**：自动管理连接，提高性能
- **HTTP/2**：支持 HTTP/2 协议
- **TLS/SSL**：内置 HTTPS 支持
- **中间件**：可扩展的请求/响应处理

### 2. 基本用法

#### GET 请求

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 简单的 GET 请求
    let response = reqwest::get("https://api.github.com/users/rust-lang")
        .await?
        .text()
        .await?;
    
    println!("响应: {}", response);
    Ok(())
}
```

#### POST 请求

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    let response = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?;
    
    println!("状态码: {}", response.status());
    Ok(())
}
```

### 3. 高级功能

#### 请求配置

```rust
use reqwest;
use std::time::Duration;

async fn advanced_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))          // 总超时
        .connect_timeout(Duration::from_secs(10))  // 连接超时
        .pool_max_idle_per_host(25)                // 连接池大小
        .http2_prior_knowledge()                   // 强制 HTTP/2
        .gzip(true)                                // 启用压缩
        .build()?;
    
    let response = client
        .get("https://api.example.com/data")
        .header("User-Agent", "MyApp/1.0")
        .header("Authorization", "Bearer token123")
        .query(&[("page", "1"), ("limit", "10")])
        .send()
        .await?;
    
    Ok(())
}
```

#### 表单提交

```rust
// URL 编码表单
async fn submit_form() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let params = [
        ("username", "alice"),
        ("password", "secret123"),
    ];
    
    let response = client
        .post("https://httpbin.org/post")
        .form(&params)
        .send()
        .await?;
    
    Ok(())
}

// 多部分表单（文件上传）
async fn upload_file() -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::multipart;
    
    let form = multipart::Form::new()
        .text("name", "profile.jpg")
        .text("description", "My profile picture")
        .file("file", "/path/to/image.jpg")
        .await?;
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    Ok(())
}
```

#### Cookie 处理

```rust
async fn cookie_example() -> Result<(), Box<dyn std::error::Error>> {
    // 自动管理 Cookie
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;
    
    // 第一次请求设置 Cookie
    client.get("https://httpbin.org/cookies/set?session=123")
        .send()
        .await?;
    
    // 后续请求自动携带 Cookie
    let response = client.get("https://httpbin.org/cookies")
        .send()
        .await?
        .text()
        .await?;
    
    println!("Cookies: {}", response);
    Ok(())
}
```

#### 错误处理

```rust
use reqwest;

async fn error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://api.example.com/data";
    
    match client.get(url).send().await {
        Ok(response) => {
            // 检查状态码
            if response.status().is_success() {
                let body = response.text().await?;
                println!("成功: {}", body);
            } else if response.status().is_client_error() {
                eprintln!("客户端错误: {}", response.status());
            } else if response.status().is_server_error() {
                eprintln!("服务器错误: {}", response.status());
            }
        }
        Err(e) => {
            if e.is_timeout() {
                eprintln!("请求超时");
            } else if e.is_connect() {
                eprintln!("连接失败");
            } else {
                eprintln!("其他错误: {}", e);
            }
        }
    }
    
    Ok(())
}

// 自动错误处理
async fn auto_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // error_for_status() 会在非 2xx 状态码时返回错误
    let response = client
        .get("https://api.example.com/data")
        .send()
        .await?
        .error_for_status()?;
    
    let data = response.json::<serde_json::Value>().await?;
    println!("数据: {:?}", data);
    
    Ok(())
}
```

#### 并发请求

```rust
use futures::future::join_all;

async fn concurrent_requests() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let urls = vec![
        "https://api.github.com/users/rust-lang",
        "https://api.github.com/users/tokio-rs",
        "https://api.github.com/users/serde-rs",
    ];
    
    // 创建所有请求的 Future
    let futures = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            client.get(url).send().await
        }
    });
    
    // 并发执行所有请求
    let results = join_all(futures).await;
    
    // 处理结果
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(response) => {
                println!("请求 {} 成功: {}", i, response.status());
            }
            Err(e) => {
                eprintln!("请求 {} 失败: {}", i, e);
            }
        }
    }
    
    Ok(())
}
```

#### 流式下载

```rust
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

async fn stream_download() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://example.com/large-file.zip")
        .send()
        .await?;
    
    let total_size = response.content_length().unwrap_or(0);
    println!("文件大小: {} bytes", total_size);
    
    let mut file = tokio::fs::File::create("output.zip").await?;
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        
        // 显示进度
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64) * 100.0;
            print!("\r下载进度: {:.2}%", progress);
        }
    }
    
    println!("\n下载完成！");
    Ok(())
}
```

---

## HTTP 服务器详解

### 1. 基础概念

HTTP 服务器用于接收 HTTP 请求并返回响应。Rust 生态系统中流行的 Web 框架：

- **Axum**：现代、类型安全、基于 tokio
- **Actix-web**：高性能
- **Rocket**：易用性强
- **Warp**：函数式风格

本指南主要使用 **Axum**。

### 2. 基本用法

#### 简单服务器

```rust
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 定义路由
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(hello));
    
    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("服务器运行在 http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello(
    axum::extract::Path(name): axum::extract::Path<String>
) -> String {
    format!("Hello, {}!", name)
}
```

### 3. 路由系统

#### 路径参数

```rust
use axum::{
    extract::Path,
    routing::get,
    Router,
};

async fn get_user(Path(id): Path<u32>) -> String {
    format!("用户 ID: {}", id)
}

async fn get_post(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
    format!("用户 {} 的帖子 {}", user_id, post_id)
}

fn routes() -> Router {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:user_id/posts/:post_id", get(get_post))
}
```

#### 查询参数

```rust
use axum::{
    extract::Query,
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

async fn list_items(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    format!("第 {} 页，每页 {} 条", page, per_page)
}

fn routes() -> Router {
    Router::new()
        .route("/items", get(list_items))
}
```

### 4. JSON 处理

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
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

// 接收 JSON 请求
async fn create_user(
    Json(payload): Json<CreateUser>
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    
    (StatusCode::CREATED, Json(user))
}

// 返回 JSON 响应
async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    })
}

fn routes() -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_user))
}
```

### 5. 状态共享

```rust
use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};

// 应用状态
struct AppState {
    counter: Mutex<u32>,
}

async fn increment_counter(
    State(state): State<Arc<AppState>>
) -> String {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    format!("计数器: {}", counter)
}

#[tokio::main]
async fn main() {
    // 创建共享状态
    let shared_state = Arc::new(AppState {
        counter: Mutex::new(0),
    });
    
    let app = Router::new()
        .route("/counter", get(increment_counter))
        .with_state(shared_state);
    
    // ... 启动服务器
}
```

### 6. 中间件

```rust
use axum::{
    middleware::{self, Next},
    response::Response,
    Router,
    http::Request,
};

// 日志中间件
async fn log_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    println!("收到请求: {} {}", request.method(), request.uri());
    let response = next.run(request).await;
    println!("响应状态: {}", response.status());
    response
}

// 认证中间件
async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    
    match auth_header {
        Some(token) if token.starts_with("Bearer ") => {
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .layer(middleware::from_fn(log_middleware))
        .route("/protected", get(protected))
        .layer(middleware::from_fn(auth_middleware))
}
```

### 7. 错误处理

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// 自定义错误类型
enum AppError {
    NotFound,
    BadRequest(String),
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "资源未找到")
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg.as_str())
            }
            AppError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "内部服务器错误")
            }
        };
        
        let body = Json(json!({
            "error": error_message,
        }));
        
        (status, body).into_response()
    }
}

// 使用自定义错误
async fn get_item(Path(id): Path<u32>) -> Result<Json<Item>, AppError> {
    if id == 0 {
        return Err(AppError::BadRequest("ID 不能为 0".to_string()));
    }
    
    // 查找数据...
    let item = find_item(id)
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(item))
}
```

### 8. 文件操作

#### 文件上传

```rust
use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::post,
    Router,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn upload_file(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        let file_name = field.file_name().unwrap_or("unnamed").to_string();
        let data = field.bytes().await.unwrap();
        
        println!("接收文件: {} (字段: {})", file_name, name);
        
        // 保存文件
        let path = format!("uploads/{}", file_name);
        let mut file = File::create(&path).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        file.write_all(&data).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok("文件上传成功".to_string())
}

fn routes() -> Router {
    Router::new()
        .route("/upload", post(upload_file))
}
```

#### 文件下载

```rust
use axum::{
    body::StreamBody,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

async fn download_file() -> impl IntoResponse {
    let file = match File::open("downloads/example.pdf").await {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    
    let headers = [
        (header::CONTENT_TYPE, "application/pdf"),
        (header::CONTENT_DISPOSITION, "attachment; filename=\"example.pdf\""),
    ];
    
    Ok((headers, body))
}
```

### 9. WebSocket

```rust
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("客户端连接");
    
    // 发送欢迎消息
    if socket.send(Message::Text("欢迎!".to_string())).await.is_err() {
        return;
    }
    
    // 处理消息
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            break;
        };
        
        match msg {
            Message::Text(text) => {
                println!("收到消息: {}", text);
                
                // 回显消息
                let response = format!("你说: {}", text);
                if socket.send(Message::Text(response)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => {
                println!("客户端断开连接");
                break;
            }
            _ => {}
        }
    }
}

fn routes() -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
}
```

---

## 实战项目

### 完整的 REST API 示例

这是一个完整的 Todo API，包含 CRUD 操作、验证和错误处理。

查看 `examples/http_rest_api.rs` 获取完整代码。

### 功能特性

1. **CRUD 操作**
   - 创建 Todo
   - 列出所有 Todos
   - 获取单个 Todo
   - 更新 Todo
   - 删除 Todo

2. **状态管理**
   - 使用 Arc<Mutex<>> 共享状态
   - 线程安全的数据访问

3. **错误处理**
   - 自定义错误类型
   - 统一的错误响应格式

4. **验证**
   - 输入验证
   - 业务逻辑验证

---

## 最佳实践

### 客户端最佳实践

1. **复用客户端实例**
   ```rust
   // ✅ 好的做法
   let client = reqwest::Client::new();
   for url in urls {
       client.get(url).send().await?;
   }
   
   // ❌ 不好的做法
   for url in urls {
       let client = reqwest::Client::new();
       client.get(url).send().await?;
   }
   ```

2. **设置超时**
   ```rust
   let client = reqwest::Client::builder()
       .timeout(Duration::from_secs(30))
       .connect_timeout(Duration::from_secs(10))
       .build()?;
   ```

3. **错误处理**
   ```rust
   match client.get(url).send().await {
       Ok(response) => {
           if response.status().is_success() {
               // 处理成功响应
           } else {
               // 处理错误状态码
           }
       }
       Err(e) => {
           // 处理网络错误
       }
   }
   ```

4. **使用连接池**
   ```rust
   let client = reqwest::Client::builder()
       .pool_max_idle_per_host(25)
       .build()?;
   ```

### 服务器最佳实践

1. **结构化路由**
   ```rust
   fn api_routes() -> Router {
       Router::new()
           .route("/users", get(list_users).post(create_user))
           .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
   }
   
   fn app() -> Router {
       Router::new()
           .nest("/api/v1", api_routes())
   }
   ```

2. **使用中间件**
   ```rust
   let app = Router::new()
       .route("/", get(root))
       .layer(TraceLayer::new_for_http())
       .layer(CorsLayer::permissive())
       .layer(CompressionLayer::new());
   ```

3. **优雅的错误处理**
   ```rust
   // 定义统一的错误类型
   enum AppError {
       NotFound,
       BadRequest(String),
       Internal(anyhow::Error),
   }
   
   // 实现 IntoResponse
   impl IntoResponse for AppError {
       fn into_response(self) -> Response {
           // 返回适当的状态码和错误消息
       }
   }
   ```

4. **输入验证**
   ```rust
   #[derive(Deserialize)]
   struct CreateUser {
       #[serde(deserialize_with = "validate_email")]
       email: String,
       
       #[serde(deserialize_with = "validate_name")]
       name: String,
   }
   ```

5. **安全性**
   - 使用 HTTPS
   - 实现认证和授权
   - 验证所有输入
   - 设置 CORS 策略
   - 实现限流

6. **性能优化**
   - 使用异步处理
   - 实现缓存
   - 启用压缩
   - 使用连接池

---

## 总结

本指南涵盖了 Rust HTTP 客户端和服务器的核心概念和实践。要深入学习，请：

1. 运行示例代码
2. 查看 `examples/` 目录中的完整示例
3. 阅读官方文档：
   - [reqwest](https://docs.rs/reqwest)
   - [axum](https://docs.rs/axum)
   - [tokio](https://docs.rs/tokio)

4. 实践项目：
   - 构建 REST API
   - 创建 API 客户端
   - 实现 WebSocket 应用
   - 开发微服务
