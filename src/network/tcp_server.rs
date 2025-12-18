// TCP 服务器

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

/// # 基本 TCP 服务器
pub fn basic_tcp_server_demo() {
    println!("\n=== 基本 TCP 服务器 ===");
    
    println!("TCP 服务器示例代码:");
    println!("  let listener = TcpListener::bind(\"127.0.0.1:7878\")?;");
    println!("  for stream in listener.incoming() {{");
    println!("      let stream = stream?;");
    println!("      handle_client(stream);");
    println!("  }}");
    
    println!("\n实际运行会阻塞，这里仅展示代码结构");
}

/// # 处理客户端连接
pub fn handle_client_demo() {
    println!("\n=== 处理客户端连接 ===");
    
    println!("处理客户端函数:");
    println!("  fn handle_client(mut stream: TcpStream) -> io::Result<()> {{");
    println!("      let mut buffer = [0; 1024];");
    println!("      let n = stream.read(&mut buffer)?;");
    println!("      ");
    println!("      let request = String::from_utf8_lossy(&buffer[..n]);");
    println!("      println!(\"收到请求: {{}}\", request);");
    println!("      ");
    println!("      let response = \"HTTP/1.1 200 OK\\r\\n\\r\\nHello!\";");
    println!("      stream.write_all(response.as_bytes())?;");
    println!("      stream.flush()?;");
    println!("      Ok(())");
    println!("  }}");
}

/// # 多线程 TCP 服务器
pub fn multithreaded_server_demo() {
    println!("\n=== 多线程 TCP 服务器 ===");
    
    println!("多线程处理连接:");
    println!("  let listener = TcpListener::bind(\"127.0.0.1:7878\")?;");
    println!("  ");
    println!("  for stream in listener.incoming() {{");
    println!("      let stream = stream?;");
    println!("      ");
    println!("      thread::spawn(move || {{");
    println!("          handle_client(stream);");
    println!("      }});");
    println!("  }}");
    
    println!("\n优点:");
    println!("  - 并发处理多个客户端");
    println!("  - 简单直接");
    
    println!("\n缺点:");
    println!("  - 每个连接一个线程，开销大");
    println!("  - 建议使用线程池");
}

/// # 线程池 TCP 服务器
pub fn thread_pool_server_demo() {
    println!("\n=== 线程池 TCP 服务器 ===");
    
    println!("使用线程池:");
    println!("  let listener = TcpListener::bind(\"127.0.0.1:7878\")?;");
    println!("  let pool = ThreadPool::new(4);");
    println!("  ");
    println!("  for stream in listener.incoming() {{");
    println!("      let stream = stream?;");
    println!("      ");
    println!("      pool.execute(move || {{");
    println!("          handle_client(stream);");
    println!("      }});");
    println!("  }}");
    
    println!("\n优点:");
    println!("  - 固定数量的线程");
    println!("  - 更好的资源控制");
    println!("  - 适合生产环境");
}

/// # 异步 TCP 服务器
pub fn async_tcp_server_demo() {
    println!("\n=== 异步 TCP 服务器 ===");
    
    println!("使用 tokio:");
    println!("  #[tokio::main]");
    println!("  async fn main() -> io::Result<()> {{");
    println!("      let listener = TcpListener::bind(\"127.0.0.1:7878\").await?;");
    println!("      ");
    println!("      loop {{");
    println!("          let (socket, _) = listener.accept().await?;");
    println!("          ");
    println!("          tokio::spawn(async move {{");
    println!("              handle_client(socket).await;");
    println!("          }});");
    println!("      }}");
    println!("  }}");
    
    println!("\n优点:");
    println!("  - 高并发性能");
    println!("  - 资源利用率高");
    println!("  - 适合 I/O 密集型应用");
}

/// # Echo 服务器示例
pub fn echo_server_demo() {
    println!("\n=== Echo 服务器示例 ===");
    
    println!("Echo 服务器（返回客户端发送的内容）:");
    println!("  fn handle_client(mut stream: TcpStream) -> io::Result<()> {{");
    println!("      let mut buffer = [0; 1024];");
    println!("      ");
    println!("      loop {{");
    println!("          let n = stream.read(&mut buffer)?;");
    println!("          if n == 0 {{");
    println!("              return Ok(());  // 连接关闭");
    println!("          }}");
    println!("          ");
    println!("          stream.write_all(&buffer[..n])?;");
    println!("          stream.flush()?;");
    println!("      }}");
    println!("  }}");
}

/// # 聊天服务器示例
pub fn chat_server_demo() {
    println!("\n=== 聊天服务器示例 ===");
    
    println!("聊天服务器结构:");
    println!("  1. 维护客户端列表");
    println!("  2. 广播消息给所有客户端");
    println!("  3. 处理客户端加入/离开");
    
    println!("\n关键代码:");
    println!("  let clients = Arc::new(Mutex::new(Vec::new()));");
    println!("  ");
    println!("  // 新客户端");
    println!("  let clients_clone = Arc::clone(&clients);");
    println!("  clients_clone.lock().unwrap().push(stream.try_clone()?);");
    println!("  ");
    println!("  // 广播消息");
    println!("  for client in clients.lock().unwrap().iter_mut() {{");
    println!("      client.write_all(message.as_bytes())?;");
    println!("  }}");
}

/// # HTTP 服务器基础
pub fn http_server_basics_demo() {
    println!("\n=== HTTP 服务器基础 ===");
    
    println!("简单 HTTP 响应:");
    println!("  let response = format!(");
    println!("      \"HTTP/1.1 200 OK\\r\\n\"");
    println!("      \"Content-Type: text/html\\r\\n\"");
    println!("      \"Content-Length: {{}}\\r\\n\"");
    println!("      \"\\r\\n\"");
    println!("      \"{{}}\",");
    println!("      body.len(),");
    println!("      body");
    println!("  );");
    println!("  stream.write_all(response.as_bytes())?;");
}

/// # 实战示例：简单 Web 服务器
pub fn simple_web_server_demo() {
    println!("\n=== 实战示例：简单 Web 服务器 ===");
    
    println!("Web 服务器功能:");
    println!("  - 处理 GET 请求");
    println!("  - 返回 HTML 页面");
    println!("  - 提供静态文件");
    println!("  - 404 错误处理");
    
    println!("\n路由示例:");
    println!("  match (method, path) {{");
    println!("      (\"GET\", \"/\") => {{");
    println!("          response = \"HTTP/1.1 200 OK\\r\\n\\r\\n<h1>首页</h1>\";");
    println!("      }}");
    println!("      (\"GET\", \"/about\") => {{");
    println!("          response = \"HTTP/1.1 200 OK\\r\\n\\r\\n<h1>关于</h1>\";");
    println!("      }}");
    println!("      _ => {{");
    println!("          response = \"HTTP/1.1 404 NOT FOUND\\r\\n\\r\\n<h1>404</h1>\";");
    println!("      }}");
    println!("  }}");
}

/// # TCP 服务器最佳实践
pub fn tcp_server_best_practices_demo() {
    println!("\n=== TCP 服务器最佳实践 ===");
    
    println!("1. 错误处理:");
    println!("   - 处理所有可能的 I/O 错误");
    println!("   - 记录错误日志");
    println!("   - 优雅地处理客户端断开");
    
    println!("\n2. 并发模型选择:");
    println!("   - 低并发: 多线程");
    println!("   - 高并发: 异步 (tokio)");
    println!("   - 混合: 线程池");
    
    println!("\n3. 资源管理:");
    println!("   - 设置超时");
    println!("   - 限制连接数");
    println!("   - 清理僵尸连接");
    
    println!("\n4. 安全性:");
    println!("   - 验证输入");
    println!("   - 限流");
    println!("   - DDoS 防护");
    
    println!("\n5. 监控和日志:");
    println!("   - 连接数统计");
    println!("   - 性能监控");
    println!("   - 详细日志");
    
    println!("\n6. 推荐库:");
    println!("   - tokio: 异步运行时");
    println!("   - async-std: 另一个异步运行时");
    println!("   - mio: 底层事件循环");
}

/// 运行所有 TCP 服务器示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust TCP 服务器详解         ║");
    println!("╚════════════════════════════════════╝");
    
    basic_tcp_server_demo();
    handle_client_demo();
    multithreaded_server_demo();
    thread_pool_server_demo();
    async_tcp_server_demo();
    echo_server_demo();
    chat_server_demo();
    http_server_basics_demo();
    simple_web_server_demo();
    tcp_server_best_practices_demo();
}
