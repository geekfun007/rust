// Reqwest 详解 - HTTP 客户端完全指南
//
// reqwest 是 Rust 中最流行的 HTTP 客户端库
// 本教程涵盖所有核心功能和实战技巧

use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Reqwest HTTP 客户端详解 ===\n");
    
    // 1. Reqwest 基础
    demo_basics().await?;
    
    // 2. GET 请求
    demo_get_requests().await?;
    
    // 3. POST 请求
    demo_post_requests().await?;
    
    // 4. 查询参数
    demo_query_params().await?;
    
    // 5. 请求头
    demo_headers().await?;
    
    // 6. JSON 处理
    demo_json().await?;
    
    // 7. 表单提交
    demo_forms().await?;
    
    // 8. 文件操作
    demo_file_operations().await?;
    
    // 9. 客户端配置
    demo_client_config().await?;
    
    // 10. 错误处理
    demo_error_handling().await?;
    
    // 11. 并发请求
    demo_concurrent_requests().await?;
    
    // 12. 实战案例
    demo_real_world_examples().await?;
    
    println!("\n✅ 所有示例运行成功！");
    Ok(())
}

// ============================================
// 1. Reqwest 基础
// ============================================
async fn demo_basics() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 1. Reqwest 基础 ---\n");
    
    println!("什么是 Reqwest？");
    println!("  - Rust 的现代 HTTP 客户端库");
    println!("  - 基于 hyper 构建");
    println!("  - 支持异步和同步 API");
    println!("  - 自动处理重定向和压缩");
    println!("  - 连接池管理\n");
    
    println!("核心特性:");
    println!("  📌 简洁的 API");
    println!("  📌 自动 JSON 序列化");
    println!("  📌 流式下载/上传");
    println!("  📌 Cookie 支持");
    println!("  📌 代理支持");
    println!("  📌 TLS/SSL 支持\n");
    
    println!("基础用法:");
    println!("  // 简单 GET 请求");
    println!("  let response = reqwest::get(\"https://httpbin.org/get\").await?;");
    println!("  let text = response.text().await?;");
    println!();
    
    Ok(())
}

// ============================================
// 2. GET 请求
// ============================================
async fn demo_get_requests() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 2. GET 请求 ---\n");
    
    // 方式 1: 简单 GET
    println!("方式 1: 简单 GET 请求");
    
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("  状态码: {}", response.status());
    println!("  成功: {}", response.status().is_success());
    println!();
    
    // 方式 2: 使用 Client
    println!("方式 2: 使用 Client (推荐)");
    
    let client = Client::new();
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // 方式 3: 构建器模式
    println!("方式 3: 构建器模式");
    
    let response = client
        .get("https://httpbin.org/get")
        .header("User-Agent", "Rust-Tutorial/1.0")
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // 读取响应体
    println!("读取响应体的方式:");
    
    let response = reqwest::get("https://httpbin.org/get").await?;
    
    // 方式 1: text()
    let text = response.text().await?;
    println!("  text() - 前 100 字符: {}...", &text[..100.min(text.len())]);
    
    // 方式 2: bytes()
    let response = reqwest::get("https://httpbin.org/get").await?;
    let bytes = response.bytes().await?;
    println!("  bytes() - 大小: {} 字节", bytes.len());
    
    // 方式 3: json() - 稍后演示
    println!("  json() - 见 JSON 章节");
    println!();
    
    Ok(())
}

// ============================================
// 3. POST 请求
// ============================================
async fn demo_post_requests() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 3. POST 请求 ---\n");
    
    let client = Client::new();
    
    // POST with body
    println!("POST 纯文本:");
    
    let response = client
        .post("https://httpbin.org/post")
        .body("Hello, World!")
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // POST with JSON
    println!("POST JSON 数据:");
    
    #[derive(Serialize)]
    struct User {
        name: String,
        email: String,
    }
    
    let user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    let response = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // POST with form
    println!("POST 表单数据:");
    
    let mut form = HashMap::new();
    form.insert("username", "alice");
    form.insert("password", "secret123");
    
    let response = client
        .post("https://httpbin.org/post")
        .form(&form)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    Ok(())
}

// ============================================
// 4. 查询参数
// ============================================
async fn demo_query_params() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 4. 查询参数 ---\n");
    
    let client = Client::new();
    
    // 方式 1: 手动构建 URL
    println!("方式 1: 手动构建 URL");
    
    let response = client
        .get("https://httpbin.org/get?key=value&page=1")
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // 方式 2: 使用 query 方法（推荐）
    println!("方式 2: 使用 query 方法");
    
    let params = [("key", "value"), ("page", "1"), ("limit", "10")];
    
    let response = client
        .get("https://httpbin.org/get")
        .query(&params)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // 方式 3: 使用结构体
    println!("方式 3: 使用结构体（类型安全）");
    
    #[derive(Serialize)]
    struct SearchParams {
        q: String,
        page: u32,
        limit: u32,
    }
    
    let params = SearchParams {
        q: "rust".to_string(),
        page: 1,
        limit: 20,
    };
    
    let response = client
        .get("https://httpbin.org/get")
        .query(&params)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    Ok(())
}

// ============================================
// 5. 请求头
// ============================================
async fn demo_headers() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 5. 请求头 ---\n");
    
    let client = Client::new();
    
    // 方式 1: 单个 header
    println!("方式 1: 单个 header");
    
    let response = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "My-App/1.0")
        .header("Authorization", "Bearer token123")
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // 方式 2: 多个 headers
    println!("方式 2: 使用 HeaderMap");
    
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
    
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("My-App/1.0"));
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer token123"));
    headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));
    
    let response = client
        .get("https://httpbin.org/headers")
        .headers(headers)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // 读取响应头
    println!("读取响应头:");
    
    let response = reqwest::get("https://httpbin.org/get").await?;
    
    if let Some(content_type) = response.headers().get("content-type") {
        println!("  Content-Type: {:?}", content_type);
    }
    
    if let Some(server) = response.headers().get("server") {
        println!("  Server: {:?}", server);
    }
    println!();
    
    Ok(())
}

// ============================================
// 6. JSON 处理
// ============================================
async fn demo_json() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 6. JSON 处理 ---\n");
    
    let client = Client::new();
    
    // 发送 JSON
    println!("发送 JSON:");
    
    #[derive(Debug, Serialize, Deserialize)]
    struct Post {
        title: String,
        body: String,
        #[serde(rename = "userId")]
        user_id: u32,
    }
    
    let new_post = Post {
        title: "My Post".to_string(),
        body: "This is the content".to_string(),
        user_id: 1,
    };
    
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    
    // 接收 JSON
    let created_post: Post = response.json().await?;
    println!("  创建的文章: {:?}", created_post);
    println!();
    
    // 获取 JSON 列表
    println!("获取 JSON 列表:");
    
    #[derive(Debug, Deserialize)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }
    
    let response = client
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await?;
    
    let users: Vec<User> = response.json().await?;
    println!("  获取到 {} 个用户", users.len());
    println!("  第一个用户: {:?}", users.first());
    println!();
    
    Ok(())
}

// ============================================
// 7. 表单提交
// ============================================
async fn demo_forms() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 7. 表单提交 ---\n");
    
    let client = Client::new();
    
    // application/x-www-form-urlencoded
    println!("URL 编码表单:");
    
    let mut form = HashMap::new();
    form.insert("username", "alice");
    form.insert("password", "secret123");
    form.insert("remember", "true");
    
    let response = client
        .post("https://httpbin.org/post")
        .form(&form)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // multipart/form-data
    println!("Multipart 表单:");
    
    let form = reqwest::multipart::Form::new()
        .text("username", "alice")
        .text("bio", "Rust developer");
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    Ok(())
}

// ============================================
// 8. 文件操作
// ============================================
async fn demo_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 8. 文件操作 ---\n");
    
    let client = Client::new();
    
    // 下载文件
    println!("下载文件:");
    
    let response = client
        .get("https://httpbin.org/image/png")
        .send()
        .await?;
    
    let bytes = response.bytes().await?;
    println!("  下载了 {} 字节", bytes.len());
    
    // 保存到文件
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create("/tmp/downloaded.png")?;
    file.write_all(&bytes)?;
    println!("  已保存到 /tmp/downloaded.png");
    println!();
    
    // 上传文件
    println!("上传文件:");
    
    // 创建测试文件
    let test_data = b"Hello, this is test file content!";
    std::fs::write("/tmp/test_upload.txt", test_data)?;
    
    // 读取文件内容
    let file_content = std::fs::read("/tmp/test_upload.txt")?;
    
    let part = reqwest::multipart::Part::bytes(file_content)
        .file_name("test_upload.txt")
        .mime_str("text/plain")?;
    
    let form = reqwest::multipart::Form::new()
        .text("description", "Test file upload")
        .part("file", part);
    
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    
    println!("  状态码: {}", response.status());
    println!();
    
    // 流式下载（大文件）
    println!("流式下载（适合大文件）:");
    println!("  使用 response.bytes_stream() 逐块读取");
    println!("  避免一次性加载到内存");
    println!();
    
    Ok(())
}

// ============================================
// 9. 客户端配置
// ============================================
async fn demo_client_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 9. 客户端配置 ---\n");
    
    // 超时设置
    println!("超时设置:");
    
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    
    println!("  总超时: 10 秒");
    println!("  连接超时: 5 秒");
    println!();
    
    // User-Agent
    println!("设置 User-Agent:");
    
    let client = Client::builder()
        .user_agent("MyApp/1.0.0")
        .build()?;
    
    println!("  User-Agent: MyApp/1.0.0");
    println!();
    
    // 默认 Headers
    println!("默认请求头:");
    
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
    
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Bearer default-token"),
    );
    
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    
    println!("  所有请求都会带上 Authorization header");
    println!();
    
    // 重定向
    println!("重定向配置:");
    
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;
    
    println!("  最多跟随 10 次重定向");
    println!();
    
    // 连接池
    println!("连接池配置:");
    
    let client = Client::builder()
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        .build()?;
    
    println!("  每个主机最多 10 个空闲连接");
    println!("  空闲超时: 90 秒");
    println!();
    
    // 完整配置示例
    println!("完整配置示例:");
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .user_agent("MyApp/1.0.0")
        .pool_max_idle_per_host(10)
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()?;
    
    println!("  ✓ 超时: 30s");
    println!("  ✓ 连接超时: 10s");
    println!("  ✓ User-Agent");
    println!("  ✓ 连接池");
    println!("  ✓ 重定向限制");
    println!();
    
    Ok(())
}

// ============================================
// 10. 错误处理
// ============================================
async fn demo_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 10. 错误处理 ---\n");
    
    let client = Client::new();
    
    // 处理 HTTP 错误
    println!("处理 HTTP 状态码:");
    
    let response = client
        .get("https://httpbin.org/status/404")
        .send()
        .await?;
    
    match response.status() {
        StatusCode::OK => println!("  成功"),
        StatusCode::NOT_FOUND => println!("  404 - 未找到资源"),
        StatusCode::INTERNAL_SERVER_ERROR => println!("  500 - 服务器错误"),
        status => println!("  其他状态码: {}", status),
    }
    println!();
    
    // 使用 error_for_status
    println!("使用 error_for_status:");
    
    let result = client
        .get("https://httpbin.org/status/500")
        .send()
        .await?
        .error_for_status();
    
    match result {
        Ok(_) => println!("  成功"),
        Err(e) => println!("  错误: {}", e),
    }
    println!();
    
    // 超时错误
    println!("超时处理:");
    
    let client = Client::builder()
        .timeout(Duration::from_millis(1))
        .build()?;
    
    let result = client
        .get("https://httpbin.org/delay/5")
        .send()
        .await;
    
    match result {
        Ok(_) => println!("  成功"),
        Err(e) => {
            if e.is_timeout() {
                println!("  请求超时");
            } else {
                println!("  其他错误: {}", e);
            }
        }
    }
    println!();
    
    // 错误类型判断
    println!("错误类型判断:");
    println!("  .is_timeout()    - 超时");
    println!("  .is_connect()    - 连接失败");
    println!("  .is_request()    - 请求错误");
    println!("  .is_status()     - HTTP 状态错误");
    println!("  .is_body()       - 响应体错误");
    println!("  .is_decode()     - 解码错误");
    println!();
    
    Ok(())
}

// ============================================
// 11. 并发请求
// ============================================
async fn demo_concurrent_requests() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 11. 并发请求 ---\n");
    
    let client = Client::new();
    
    // 顺序请求
    println!("顺序请求（慢）:");
    
    let start = std::time::Instant::now();
    
    for i in 1..=3 {
        let _response = client
            .get(format!("https://jsonplaceholder.typicode.com/posts/{}", i))
            .send()
            .await?;
    }
    
    println!("  耗时: {:?}", start.elapsed());
    println!();
    
    // 并发请求
    println!("并发请求（快）:");
    
    let start = std::time::Instant::now();
    
    let tasks: Vec<_> = (1..=3)
        .map(|i| {
            let client = client.clone();
            tokio::spawn(async move {
                client
                    .get(format!("https://jsonplaceholder.typicode.com/posts/{}", i))
                    .send()
                    .await
            })
        })
        .collect();
    
    for task in tasks {
        let _response = task.await??;
    }
    
    println!("  耗时: {:?}", start.elapsed());
    println!();
    
    // 使用 join_all
    println!("使用 futures::join_all:");
    
    use futures::future::join_all;
    
    let futures: Vec<_> = (1..=3)
        .map(|i| {
            client.get(format!("https://jsonplaceholder.typicode.com/posts/{}", i))
                .send()
        })
        .collect();
    
    let results = join_all(futures).await;
    
    println!("  成功请求数: {}", results.iter().filter(|r| r.is_ok()).count());
    println!();
    
    Ok(())
}

// ============================================
// 12. 实战案例
// ============================================
async fn demo_real_world_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 12. 实战案例 ---\n");
    
    // 案例 1: REST API 客户端
    println!("案例 1: REST API 客户端\n");
    rest_api_client_example().await?;
    
    // 案例 2: 网页爬虫
    println!("\n案例 2: 简单网页爬虫\n");
    web_scraper_example().await?;
    
    // 案例 3: 文件下载器
    println!("\n案例 3: 批量文件下载\n");
    file_downloader_example().await?;
    
    Ok(())
}

// 案例 1: REST API 客户端
async fn rest_api_client_example() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, Deserialize)]
    struct ApiUser {
        id: u32,
        name: String,
        email: String,
    }
    
    struct ApiClient {
        client: Client,
        base_url: String,
        api_key: String,
    }
    
    impl ApiClient {
        fn new(base_url: String, api_key: String) -> Self {
            let client = Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap();
            
            Self {
                client,
                base_url,
                api_key,
            }
        }
        
        async fn get_user(&self, id: u32) -> Result<ApiUser, Box<dyn std::error::Error>> {
            let url = format!("{}/users/{}", self.base_url, id);
            
            let user = self.client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?
                .error_for_status()?
                .json::<ApiUser>()
                .await?;
            
            Ok(user)
        }
        
        async fn create_post(&self, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
            #[derive(Serialize)]
            struct NewPost<'a> {
                title: &'a str,
                body: &'a str,
                #[serde(rename = "userId")]
                user_id: u32,
            }
            
            let url = format!("{}/posts", self.base_url);
            
            let post = NewPost {
                title,
                body,
                user_id: 1,
            };
            
            self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&post)
                .send()
                .await?
                .error_for_status()?;
            
            Ok(())
        }
    }
    
    // 使用 API 客户端
    let api = ApiClient::new(
        "https://jsonplaceholder.typicode.com".to_string(),
        "fake-api-key".to_string(),
    );
    
    // 获取用户
    match api.get_user(1).await {
        Ok(user) => {
            println!("  用户信息:");
            println!("    ID: {}", user.id);
            println!("    姓名: {}", user.name);
            println!("    邮箱: {}", user.email);
        }
        Err(e) => println!("  错误: {}", e),
    }
    
    // 创建文章
    match api.create_post("Test Title", "Test body content").await {
        Ok(_) => println!("  ✓ 文章创建成功"),
        Err(e) => println!("  ✗ 创建失败: {}", e),
    }
    
    Ok(())
}

// 案例 2: 网页爬虫
async fn web_scraper_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Bot/1.0)")
        .timeout(Duration::from_secs(10))
        .build()?;
    
    // 抓取网页
    let response = client
        .get("https://httpbin.org/html")
        .send()
        .await?;
    
    let html = response.text().await?;
    
    println!("  抓取的 HTML 长度: {} 字节", html.len());
    
    // 简单解析（实际项目中使用 scraper 或 select.rs）
    let title_count = html.matches("<title>").count();
    println!("  找到 {} 个 <title> 标签", title_count);
    
    Ok(())
}

// 案例 3: 文件下载器
async fn file_downloader_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 模拟批量下载
    let urls = vec![
        "https://httpbin.org/image/png",
        "https://httpbin.org/image/jpeg",
    ];
    
    println!("  开始下载 {} 个文件...", urls.len());
    
    for (i, url) in urls.iter().enumerate() {
        match client.get(*url).send().await {
            Ok(response) => {
                if let Ok(bytes) = response.bytes().await {
                    println!("    ✓ 文件 {}: {} 字节", i + 1, bytes.len());
                }
            }
            Err(e) => {
                println!("    ✗ 文件 {} 下载失败: {}", i + 1, e);
            }
        }
    }
    
    println!("  下载完成");
    
    Ok(())
}

/*
=== 总结 ===

1. Reqwest 核心概念:

   基础请求:
   - GET, POST, PUT, DELETE
   - 请求构建器模式
   - 查询参数
   
   数据格式:
   - JSON 序列化/反序列化
   - 表单提交
   - Multipart 上传
   
   高级功能:
   - 自定义 Headers
   - Cookie 处理
   - 超时和重试
   - 连接池

2. 客户端配置:

   基础:
   - timeout() - 总超时
   - connect_timeout() - 连接超时
   - user_agent() - User-Agent
   
   高级:
   - default_headers() - 默认请求头
   - pool_max_idle_per_host() - 连接池
   - redirect() - 重定向策略

3. 最佳实践:

   DO:
   ✓ 复用 Client 实例
   ✓ 设置合理的超时
   ✓ 使用 error_for_status()
   ✓ 并发请求时使用 tokio::spawn
   
   DON'T:
   ✗ 为每个请求创建新 Client
   ✗ 忽略错误处理
   ✗ 阻塞异步任务

4. 常用模式:

   API 客户端:
   - 封装 Client
   - 统一错误处理
   - 重试逻辑
   
   并发请求:
   - tokio::spawn
   - futures::join_all
   - 控制并发数

运行示例:
  cargo run --bin reqwest_detailed
*/
