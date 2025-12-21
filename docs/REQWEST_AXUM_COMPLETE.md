# Rust HTTP 完全指南: reqwest + axum 详解与实战

## 目录

### Part 1: reqwest 详解
1. [reqwest 基础](#reqwest-基础)
2. [客户端配置](#客户端配置)
3. [请求方法](#请求方法)
4. [响应处理](#响应处理)
5. [高级功能](#reqwest-高级功能)

### Part 2: axum 详解
6. [axum 基础](#axum-基础)
7. [路由系统](#路由系统)
8. [提取器](#提取器)
9. [响应类型](#响应类型)
10. [中间件](#中间件)
11. [状态管理](#状态管理)

---

# Part 1: reqwest 详解

## reqwest 基础

### 什么是 reqwest?

reqwest 是 Rust 生态系统中最流行的 HTTP 客户端库，提供：
- 🚀 **异步支持**: 基于 tokio
- 🔒 **HTTPS**: 内置 TLS 支持
- 📦 **JSON**: 自动序列化/反序列化
- 🔄 **连接池**: 自动管理连接
- 🎯 **类型安全**: 利用 Rust 类型系统

### 快速开始

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 简单的 GET 请求
    let response = reqwest::get("https://httpbin.org/get")
        .await?
        .text()
        .await?;
    
    println!("Response: {}", response);
    Ok(())
}
```

---

## 客户端配置

### Client vs 便捷函数

```rust
// 便捷函数 - 每次创建新客户端
let response = reqwest::get("https://api.example.com").await?;

// Client - 复用连接池（推荐）
let client = reqwest::Client::new();
let response = client.get("https://api.example.com").send().await?;
```

### ClientBuilder 配置

```rust
use reqwest::Client;
use std::time::Duration;

let client = Client::builder()
    // 超时设置
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(10))
    
    // 连接池
    .pool_max_idle_per_host(25)
    .pool_idle_timeout(Duration::from_secs(90))
    
    // TLS 配置
    .use_rustls_tls()  // 或 use_native_tls()
    .danger_accept_invalid_certs(false)
    
    // HTTP 版本
    .http1_only()  // 或 http2_prior_knowledge()
    
    // 重定向
    .redirect(reqwest::redirect::Policy::limited(10))
    
    // User-Agent
    .user_agent("MyApp/1.0")
    
    // 代理
    .proxy(reqwest::Proxy::all("http://proxy:8080")?)
    
    // 构建
    .build()?;
```

### 常用配置模式

```rust
// 生产环境推荐配置
let client = Client::builder()
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(10))
    .pool_max_idle_per_host(10)
    .use_rustls_tls()
    .build()?;

// 开发环境配置（允许自签名证书）
let client = Client::builder()
    .timeout(Duration::from_secs(60))
    .danger_accept_invalid_certs(true)
    .build()?;
```

---

## 请求方法

### HTTP 方法

```rust
let client = Client::new();

// GET
let response = client.get("https://api.example.com/users").send().await?;

// POST
let response = client.post("https://api.example.com/users")
    .json(&user_data)
    .send()
    .await?;

// PUT
let response = client.put("https://api.example.com/users/1")
    .json(&user_data)
    .send()
    .await?;

// DELETE
let response = client.delete("https://api.example.com/users/1").send().await?;

// PATCH
let response = client.patch("https://api.example.com/users/1")
    .json(&patch_data)
    .send()
    .await?;

// HEAD
let response = client.head("https://api.example.com/users").send().await?;
```

### 请求构建

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
}

// 查询参数
let response = client
    .get("https://api.example.com/search")
    .query(&[("q", "rust"), ("page", "1")])
    .send()
    .await?;

// 请求头
let response = client
    .get("https://api.example.com/data")
    .header("Authorization", "Bearer token123")
    .header("Accept", "application/json")
    .send()
    .await?;

// JSON 请求体
let user = User {
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

let response = client
    .post("https://api.example.com/users")
    .json(&user)
    .send()
    .await?;

// 表单数据
let params = [("username", "alice"), ("password", "secret")];
let response = client
    .post("https://api.example.com/login")
    .form(&params)
    .send()
    .await?;

// 原始请求体
let response = client
    .post("https://api.example.com/data")
    .body("raw data")
    .send()
    .await?;
```

### 文件上传

```rust
use reqwest::multipart;

// 多部分表单
let form = multipart::Form::new()
    .text("name", "profile.jpg")
    .text("description", "My profile picture")
    .file("file", "/path/to/image.jpg")
    .await?;

let response = client
    .post("https://api.example.com/upload")
    .multipart(form)
    .send()
    .await?;

// 从内存上传
let file_data = vec![0u8; 1024]; // 文件数据
let part = multipart::Part::bytes(file_data)
    .file_name("data.bin")
    .mime_str("application/octet-stream")?;

let form = multipart::Form::new()
    .part("file", part);

let response = client
    .post("https://api.example.com/upload")
    .multipart(form)
    .send()
    .await?;
```

---

## 响应处理

### 基本响应方法

```rust
let response = client.get("https://api.example.com/data").send().await?;

// 状态码
let status = response.status();
println!("Status: {}", status);
println!("Is success: {}", status.is_success());
println!("Is client error: {}", status.is_client_error());
println!("Is server error: {}", status.is_server_error());

// 响应头
let headers = response.headers();
if let Some(content_type) = headers.get("content-type") {
    println!("Content-Type: {:?}", content_type);
}

// URL（可能重定向后的）
let url = response.url();
println!("Final URL: {}", url);

// HTTP 版本
let version = response.version();
println!("HTTP version: {:?}", version);

// 内容长度
if let Some(len) = response.content_length() {
    println!("Content-Length: {}", len);
}
```

### 读取响应体

```rust
// 文本
let text = response.text().await?;

// 字节
let bytes = response.bytes().await?;

// JSON
#[derive(Deserialize)]
struct ApiResponse {
    id: u32,
    name: String,
}

let data: ApiResponse = response.json().await?;

// 流式读取
use futures_util::StreamExt;

let mut stream = response.bytes_stream();
while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    // 处理数据块
    println!("Received {} bytes", chunk.len());
}
```

### 错误处理

```rust
// 检查状态码
let response = client.get(url).send().await?;

if response.status().is_success() {
    let data = response.text().await?;
    println!("Success: {}", data);
} else {
    eprintln!("Error: {}", response.status());
}

// 自动错误处理
let response = client
    .get(url)
    .send()
    .await?
    .error_for_status()?;  // 非 2xx 返回错误

// 详细错误处理
match client.get(url).send().await {
    Ok(response) => {
        match response.error_for_status() {
            Ok(res) => {
                let data = res.text().await?;
                println!("Data: {}", data);
            }
            Err(e) => {
                eprintln!("HTTP error: {}", e);
            }
        }
    }
    Err(e) => {
        if e.is_timeout() {
            eprintln!("Request timed out");
        } else if e.is_connect() {
            eprintln!("Connection failed");
        } else if e.is_request() {
            eprintln!("Request error");
        } else {
            eprintln!("Other error: {}", e);
        }
    }
}
```

---

## reqwest 高级功能

### 并发请求

```rust
use futures::future::join_all;

let urls = vec![
    "https://api.example.com/data/1",
    "https://api.example.com/data/2",
    "https://api.example.com/data/3",
];

let client = Client::new();

// 创建所有请求
let futures = urls.into_iter().map(|url| {
    let client = client.clone();
    async move {
        client.get(url).send().await
    }
});

// 并发执行
let results = join_all(futures).await;

// 处理结果
for (i, result) in results.into_iter().enumerate() {
    match result {
        Ok(response) => println!("Request {}: {}", i, response.status()),
        Err(e) => eprintln!("Request {} failed: {}", i, e),
    }
}
```

### 重试机制

```rust
use tokio::time::{sleep, Duration};

async fn request_with_retry(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<reqwest::Response, reqwest::Error> {
    let mut attempts = 0;
    
    loop {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response);
                }
                
                if attempts >= max_retries {
                    return Ok(response);
                }
            }
            Err(e) => {
                if attempts >= max_retries {
                    return Err(e);
                }
            }
        }
        
        attempts += 1;
        let delay = Duration::from_secs(2u64.pow(attempts));
        println!("Retry {} after {:?}", attempts, delay);
        sleep(delay).await;
    }
}
```

### 认证

```rust
// Bearer Token
let response = client
    .get("https://api.example.com/protected")
    .bearer_auth("your_token_here")
    .send()
    .await?;

// Basic Auth
let response = client
    .get("https://api.example.com/protected")
    .basic_auth("username", Some("password"))
    .send()
    .await?;

// 自定义认证头
let response = client
    .get("https://api.example.com/protected")
    .header("X-API-Key", "your_api_key")
    .send()
    .await?;
```

---

# Part 2: axum 详解

## axum 基础

### 什么是 axum?

axum 是基于 tokio 和 hyper 的现代 Web 框架，特点：
- 🎯 **类型安全**: 利用 Rust 类型系统
- ⚡ **高性能**: 基于 hyper
- 🔧 **可组合**: 模块化设计
- 🛡️ **安全**: 编译时保证

### 最简单的服务器

```rust
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
```

---

## 路由系统

### 基础路由

```rust
use axum::{routing::{get, post, put, delete}, Router};

let app = Router::new()
    // 单个方法
    .route("/", get(index))
    .route("/about", get(about))
    
    // 多个方法
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", 
        get(get_user)
        .put(update_user)
        .delete(delete_user)
    )
    
    // 任意方法
    .route("/any", any(handle_any));

async fn index() -> &'static str {
    "Home"
}
```

### 路径参数

```rust
use axum::extract::Path;

// 单个参数
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

// 多个参数
async fn get_post(
    Path((user_id, post_id)): Path<(u32, u32)>
) -> String {
    format!("User {} Post {}", user_id, post_id)
}

// 使用结构体
#[derive(Deserialize)]
struct Params {
    user_id: u32,
    post_id: u32,
}

async fn get_post_struct(Path(params): Path<Params>) -> String {
    format!("User {} Post {}", params.user_id, params.post_id)
}

// 路由定义
let app = Router::new()
    .route("/users/:id", get(get_user))
    .route("/users/:user_id/posts/:post_id", get(get_post));
```

### 查询参数

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

async fn list_items(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    format!("Page {} with {} items per page", page, per_page)
}

// 路由: /items?page=2&per_page=20
```

### 嵌套路由

```rust
// API 路由
fn api_routes() -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user))
        .route("/posts", get(list_posts))
}

// 主应用
let app = Router::new()
    .route("/", get(index))
    .nest("/api/v1", api_routes())
    .nest("/api/v2", api_v2_routes());
```

---

## 提取器

### 常用提取器

```rust
use axum::{
    extract::{Path, Query, Json, State, Form},
    http::{HeaderMap, Method, Uri},
};

// Path - 路径参数
async fn handler1(Path(id): Path<u32>) {}

// Query - 查询参数
async fn handler2(Query(params): Query<HashMap<String, String>>) {}

// Json - JSON 请求体
async fn handler3(Json(data): Json<User>) {}

// Form - 表单数据
async fn handler4(Form(data): Form<LoginForm>) {}

// State - 应用状态
async fn handler5(State(state): State<Arc<AppState>>) {}

// HeaderMap - 请求头
async fn handler6(headers: HeaderMap) {}

// Method - HTTP 方法
async fn handler7(method: Method) {}

// Uri - 请求 URI
async fn handler8(uri: Uri) {}
```

### 组合提取器

```rust
use axum::extract::{Path, Query, Json, State};

#[derive(Deserialize)]
struct CreatePost {
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct PostQuery {
    draft: Option<bool>,
}

async fn create_post(
    Path(user_id): Path<u32>,
    Query(query): Query<PostQuery>,
    State(state): State<Arc<AppState>>,
    Json(post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    // user_id 来自路径
    // query 来自查询参数
    // state 是应用状态
    // post 是 JSON 请求体
    
    // 处理逻辑...
    Ok(Json(created_post))
}
```

### 自定义提取器

```rust
use axum::{
    async_trait,
    extract::{FromRequestParts, FromRequest},
    http::{request::Parts, Request},
};

// 从请求头提取用户
struct AuthUser {
    user_id: u32,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);
    
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 从请求头获取 token
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing token"))?;
        
        // 验证 token 并提取用户 ID
        let user_id = validate_token(token)
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid token"))?;
        
        Ok(AuthUser { user_id })
    }
}

// 使用自定义提取器
async fn protected_route(user: AuthUser) -> String {
    format!("Hello, user {}!", user.user_id)
}
```

---

## 响应类型

### 基本响应

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// 纯文本
async fn text_response() -> &'static str {
    "Hello, World!"
}

// String
async fn string_response() -> String {
    "Dynamic response".to_string()
}

// JSON
async fn json_response() -> Json<User> {
    Json(User {
        id: 1,
        name: "Alice".to_string(),
    })
}

// 状态码
async fn status_response() -> StatusCode {
    StatusCode::NO_CONTENT
}

// 状态码 + JSON
async fn status_json() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(user))
}

// 自定义头
async fn with_headers() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", "value".parse().unwrap());
    (headers, "Response with headers".to_string())
}
```

### Result 响应

```rust
// Result<T, E> 其中 E 实现 IntoResponse
async fn fallible_handler() -> Result<Json<User>, StatusCode> {
    let user = find_user(1)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(user))
}

// 自定义错误类型
enum ApiError {
    NotFound,
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
        };
        
        (status, Json(json!({ "error": message }))).into_response()
    }
}

async fn handler() -> Result<Json<User>, ApiError> {
    let user = find_user(1)
        .ok_or(ApiError::NotFound)?;
    
    Ok(Json(user))
}
```

### 流式响应

```rust
use axum::{
    body::StreamBody,
    response::IntoResponse,
};
use futures::stream::{self, Stream};

async fn stream_response() -> impl IntoResponse {
    let stream = stream::iter(vec![
        Ok::<_, std::io::Error>("Hello, "),
        Ok("streaming "),
        Ok("world!"),
    ]);
    
    StreamBody::new(stream)
}
```

---

## 中间件

### Tower 中间件

```rust
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
};

let app = Router::new()
    .route("/", get(handler))
    // 日志
    .layer(TraceLayer::new_for_http())
    // CORS
    .layer(CorsLayer::permissive())
    // 压缩
    .layer(CompressionLayer::new());
```

### 自定义中间件

```rust
use axum::{
    middleware::{self, Next},
    http::Request,
    response::Response,
};

// 中间件函数
async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // 检查认证
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    
    if token.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // 继续处理请求
    Ok(next.run(request).await)
}

// 应用中间件
let app = Router::new()
    .route("/protected", get(handler))
    .layer(middleware::from_fn(auth_middleware));
```

---

## 状态管理

### 共享状态

```rust
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
```

### 使用 RwLock

```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Clone)]
struct AppState {
    cache: Arc<RwLock<HashMap<String, String>>>,
}

async fn get_cache(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<String, StatusCode> {
    let cache = state.cache.read().unwrap();
    cache.get(&key)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)
}

async fn set_cache(
    State(state): State<AppState>,
    Path(key): Path<String>,
    body: String,
) -> StatusCode {
    let mut cache = state.cache.write().unwrap();
    cache.insert(key, body);
    StatusCode::OK
}
```

---

## 完整示例

查看以下实战示例获取完整代码：
- `examples/reqwest_advanced.rs` - reqwest 高级用法
- `examples/axum_complete.rs` - axum 完整应用
- `examples/http_client_practical.rs` - HTTP 客户端实战
- `examples/http_server_practical.rs` - HTTP 服务器实战

---

## 最佳实践

### reqwest 最佳实践

1. **复用 Client**
   ```rust
   // ✅ 好
   let client = Client::new();
   for url in urls {
       client.get(url).send().await?;
   }
   
   // ❌ 差
   for url in urls {
       reqwest::get(url).await?;
   }
   ```

2. **设置超时**
   ```rust
   let client = Client::builder()
       .timeout(Duration::from_secs(30))
       .build()?;
   ```

3. **错误处理**
   ```rust
   let response = client.get(url)
       .send()
        .await?
       .error_for_status()?;
   ```

### axum 最佳实践

1. **使用类型安全的提取器**
   ```rust
   // ✅ 好
   async fn handler(Json(data): Json<User>) -> Result<Json<User>, ApiError>
   
   // ❌ 差
   async fn handler(body: String) -> String
   ```

2. **统一错误处理**
   ```rust
   enum ApiError {
       NotFound,
       BadRequest(String),
   }
   
   impl IntoResponse for ApiError { ... }
   ```

3. **使用状态共享**
   ```rust
   #[derive(Clone)]
   struct AppState {
       db: Arc<Database>,
   }
   
   let app = Router::new()
       .route("/", get(handler))
       .with_state(state);
   ```

---

## 总结

### reqwest 核心要点
- 使用 `Client` 复用连接
- 配置超时和重试
- 处理所有错误情况
- 使用类型安全的 JSON 序列化

### axum 核心要点
- 利用类型系统保证安全
- 使用提取器简化代码
- 实现 `IntoResponse` 统一响应
- 使用中间件处理横切关注点

运行示例：
```bash
cargo run --example reqwest_advanced --features full
cargo run --example axum_complete --features full
```
