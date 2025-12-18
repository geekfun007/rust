// HTTP 客户端

/// # reqwest 基础
pub fn reqwest_basics_demo() {
    println!("\n=== reqwest 基础 ===");
    
    println!("GET 请求:");
    println!("  let response = reqwest::get(\"https://api.example.com/users\")");
    println!("      .await?");
    println!("      .text()");
    println!("      .await?;");
    println!("  println!(\"响应: {{}}\", response);");
    
    println!("\nPOST 请求:");
    println!("  let client = reqwest::Client::new();");
    println!("  let response = client");
    println!("      .post(\"https://api.example.com/users\")");
    println!("      .json(&user_data)");
    println!("      .send()");
    println!("      .await?;");
}

/// # JSON 处理
pub fn json_handling_demo() {
    println!("\n=== JSON 处理 ===");
    
    println!("发送 JSON:");
    println!("  use serde::{{Serialize, Deserialize}};");
    println!("  ");
    println!("  #[derive(Serialize)]");
    println!("  struct User {{");
    println!("      name: String,");
    println!("      email: String,");
    println!("  }}");
    println!("  ");
    println!("  let user = User {{");
    println!("      name: \"Alice\".to_string(),");
    println!("      email: \"alice@example.com\".to_string(),");
    println!("  }};");
    println!("  ");
    println!("  let response = client");
    println!("      .post(\"https://api.example.com/users\")");
    println!("      .json(&user)");
    println!("      .send()");
    println!("      .await?;");
    
    println!("\n接收 JSON:");
    println!("  #[derive(Deserialize)]");
    println!("  struct ApiResponse {{");
    println!("      id: u32,");
    println!("      name: String,");
    println!("  }}");
    println!("  ");
    println!("  let data: ApiResponse = response.json().await?;");
    println!("  println!(\"ID: {{}}, Name: {{}}\", data.id, data.name);");
}

/// # 请求配置
pub fn request_configuration_demo() {
    println!("\n=== 请求配置 ===");
    
    println!("设置请求头:");
    println!("  let response = client");
    println!("      .get(\"https://api.example.com/data\")");
    println!("      .header(\"Authorization\", \"Bearer token\")");
    println!("      .header(\"User-Agent\", \"MyApp/1.0\")");
    println!("      .send()");
    println!("      .await?;");
    
    println!("\n查询参数:");
    println!("  let response = client");
    println!("      .get(\"https://api.example.com/search\")");
    println!("      .query(&[(\"q\", \"rust\"), (\"page\", \"1\")])");
    println!("      .send()");
    println!("      .await?;");
    
    println!("\n超时设置:");
    println!("  use std::time::Duration;");
    println!("  ");
    println!("  let client = reqwest::Client::builder()");
    println!("      .timeout(Duration::from_secs(10))");
    println!("      .build()?;");
}

/// # 表单提交
pub fn form_submission_demo() {
    println!("\n=== 表单提交 ===");
    
    println!("URL编码表单:");
    println!("  let params = [(\"username\", \"alice\"), (\"password\", \"secret\")];");
    println!("  ");
    println!("  let response = client");
    println!("      .post(\"https://api.example.com/login\")");
    println!("      .form(&params)");
    println!("      .send()");
    println!("      .await?;");
    
    println!("\n多部分表单（文件上传）:");
    println!("  use reqwest::multipart;");
    println!("  ");
    println!("  let form = multipart::Form::new()");
    println!("      .text(\"name\", \"image.jpg\")");
    println!("      .file(\"file\", \"/path/to/image.jpg\")?;");
    println!("  ");
    println!("  let response = client");
    println!("      .post(\"https://api.example.com/upload\")");
    println!("      .multipart(form)");
    println!("      .send()");
    println!("      .await?;");
}

/// # Cookie 处理
pub fn cookie_handling_demo() {
    println!("\n=== Cookie 处理 ===");
    
    println!("启用 Cookie:");
    println!("  let client = reqwest::Client::builder()");
    println!("      .cookie_store(true)");
    println!("      .build()?;");
    println!("  ");
    println!("  // 第一次请求设置 Cookie");
    println!("  client.get(\"https://api.example.com/login\").send().await?;");
    println!("  ");
    println!("  // 后续请求自动携带 Cookie");
    println!("  client.get(\"https://api.example.com/profile\").send().await?;");
}

/// # 错误处理
pub fn error_handling_demo() {
    println!("\n=== 错误处理 ===");
    
    println!("检查状态码:");
    println!("  let response = client.get(url).send().await?;");
    println!("  ");
    println!("  if response.status().is_success() {{");
    println!("      let body = response.text().await?;");
    println!("      println!(\"成功: {{}}\", body);");
    println!("  }} else {{");
    println!("      eprintln!(\"错误: {{}}\", response.status());");
    println!("  }}");
    
    println!("\n自动错误处理:");
    println!("  let response = client");
    println!("      .get(url)");
    println!("      .send()");
    println!("      .await?");
    println!("      .error_for_status()?;  // 非 2xx 状态码返回错误");
}

/// # 代理设置
pub fn proxy_demo() {
    println!("\n=== 代理设置 ===");
    
    println!("HTTP 代理:");
    println!("  let proxy = reqwest::Proxy::http(\"http://proxy.example.com:8080\")?;");
    println!("  ");
    println!("  let client = reqwest::Client::builder()");
    println!("      .proxy(proxy)");
    println!("      .build()?;");
    
    println!("\nHTTPS 代理:");
    println!("  let proxy = reqwest::Proxy::https(\"https://proxy.example.com:8080\")?;");
}

/// # 并发请求
pub fn concurrent_requests_demo() {
    println!("\n=== 并发请求 ===");
    
    println!("同时发送多个请求:");
    println!("  let urls = vec![");
    println!("      \"https://api.example.com/data/1\",");
    println!("      \"https://api.example.com/data/2\",");
    println!("      \"https://api.example.com/data/3\",");
    println!("  ];");
    println!("  ");
    println!("  let client = reqwest::Client::new();");
    println!("  let futures: Vec<_> = urls");
    println!("      .into_iter()");
    println!("      .map(|url| client.get(url).send())");
    println!("      .collect();");
    println!("  ");
    println!("  let results = futures::future::join_all(futures).await;");
    println!("  ");
    println!("  for result in results {{");
    println!("      match result {{");
    println!("          Ok(response) => println!(\"成功: {{}}\", response.status()),");
    println!("          Err(e) => eprintln!(\"错误: {{}}\", e),");
    println!("      }}");
    println!("  }}");
}

/// # 流式下载
pub fn streaming_download_demo() {
    println!("\n=== 流式下载 ===");
    
    println!("下载大文件:");
    println!("  use tokio::io::AsyncWriteExt;");
    println!("  ");
    println!("  let response = client");
    println!("      .get(\"https://example.com/large_file.zip\")");
    println!("      .send()");
    println!("      .await?;");
    println!("  ");
    println!("  let mut file = tokio::fs::File::create(\"output.zip\").await?;");
    println!("  let mut stream = response.bytes_stream();");
    println!("  ");
    println!("  while let Some(chunk) = stream.next().await {{");
    println!("      let chunk = chunk?;");
    println!("      file.write_all(&chunk).await?;");
    println!("  }}");
}

/// # 实战示例：API 客户端
pub fn api_client_demo() {
    println!("\n=== 实战示例：API 客户端 ===");
    
    println!("构建 API 客户端:");
    println!("  struct ApiClient {{");
    println!("      client: reqwest::Client,");
    println!("      base_url: String,");
    println!("      api_key: String,");
    println!("  }}");
    println!("  ");
    println!("  impl ApiClient {{");
    println!("      fn new(base_url: String, api_key: String) -> Self {{");
    println!("          let client = reqwest::Client::new();");
    println!("          ApiClient {{ client, base_url, api_key }}");
    println!("      }}");
    println!("      ");
    println!("      async fn get_user(&self, id: u32) -> Result<User, Error> {{");
    println!("          let url = format!(\"{{}}/users/{{}}\", self.base_url, id);");
    println!("          ");
    println!("          let response = self.client");
    println!("              .get(&url)");
    println!("              .header(\"Authorization\", format!(\"Bearer {{}}\", self.api_key))");
    println!("              .send()");
    println!("              .await?");
    println!("              .error_for_status()?;");
    println!("          ");
    println!("          let user = response.json::<User>().await?;");
    println!("          Ok(user)");
    println!("      }}");
    println!("  }}");
}

/// # HTTP 客户端最佳实践
pub fn http_client_best_practices_demo() {
    println!("\n=== HTTP 客户端最佳实践 ===");
    
    println!("1. 客户端复用:");
    println!("   - 创建一个 Client 实例并复用");
    println!("   - 避免为每个请求创建新客户端");
    println!("   - 连接池自动管理");
    
    println!("\n2. 超时设置:");
    println!("   - 总是设置超时");
    println!("   - 连接超时和读取超时");
    println!("   - 避免无限等待");
    
    println!("\n3. 错误处理:");
    println!("   - 处理网络错误");
    println!("   - 检查状态码");
    println!("   - 实现重试机制");
    
    println!("\n4. 性能优化:");
    println!("   - 使用连接池");
    println!("   - 并发请求");
    println!("   - HTTP/2 支持");
    println!("   - 压缩");
    
    println!("\n5. 安全性:");
    println!("   - HTTPS");
    println!("   - 证书验证");
    println!("   - 敏感信息保护");
    
    println!("\n6. 推荐配置:");
    println!("   let client = reqwest::Client::builder()");
    println!("       .timeout(Duration::from_secs(30))");
    println!("       .pool_max_idle_per_host(10)");
    println!("       .http2_prior_knowledge()");
    println!("       .gzip(true)");
    println!("       .build()?;");
}

/// 运行所有 HTTP 客户端示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║       Rust HTTP 客户端详解         ║");
    println!("╚════════════════════════════════════╝");
    
    reqwest_basics_demo();
    json_handling_demo();
    request_configuration_demo();
    form_submission_demo();
    cookie_handling_demo();
    error_handling_demo();
    proxy_demo();
    concurrent_requests_demo();
    streaming_download_demo();
    api_client_demo();
    http_client_best_practices_demo();
}
