// reqwest 高级用法实战示例
// 运行命令: cargo run --example reqwest_advanced --features full

use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio;

// ============================================================================
// 数据结构
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

// ============================================================================
// 示例 1: 高级客户端配置
// ============================================================================

fn create_advanced_client() -> Result<Client, Box<dyn std::error::Error>> {
    println!("\n=== 示例 1: 高级客户端配置 ===\n");
    
    let client = Client::builder()
        // 超时配置
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        
        // 连接池配置
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        
        // TLS 配置
        .use_rustls_tls()
        
        // 重定向策略
        .redirect(reqwest::redirect::Policy::limited(10))
        
        // User-Agent
        .user_agent("RustAdvancedClient/1.0")
        
        // 默认请求头
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("application/json")
            );
            headers
        })
        
        .build()?;
    
    println!("✓ 高级客户端创建成功");
    println!("  - 超时: 30秒");
    println!("  - 连接超时: 10秒");
    println!("  - 连接池: 10个空闲连接/主机");
    println!("  - TLS: rustls");
    println!("  - 重定向: 最多10次");
    
    Ok(client)
}

// ============================================================================
// 示例 2: 请求重试机制
// ============================================================================

async fn request_with_retry(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let mut attempts = 0;
    
    loop {
        println!("  尝试 {} ...", attempts + 1);
        
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("  ✓ 请求成功");
                    return Ok(response);
                }
                
                if attempts >= max_retries {
                    println!("  ✗ 达到最大重试次数");
                    return Ok(response);
                }
                
                println!("  状态码 {}, 重试中...", response.status());
            }
            Err(e) => {
                if attempts >= max_retries {
                    return Err(Box::new(e));
                }
                
                println!("  错误: {}, 重试中...", e);
            }
        }
        
        attempts += 1;
        let delay = Duration::from_secs(2u64.pow(attempts.min(5)));
        println!("  等待 {:?} 后重试", delay);
        tokio::time::sleep(delay).await;
    }
}

async fn example_retry() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 2: 请求重试机制 ===\n");
    
    let client = Client::new();
    
    // 测试正常请求
    println!("测试正常请求:");
    let response = request_with_retry(
        &client,
        "https://httpbin.org/get",
        3
    ).await?;
    println!("  最终状态: {}\n", response.status());
    
    // 测试失败请求（会重试）
    println!("测试失败请求（404）:");
    let response = request_with_retry(
        &client,
        "https://httpbin.org/status/404",
        2
    ).await?;
    println!("  最终状态: {}", response.status());
    
    Ok(())
}

// ============================================================================
// 示例 3: 并发请求与超时控制
// ============================================================================

async fn example_concurrent_with_timeout() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 3: 并发请求与超时控制 ===\n");
    
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    
    let urls = vec![
        ("快速", "https://httpbin.org/delay/1"),
        ("中速", "https://httpbin.org/delay/2"),
        ("慢速", "https://httpbin.org/delay/3"),
    ];
    
    println!("发送 {} 个并发请求（超时5秒）...\n", urls.len());
    
    let futures = urls.into_iter().map(|(name, url)| {
        let client = client.clone();
        async move {
            let start = std::time::Instant::now();
            let result = client.get(url).send().await;
            let duration = start.elapsed();
            (name, result, duration)
        }
    });
    
    let results = futures::future::join_all(futures).await;
    
    for (name, result, duration) in results {
        match result {
            Ok(response) => {
                println!("✓ {} 请求: {} (耗时: {:?})", 
                    name, response.status(), duration);
            }
            Err(e) => {
                if e.is_timeout() {
                    println!("✗ {} 请求: 超时 (耗时: {:?})", name, duration);
                } else {
                    println!("✗ {} 请求: {} (耗时: {:?})", name, e, duration);
                }
            }
        }
    }
    
    Ok(())
}

// ============================================================================
// 示例 4: 流式下载
// ============================================================================

async fn example_streaming_download() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 4: 流式下载 ===\n");
    
    use futures_util::StreamExt;
    
    let client = Client::new();
    let url = "https://httpbin.org/bytes/10240"; // 下载 10KB
    
    println!("开始流式下载...");
    let response = client.get(url).send().await?;
    
    let total_size = response.content_length().unwrap_or(0);
    println!("文件大小: {} 字节\n", total_size);
    
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        downloaded += chunk.len() as u64;
        
        let progress = if total_size > 0 {
            (downloaded as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };
        
        print!("\r下载进度: {:.1}% ({}/{} 字节)", 
            progress, downloaded, total_size);
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    
    println!("\n✓ 下载完成！");
    
    Ok(())
}

// ============================================================================
// 示例 5: API 客户端封装
// ============================================================================

struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    fn new(base_url: String, api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        Ok(ApiClient {
            client,
            base_url,
            api_key,
        })
    }
    
    async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        
        let data = response.json::<T>().await?;
        Ok(data)
    }
    
    async fn post<T, R>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R, Box<dyn std::error::Error>>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        
        let data = response.json::<R>().await?;
        Ok(data)
    }
}

async fn example_api_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 5: API 客户端封装 ===\n");
    
    let api = ApiClient::new(
        "https://jsonplaceholder.typicode.com".to_string(),
        "demo-key-123".to_string(),
    )?;
    
    println!("1. GET 请求获取用户列表");
    let users: Vec<User> = api.get("/users").await?;
    println!("   获取到 {} 个用户", users.len());
    for user in users.iter().take(3) {
        println!("   - {} ({})", user.name, user.email);
    }
    
    println!("\n2. POST 请求创建用户");
    let new_user = User {
        id: None,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let created: User = api.post("/users", &new_user).await?;
    println!("   ✓ 用户创建成功");
    println!("   ID: {:?}", created.id);
    println!("   姓名: {}", created.name);
    
    Ok(())
}

// ============================================================================
// 示例 6: 认证方式
// ============================================================================

async fn example_authentication() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 6: 认证方式 ===\n");
    
    let client = Client::new();
    
    // 1. Bearer Token
    println!("1. Bearer Token 认证");
    let response = client
        .get("https://httpbin.org/bearer")
        .bearer_auth("test-token-123")
        .send()
        .await?;
    println!("   状态: {}", response.status());
    
    // 2. Basic Auth
    println!("\n2. Basic Auth 认证");
    let response = client
        .get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", Some("pass"))
        .send()
        .await?;
    println!("   状态: {}", response.status());
    
    // 3. API Key
    println!("\n3. API Key 认证");
    let response = client
        .get("https://httpbin.org/get")
        .header("X-API-Key", "my-api-key-123")
        .send()
        .await?;
    println!("   状态: {}", response.status());
    
    // 4. 自定义认证头
    println!("\n4. 自定义认证头");
    let response = client
        .get("https://httpbin.org/get")
        .header("Authorization", "Custom token123")
        .send()
        .await?;
    println!("   状态: {}", response.status());
    
    Ok(())
}

// ============================================================================
// 示例 7: 错误处理最佳实践
// ============================================================================

#[derive(Debug)]
enum ApiError {
    Network(reqwest::Error),
    Http(reqwest::StatusCode),
    Parse(serde_json::Error),
    Timeout,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApiError::Network(e) => write!(f, "网络错误: {}", e),
            ApiError::Http(status) => write!(f, "HTTP 错误: {}", status),
            ApiError::Parse(e) => write!(f, "解析错误: {}", e),
            ApiError::Timeout => write!(f, "请求超时"),
        }
    }
}

impl std::error::Error for ApiError {}

async fn make_request(url: &str) -> Result<serde_json::Value, ApiError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(ApiError::Network)?;
    
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                ApiError::Timeout
            } else {
                ApiError::Network(e)
            }
        })?;
    
    let status = response.status();
    if !status.is_success() {
        return Err(ApiError::Http(status));
    }
    
    let data = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| ApiError::Network(e))?;
    
    Ok(data)
}

async fn example_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 示例 7: 错误处理最佳实践 ===\n");
    
    let test_cases = vec![
        ("正常请求", "https://httpbin.org/get"),
        ("404 错误", "https://httpbin.org/status/404"),
        ("500 错误", "https://httpbin.org/status/500"),
    ];
    
    for (name, url) in test_cases {
        println!("测试: {}", name);
        match make_request(url).await {
            Ok(data) => {
                println!("  ✓ 成功");
                println!("  数据长度: {} 字节", data.to_string().len());
            }
            Err(e) => {
                println!("  ✗ 失败: {}", e);
            }
        }
        println!();
    }
    
    Ok(())
}

// ============================================================================
// 主函数
// ============================================================================

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║      reqwest 高级用法实战示例               ║");
    println!("╚══════════════════════════════════════════════╝");
    
    // 示例 1
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 高级客户端配置");
    println!("{}", "=".repeat(60));
    if let Err(e) = create_advanced_client().map(|_| ()) {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 2
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 请求重试机制");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_retry().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 3
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 并发请求与超时");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_concurrent_with_timeout().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 4
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 流式下载");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_streaming_download().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 5
    println!("\n{}", "=".repeat(60));
    println!("运行示例: API 客户端封装");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_api_client().await {
        eprintln!("❌ 示例运行失败: {}", e);
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 示例 6
    println!("\n{}", "=".repeat(60));
    println!("运行示例: 认证方式");
    println!("{}", "=".repeat(60));
    if let Err(e) = example_authentication().await {
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
    
    println!("\n{}", "=".repeat(60));
    println!("所有示例运行完成！");
    println!("{}", "=".repeat(60));
    println!();
}
