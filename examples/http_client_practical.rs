// HTTP 客户端实战示例
// 运行命令: cargo run --example http_client_practical --features full

use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio;
use futures::future::join_all;

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    username: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Post {
    #[allow(dead_code)]
    id: u32,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    body: String,
    #[allow(dead_code)]
    user_id: u32,
}

#[derive(Debug, Deserialize)]
struct GithubUser {
    login: String,
    id: u32,
    name: Option<String>,
    #[allow(dead_code)]
    public_repos: u32,
    #[allow(dead_code)]
    followers: u32,
}

// ============================================================================
// 示例 1: 基础 GET 请求
// ============================================================================

async fn example_basic_get() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 1: 基础 GET 请求 ===");
    
    // 使用便捷方法发送简单的 GET 请求
    let response = reqwest::get("https://httpbin.org/get")
        .await?
        .text()
        .await?;
    
    println!("响应内容: {}", &response[..200.min(response.len())]);
    println!("...(已截断)\n");
    
    Ok(())
}

// ============================================================================
// 示例 2: 带查询参数的 GET 请求
// ============================================================================

async fn example_get_with_params() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 2: 带查询参数的 GET 请求 ===");
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://httpbin.org/get")
        .query(&[
            ("name", "Alice"),
            ("age", "30"),
            ("city", "Tokyo"),
        ])
        .send()
        .await?;
    
    println!("状态码: {}", response.status());
    println!("URL: {}", response.url());
    
    let json: serde_json::Value = response.json().await?;
    println!("查询参数: {}\n", json["args"]);
    
    Ok(())
}

// ============================================================================
// 示例 3: POST 请求发送 JSON
// ============================================================================

async fn example_post_json() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 3: POST 请求发送 JSON ===");
    
    let client = reqwest::Client::new();
    
    let user = User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        username: Some("alice123".to_string()),
    };
    
    println!("发送数据: {:?}", user);
    
    let response = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?;
    
    println!("状态码: {}", response.status());
    
    let json: serde_json::Value = response.json().await?;
    println!("服务器接收到的 JSON: {}\n", json["json"]);
    
    Ok(())
}

// ============================================================================
// 示例 4: 设置请求头
// ============================================================================

async fn example_custom_headers() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 4: 设置请求头 ===");
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "MyRustApp/1.0")
        .header("X-Custom-Header", "CustomValue")
        .header("Authorization", "Bearer fake-token-123")
        .send()
        .await?;
    
    let json: serde_json::Value = response.json().await?;
    println!("服务器接收到的请求头:");
    println!("{:#}\n", json["headers"]);
    
    Ok(())
}

// ============================================================================
// 示例 5: 表单提交
// ============================================================================

async fn example_form_submission() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 5: 表单提交 (application/x-www-form-urlencoded) ===");
    
    let client = reqwest::Client::new();
    
    let params = [
        ("username", "alice"),
        ("password", "secret123"),
        ("remember", "true"),
    ];
    
    let response = client
        .post("https://httpbin.org/post")
        .form(&params)
        .send()
        .await?;
    
    let json: serde_json::Value = response.json().await?;
    println!("表单数据: {}\n", json["form"]);
    
    Ok(())
}

// ============================================================================
// 示例 6: 超时设置
// ============================================================================

async fn example_timeout() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 6: 超时设置 ===");
    
    // 创建一个设置了超时的客户端
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    
    println!("客户端配置:");
    println!("- 总超时: 10 秒");
    println!("- 连接超时: 5 秒");
    
    // 测试正常请求
    match client.get("https://httpbin.org/delay/2").send().await {
        Ok(response) => {
            println!("✓ 正常请求成功 (延迟 2 秒): {}", response.status());
        }
        Err(e) => {
            println!("✗ 请求失败: {}", e);
        }
    }
    
    // 测试超时请求
    println!("\n尝试请求延迟 15 秒的接口 (会超时)...");
    match client.get("https://httpbin.org/delay/15").send().await {
        Ok(response) => {
            println!("响应: {}", response.status());
        }
        Err(e) => {
            if e.is_timeout() {
                println!("✓ 请求超时 (符合预期)");
            } else {
                println!("✗ 其他错误: {}", e);
            }
        }
    }
    println!();
    
    Ok(())
}

// ============================================================================
// 示例 7: 错误处理
// ============================================================================

async fn example_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 7: 错误处理 ===");
    
    let client = reqwest::Client::new();
    
    // 测试不同的 HTTP 状态码
    let test_cases = vec![
        ("200 OK", "https://httpbin.org/status/200"),
        ("404 Not Found", "https://httpbin.org/status/404"),
        ("500 Internal Server Error", "https://httpbin.org/status/500"),
    ];
    
    for (description, url) in test_cases {
        println!("\n测试: {}", description);
        
        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status();
                println!("状态码: {}", status);
                
                if status.is_success() {
                    println!("✓ 成功 (2xx)");
                } else if status.is_client_error() {
                    println!("✗ 客户端错误 (4xx)");
                } else if status.is_server_error() {
                    println!("✗ 服务器错误 (5xx)");
                }
                
                // 使用 error_for_status 自动处理错误状态码
                if let Err(e) = response.error_for_status_ref() {
                    println!("错误详情: {}", e);
                }
            }
            Err(e) => {
                println!("请求失败: {}", e);
                
                if e.is_timeout() {
                    println!("原因: 超时");
                } else if e.is_connect() {
                    println!("原因: 连接失败");
                } else if e.is_request() {
                    println!("原因: 请求错误");
                }
            }
        }
    }
    println!();
    
    Ok(())
}

// ============================================================================
// 示例 8: Cookie 处理
// ============================================================================

async fn example_cookie_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 8: Cookie 处理 ===");
    
    // 注意: rustls-tls 不支持自动 Cookie 存储
    // 需要手动管理 Cookie 或使用 cookie_store crate
    let client = reqwest::Client::new();
    
    // 第一次请求设置 Cookie
    println!("1. 设置 Cookie...");
    let response: String = client
        .get("https://httpbin.org/cookies/set?session=abc123&user=alice")
        .send()
        .await?
        .text()
        .await?;
    
    println!("响应: {}", &response[..200.min(response.len())]);
    
    // 第二次请求读取 Cookie
    println!("\n2. 读取 Cookie...");
    let response: String = client
        .get("https://httpbin.org/cookies")
        .send()
        .await?
        .text()
        .await?;
    
    println!("Cookie 信息: {}\n", &response[..300.min(response.len())]);
    
    Ok(())
}

// ============================================================================
// 示例 9: 并发请求
// ============================================================================

async fn example_concurrent_requests() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 9: 并发请求 ===");
    
    let client = reqwest::Client::new();
    
    // GitHub 用户列表
    let usernames = vec!["rust-lang", "tokio-rs", "serde-rs"];
    
    println!("并发获取 {} 个 GitHub 用户信息...\n", usernames.len());
    
    // 创建所有请求的 Future
    let futures = usernames.iter().map(|username| {
        let client = client.clone();
        let url = format!("https://api.github.com/users/{}", username);
        async move {
            let result = client
                .get(&url)
                .header("User-Agent", "Rust-Tutorial")
                .send()
                .await;
            (username, result)
        }
    });
    
    // 并发执行所有请求
    let start = std::time::Instant::now();
    let results = join_all(futures).await;
    let duration = start.elapsed();
    
    // 处理结果
    for (username, result) in results {
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(user) = response.json::<GithubUser>().await {
                        println!("✓ {}: {} (ID: {})", 
                            username, 
                            user.name.unwrap_or_else(|| "N/A".to_string()),
                            user.id
                        );
                    }
                } else {
                    println!("✗ {}: HTTP {}", username, response.status());
                }
            }
            Err(e) => {
                println!("✗ {}: {}", username, e);
            }
        }
    }
    
    println!("\n总耗时: {:?}", duration);
    println!("(并发请求比顺序请求快得多)\n");
    
    Ok(())
}

// ============================================================================
// 示例 10: 高级客户端配置
// ============================================================================

async fn example_advanced_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 10: 高级客户端配置 ===");
    
    let client = reqwest::Client::builder()
        // 超时设置
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        // 连接池配置
        .pool_max_idle_per_host(25)
        .pool_idle_timeout(Duration::from_secs(90))
        // 重定向
        .redirect(reqwest::redirect::Policy::limited(10))
        // 构建客户端
        .build()?;
    
    println!("客户端配置:");
    println!("✓ 超时: 30秒");
    println!("✓ 连接超时: 10秒");
    println!("✓ 连接池: 最多 25 个空闲连接/主机");
    println!("✓ 启用 rustls-tls");
    println!("✓ 重定向限制: 最多 10 次");
    
    // 测试请求
    let response: reqwest::Response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;
    
    println!("\n测试请求:");
    println!("状态码: {}", response.status());
    println!("HTTP 版本: {:?}", response.version());
    
    if let Some(content_encoding) = response.headers().get("content-encoding") {
        println!("内容编码: {:?}", content_encoding);
    }
    
    println!();
    
    Ok(())
}

// ============================================================================
// 示例 11: 实战 - API 客户端封装
// ============================================================================

/// 封装的 API 客户端
struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create client");
        
        ApiClient { client, base_url }
    }
    
    /// 获取用户列表
    async fn list_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let url = format!("{}/users", self.base_url);
        let response = self.client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        
        let users = response.json::<Vec<User>>().await?;
        Ok(users)
    }
    
    /// 获取单个用户
    async fn get_user(&self, id: u32) -> Result<User, Box<dyn std::error::Error>> {
        let url = format!("{}/users/{}", self.base_url, id);
        let response = self.client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;
        
        let user = response.json::<User>().await?;
        Ok(user)
    }
    
    /// 创建用户
    async fn create_user(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let url = format!("{}/users", self.base_url);
        let response = self.client
            .post(&url)
            .json(&user)
            .send()
            .await?
            .error_for_status()?;
        
        let created_user = response.json::<User>().await?;
        Ok(created_user)
    }
    
    /// 更新用户
    async fn update_user(&self, id: u32, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let url = format!("{}/users/{}", self.base_url, id);
        let response = self.client
            .put(&url)
            .json(&user)
            .send()
            .await?
            .error_for_status()?;
        
        let updated_user = response.json::<User>().await?;
        Ok(updated_user)
    }
    
    /// 删除用户
    async fn delete_user(&self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/users/{}", self.base_url, id);
        self.client
            .delete(&url)
            .send()
            .await?
            .error_for_status()?;
        
        Ok(())
    }
}

async fn example_api_client_wrapper() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 11: 实战 - API 客户端封装 ===");
    
    let api = ApiClient::new("https://jsonplaceholder.typicode.com".to_string());
    
    // 获取用户列表
    println!("1. 获取用户列表 (前3个)...");
    match api.list_users().await {
        Ok(users) => {
            for user in users.iter().take(3) {
                println!("   - {} ({})", user.name, user.email);
            }
        }
        Err(e) => {
            println!("   错误: {}", e);
        }
    }
    
    // 获取单个用户
    println!("\n2. 获取用户 #1...");
    match api.get_user(1).await {
        Ok(user) => {
            println!("   ID: {:?}", user.id);
            println!("   姓名: {}", user.name);
            println!("   邮箱: {}", user.email);
        }
        Err(e) => {
            println!("   错误: {}", e);
        }
    }
    
    // 创建用户
    println!("\n3. 创建新用户...");
    let new_user = User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        username: Some("alice123".to_string()),
    };
    
    match api.create_user(new_user).await {
        Ok(user) => {
            println!("   ✓ 创建成功");
            println!("   ID: {:?}", user.id);
            println!("   姓名: {}", user.name);
        }
        Err(e) => {
            println!("   错误: {}", e);
        }
    }
    
    println!("\n✓ API 客户端封装演示完成\n");
    
    Ok(())
}

// ============================================================================
// 主函数
// ============================================================================

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║      Rust HTTP 客户端实战示例               ║");
    println!("╚══════════════════════════════════════════════╝");
    
    // 示例 1
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 基础 GET 请求");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_basic_get().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 2
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 带查询参数的 GET");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_get_with_params().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 3
    println!("\n{}", "=".repeat(60));
    println!("运行示例: POST JSON");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_post_json().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 4
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 自定义请求头");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_custom_headers().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 5
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 表单提交");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_form_submission().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 6
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 超时设置");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_timeout().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 7
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 错误处理");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_error_handling().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 8
    println!("\n{}", "=".repeat(60));
    println!("运行示例: Cookie 处理");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_cookie_handling().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 9
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 并发请求");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_concurrent_requests().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 10
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 高级客户端配置");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_advanced_client().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 11
    println!("\n{}", "=".repeat(60));
    println!("运行示例: API 客户端封装");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_api_client_wrapper().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    
    println!("\n{}", "=".repeat(60));
    println!("所有示例运行完成！");
    println!("{}", "=".repeat(60));
    println!();
}
