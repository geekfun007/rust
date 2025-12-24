// Rust HTTP Client 详解 - 使用 reqwest 库
// 
// reqwest 是 Rust 最流行的 HTTP 客户端库，支持异步操作

use reqwest::{Client, header, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ============================================
// 数据模型
// ============================================

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug, Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: serde_json::Value,
}

// ============================================
// 主函数
// ============================================

#[tokio::main]
async fn main() {
    println!("=== Rust HTTP Client 实战（reqwest） ===\n");
    
    // 1. 基础 GET 请求
    println!("--- 1. 基础 GET 请求 ---");
    basic_get_request().await;
    
    // 2. 带参数的 GET 请求
    println!("\n--- 2. 带参数的 GET 请求 ---");
    get_with_params().await;
    
    // 3. POST 请求（JSON）
    println!("\n--- 3. POST 请求（JSON） ---");
    post_json_request().await;
    
    // 4. 自定义请求头
    println!("\n--- 4. 自定义请求头 ---");
    request_with_headers().await;
    
    // 5. 错误处理
    println!("\n--- 5. 错误处理 ---");
    error_handling_example().await;
    
    // 6. 高级配置（超时、重试等）
    println!("\n--- 6. 高级配置 ---");
    advanced_client_config().await;
    
    // 7. 文件下载
    println!("\n--- 7. 文件下载 ---");
    download_file().await;
    
    // 8. 并发请求
    println!("\n--- 8. 并发请求 ---");
    concurrent_requests().await;
    
    // 9. 流式响应
    println!("\n--- 9. 流式响应 ---");
    streaming_response().await;
    
    // 10. 实战案例
    println!("\n--- 10. 实战案例 ---");
    real_world_example().await;
}

// ============================================
// 示例函数
// ============================================

// 1. 基础 GET 请求
async fn basic_get_request() {
    // 最简单的 GET 请求
    match reqwest::get("https://jsonplaceholder.typicode.com/posts/1").await {
        Ok(response) => {
            println!("状态码: {}", response.status());
            
            // 获取响应文本
            match response.text().await {
                Ok(body) => println!("响应体: {}", &body[..100.min(body.len())]),
                Err(e) => println!("读取响应失败: {}", e),
            }
        }
        Err(e) => println!("请求失败: {}", e),
    }
}

// 2. 带查询参数的 GET 请求
async fn get_with_params() {
    let client = Client::new();
    
    // 方式 1: 使用 query() 方法
    let params = [("userId", "1"), ("_limit", "3")];
    
    match client
        .get("https://jsonplaceholder.typicode.com/posts")
        .query(&params)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<Post>>().await {
                    Ok(posts) => {
                        println!("获取到 {} 篇文章:", posts.len());
                        for post in posts.iter().take(2) {
                            println!("  - {}", post.title);
                        }
                    }
                    Err(e) => println!("JSON 解析失败: {}", e),
                }
            }
        }
        Err(e) => println!("请求失败: {}", e),
    }
    
    // 方式 2: 直接在 URL 中包含参数
    let url = "https://jsonplaceholder.typicode.com/posts?userId=1&_limit=2";
    match client.get(url).send().await {
        Ok(response) => {
            println!("直接 URL 参数请求状态: {}", response.status());
        }
        Err(e) => println!("请求失败: {}", e),
    }
}

// 3. POST 请求（发送 JSON）
async fn post_json_request() {
    let client = Client::new();
    
    // 创建要发送的数据
    let new_post = serde_json::json!({
        "title": "Rust HTTP Client 教程",
        "body": "学习使用 reqwest 发送 HTTP 请求",
        "userId": 1
    });
    
    match client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await
    {
        Ok(response) => {
            println!("POST 状态码: {}", response.status());
            
            if let Ok(created) = response.json::<serde_json::Value>().await {
                println!("创建的资源 ID: {}", created["id"]);
            }
        }
        Err(e) => println!("POST 请求失败: {}", e),
    }
    
    // 使用结构体
    let new_user = User {
        id: None,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        age: 25,
    };
    
    match client
        .post("https://jsonplaceholder.typicode.com/users")
        .json(&new_user)
        .send()
        .await
    {
        Ok(response) => {
            println!("创建用户状态: {}", response.status());
        }
        Err(e) => println!("创建用户失败: {}", e),
    }
}

// 4. 自定义请求头
async fn request_with_headers() {
    let client = Client::new();
    
    // 方式 1: 使用 header() 方法
    match client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .header("User-Agent", "Rust-HTTP-Client/1.0")
        .header("Accept", "application/json")
        .header("Authorization", "Bearer fake-token-123")
        .send()
        .await
    {
        Ok(response) => {
            println!("带自定义头部的请求状态: {}", response.status());
        }
        Err(e) => println!("请求失败: {}", e),
    }
    
    // 方式 2: 使用 HeaderMap
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json")
    );
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Rust-Client")
    );
    
    match client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .headers(headers)
        .send()
        .await
    {
        Ok(response) => {
            println!("使用 HeaderMap 的请求状态: {}", response.status());
        }
        Err(e) => println!("请求失败: {}", e),
    }
}

// 5. 错误处理
async fn error_handling_example() {
    let client = Client::new();
    
    // 处理不同类型的错误
    let result = client
        .get("https://jsonplaceholder.typicode.com/posts/99999")
        .send()
        .await;
    
    match result {
        Ok(response) => {
            println!("请求成功，状态码: {}", response.status());
            
            // 检查状态码
            match response.status() {
                StatusCode::OK => println!("✓ 200 OK"),
                StatusCode::NOT_FOUND => println!("✗ 404 未找到"),
                StatusCode::INTERNAL_SERVER_ERROR => println!("✗ 500 服务器错误"),
                status => println!("状态码: {}", status),
            }
            
            // 尝试解析 JSON
            match response.json::<Post>().await {
                Ok(post) => println!("文章标题: {}", post.title),
                Err(e) => println!("JSON 解析错误: {}", e),
            }
        }
        Err(e) => {
            // 详细的错误处理
            if e.is_timeout() {
                println!("✗ 请求超时");
            } else if e.is_connect() {
                println!("✗ 连接失败");
            } else if e.is_request() {
                println!("✗ 请求构建失败");
            } else {
                println!("✗ 其他错误: {}", e);
            }
        }
    }
    
    // 使用 ? 操作符简化错误传播
    async fn fetch_post(id: u32) -> Result<Post, reqwest::Error> {
        let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
        let post = reqwest::get(&url)
            .await?
            .json::<Post>()
            .await?;
        Ok(post)
    }
    
    match fetch_post(1).await {
        Ok(post) => println!("获取文章成功: {}", post.title),
        Err(e) => println!("获取文章失败: {}", e),
    }
}

// 6. 高级客户端配置
async fn advanced_client_config() {
    // 创建带自定义配置的客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(10)) // 超时时间
        .connect_timeout(Duration::from_secs(5)) // 连接超时
        .user_agent("Rust-Advanced-Client/1.0") // User-Agent
        // gzip 默认启用，使用 .no_gzip() 禁用
        .redirect(reqwest::redirect::Policy::limited(5)) // 最多 5 次重定向
        .build();
    
    match client {
        Ok(client) => {
            match client
                .get("https://jsonplaceholder.typicode.com/posts/1")
                .send()
                .await
            {
                Ok(response) => {
                    println!("高级配置客户端请求成功: {}", response.status());
                }
                Err(e) => println!("请求失败: {}", e),
            }
        }
        Err(e) => println!("客户端创建失败: {}", e),
    }
    
    // 使用默认客户端但设置单次请求超时
    match Client::new()
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .timeout(Duration::from_secs(3))
        .send()
        .await
    {
        Ok(response) => println!("单次请求超时设置成功: {}", response.status()),
        Err(e) => println!("请求失败: {}", e),
    }
}

// 7. 文件下载
async fn download_file() {
    let client = Client::new();
    
    // 下载小文件
    match client
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => {
                        println!("下载完成，大小: {} 字节", bytes.len());
                        // 在实际应用中，可以保存到文件
                        // std::fs::write("posts.json", bytes)?;
                    }
                    Err(e) => println!("读取字节失败: {}", e),
                }
            }
        }
        Err(e) => println!("下载失败: {}", e),
    }
}

// 8. 并发请求
async fn concurrent_requests() {
    let client = Client::new();
    
    // 创建多个请求
    let urls = vec![
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];
    
    // 使用 futures 并发执行
    let mut tasks = vec![];
    
    for url in urls {
        let client = client.clone();
        let task = tokio::spawn(async move {
            client.get(url).send().await
        });
        tasks.push(task);
    }
    
    // 等待所有请求完成
    let mut success_count = 0;
    for task in tasks {
        if let Ok(Ok(response)) = task.await {
            if response.status().is_success() {
                success_count += 1;
            }
        }
    }
    
    println!("并发请求完成: {}/3 成功", success_count);
}

// 9. 流式响应
async fn streaming_response() {
    let client = Client::new();
    
    match client
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await
    {
        Ok(response) => {
            println!("开始读取响应...");
            
            // 直接读取字节
            match response.bytes().await {
                Ok(bytes) => {
                    let total_bytes = bytes.len();
                    println!("读取完成，共 {} 字节", total_bytes);
                }
                Err(e) => {
                    println!("读取错误: {}", e);
                }
            }
        }
        Err(e) => println!("请求失败: {}", e),
    }
}

// 10. 实战案例：完整的 API 客户端
async fn real_world_example() {
    println!("实战案例: 构建一个完整的 API 客户端");
    
    // API 客户端结构体
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
        
        async fn get_posts(&self) -> Result<Vec<Post>, reqwest::Error> {
            let url = format!("{}/posts", self.base_url);
            
            self.client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?
                .json()
                .await
        }
        
        async fn get_post(&self, id: u32) -> Result<Post, reqwest::Error> {
            let url = format!("{}/posts/{}", self.base_url, id);
            
            self.client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?
                .json()
                .await
        }
        
        async fn create_post(&self, title: &str, body: &str) -> Result<Post, reqwest::Error> {
            let url = format!("{}/posts", self.base_url);
            
            let payload = serde_json::json!({
                "title": title,
                "body": body,
                "userId": 1
            });
            
            self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&payload)
                .send()
                .await?
                .json()
                .await
        }
    }
    
    // 使用 API 客户端
    let api = ApiClient::new(
        "https://jsonplaceholder.typicode.com".to_string(),
        "fake-api-key-123".to_string(),
    );
    
    // 获取所有文章
    match api.get_posts().await {
        Ok(posts) => println!("  获取到 {} 篇文章", posts.len()),
        Err(e) => println!("  获取文章失败: {}", e),
    }
    
    // 获取单篇文章
    match api.get_post(1).await {
        Ok(post) => println!("  文章标题: {}", post.title),
        Err(e) => println!("  获取文章失败: {}", e),
    }
    
    // 创建文章
    match api.create_post("新文章", "这是内容").await {
        Ok(post) => println!("  创建文章成功，ID: {}", post.id),
        Err(e) => println!("  创建文章失败: {}", e),
    }
}

// ============================================
// 更多实用技巧
// ============================================

/*
reqwest 高级特性：

1. Cookie 管理
   let client = Client::builder()
       .cookie_store(true)
       .build()?;

2. 代理设置
   let proxy = reqwest::Proxy::all("http://proxy.example.com:8080")?;
   let client = Client::builder()
       .proxy(proxy)
       .build()?;

3. 自定义 TLS 配置
   use reqwest::tls;
   let client = Client::builder()
       .danger_accept_invalid_certs(true) // 仅用于测试！
       .build()?;

4. HTTP/2 支持
   reqwest 默认支持 HTTP/2

5. 表单数据
   let params = [("key1", "value1"), ("key2", "value2")];
   client.post(url)
       .form(&params)
       .send()
       .await?;

6. 文件上传
   use reqwest::multipart;
   let form = multipart::Form::new()
       .text("field", "value")
       .file("file", "/path/to/file")?;
   client.post(url)
       .multipart(form)
       .send()
       .await?;

7. Basic Auth
   client.get(url)
       .basic_auth("username", Some("password"))
       .send()
       .await?;

8. 重试机制（需要额外库 reqwest-retry）

运行示例：
  cargo run --bin http_client
*/
