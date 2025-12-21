// HTTP 客户端模块
//
// 本模块提供 HTTP 客户端功能的说明和示例
// 完整的实战代码请参考: examples/http_client_practical.rs 和 examples/reqwest_advanced.rs

/// HTTP 客户端概述
///
/// Rust 生态系统中主要的 HTTP 客户端库是 `reqwest`
///
/// # 特性
/// - 异步和同步 API
/// - 自动连接池管理
/// - JSON 序列化/反序列化
/// - 文件上传/下载
/// - Cookie 管理
/// - 代理支持
/// - TLS/HTTPS 支持
pub mod overview {
    pub const DESCRIPTION: &str = r#"
reqwest 是 Rust 最流行的 HTTP 客户端库

核心特性:
- 基于 hyper 和 tokio
- 自动连接池和 keep-alive
- 支持 HTTP/1.1 和 HTTP/2
- JSON 自动序列化
- 流式上传和下载
- 重定向处理
- 压缩支持 (gzip, br)
"#;
}

/// 基础用法示例
pub mod basics {
    /// 简单的 GET 请求示例
    pub const GET_EXAMPLE: &str = r#"
use reqwest;

// 简单 GET 请求
let response = reqwest::get("https://api.example.com/data")
    .await?
    .text()
    .await?;

println!("Response: {}", response);
"#;

    /// POST 请求示例
    pub const POST_EXAMPLE: &str = r#"
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
}

let client = Client::new();
let user = User {
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

let response = client
    .post("https://api.example.com/users")
    .json(&user)
    .send()
    .await?;
"#;
}

/// 客户端配置
pub mod configuration {
    /// 客户端构建器示例
    pub const CLIENT_BUILDER: &str = r#"
use reqwest::Client;
use std::time::Duration;

let client = Client::builder()
    // 超时设置
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
    .user_agent("MyApp/1.0")
    
    .build()?;
"#;
}

/// 高级功能
pub mod advanced {
    /// 并发请求示例
    pub const CONCURRENT_REQUESTS: &str = r#"
use futures::future::join_all;

let urls = vec![
    "https://api.example.com/data/1",
    "https://api.example.com/data/2",
    "https://api.example.com/data/3",
];

let client = Client::new();

let futures = urls.into_iter().map(|url| {
    let client = client.clone();
    async move { client.get(url).send().await }
});

let results = join_all(futures).await;
"#;

    /// 文件上传示例
    pub const FILE_UPLOAD: &str = r#"
use reqwest::multipart;

let form = multipart::Form::new()
    .text("name", "profile.jpg")
    .file("file", "/path/to/image.jpg")
    .await?;

let response = client
    .post("https://api.example.com/upload")
    .multipart(form)
    .send()
    .await?;
"#;

    /// 流式下载示例
    pub const STREAMING_DOWNLOAD: &str = r#"
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

let response = client
    .get("https://example.com/large_file.zip")
    .send()
    .await?;

let mut file = tokio::fs::File::create("output.zip").await?;
let mut stream = response.bytes_stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    file.write_all(&chunk).await?;
}
"#;
}

/// 错误处理
pub mod error_handling {
    /// 错误处理最佳实践
    pub const ERROR_HANDLING: &str = r#"
// 基础错误处理
match client.get(url).send().await {
    Ok(response) => {
        if response.status().is_success() {
            let body = response.text().await?;
            println!("Success: {}", body);
        } else {
            eprintln!("HTTP Error: {}", response.status());
        }
    }
    Err(e) => {
        if e.is_timeout() {
            eprintln!("Request timed out");
        } else if e.is_connect() {
            eprintln!("Connection failed");
        } else {
            eprintln!("Error: {}", e);
        }
    }
}

// 使用 error_for_status
let response = client
    .get(url)
    .send()
    .await?
    .error_for_status()?;  // 非 2xx 返回错误
"#;
}

/// 运行示例和文档
pub fn print_documentation() {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║       Rust HTTP 客户端 (reqwest)          ║");
    println!("╚════════════════════════════════════════════╝\n");
    
    println!("{}", overview::DESCRIPTION);
    
    println!("\n📖 详细文档:");
    println!("   docs/HTTP_PRACTICAL_GUIDE.md");
    println!("   docs/REQWEST_AXUM_COMPLETE.md");
    println!("   docs/HTTP_CHEATSHEET.md");
    
    println!("\n🚀 完整示例:");
    println!("   examples/http_client_practical.rs");
    println!("   examples/reqwest_advanced.rs");
    
    println!("\n▶️  运行示例:");
    println!("   cargo run --example http_client_practical --features full");
    println!("   cargo run --example reqwest_advanced --features full");
    
    println!("\n💡 快速开始:");
    println!("{}", basics::GET_EXAMPLE);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_documentation_exists() {
        assert!(!overview::DESCRIPTION.is_empty());
        assert!(!basics::GET_EXAMPLE.is_empty());
        assert!(!basics::POST_EXAMPLE.is_empty());
    }
}
