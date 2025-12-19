// TCP 客户端

use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;

/// # 基本 TCP 客户端
pub fn basic_tcp_client_demo() {
    println!("\n=== 基本 TCP 客户端 ===");
    
    println!("连接服务器:");
    println!("  let mut stream = TcpStream::connect(\"127.0.0.1:7878\")?;");
    println!("  ");
    println!("  // 发送数据");
    println!("  stream.write_all(b\"Hello, Server!\")?;");
    println!("  stream.flush()?;");
    println!("  ");
    println!("  // 接收响应");
    println!("  let mut buffer = [0; 1024];");
    println!("  let n = stream.read(&mut buffer)?;");
    println!("  let response = String::from_utf8_lossy(&buffer[..n]);");
    println!("  println!(\"响应: {{}}\", response);");
}

/// # 设置超时
pub fn timeout_demo() {
    println!("\n=== 设置超时 ===");
    
    println!("连接超时:");
    println!("  let stream = TcpStream::connect_timeout(");
    println!("      &\"127.0.0.1:7878\".parse()?,");
    println!("      Duration::from_secs(5)");
    println!("  )?;");
    
    println!("\n读写超时:");
    println!("  stream.set_read_timeout(Some(Duration::from_secs(5)))?;");
    println!("  stream.set_write_timeout(Some(Duration::from_secs(5)))?;");
}

/// # 持久连接
pub fn persistent_connection_demo() {
    println!("\n=== 持久连接 ===");
    
    println!("保持连接:");
    println!("  let mut stream = TcpStream::connect(\"127.0.0.1:7878\")?;");
    println!("  ");
    println!("  loop {{");
    println!("      // 发送请求");
    println!("      stream.write_all(b\"request\")?;");
    println!("      ");
    println!("      // 接收响应");
    println!("      let mut buffer = [0; 1024];");
    println!("      let n = stream.read(&mut buffer)?;");
    println!("      if n == 0 {{");
    println!("          break;  // 连接关闭");
    println!("      }}");
    println!("      ");
    println!("      // 处理响应");
    println!("      process_response(&buffer[..n]);");
    println!("  }}");
}

/// # 异步 TCP 客户端
pub fn async_tcp_client_demo() {
    println!("\n=== 异步 TCP 客户端 ===");
    
    println!("使用 tokio:");
    println!("  use tokio::net::TcpStream;");
    println!("  use tokio::io::{{AsyncReadExt, AsyncWriteExt}};");
    println!("  ");
    println!("  let mut stream = TcpStream::connect(\"127.0.0.1:7878\").await?;");
    println!("  ");
    println!("  stream.write_all(b\"Hello\").await?;");
    println!("  ");
    println!("  let mut buffer = [0; 1024];");
    println!("  let n = stream.read(&mut buffer).await?;");
}

/// # 实战示例：Echo 客户端
pub fn echo_client_demo() {
    println!("\n=== 实战示例：Echo 客户端 ===");
    
    println!("Echo 客户端:");
    println!("  let mut stream = TcpStream::connect(\"127.0.0.1:7878\")?;");
    println!("  let mut buffer = [0; 1024];");
    println!("  ");
    println!("  loop {{");
    println!("      // 读取用户输入");
    println!("      let mut input = String::new();");
    println!("      io::stdin().read_line(&mut input)?;");
    println!("      ");
    println!("      // 发送到服务器");
    println!("      stream.write_all(input.as_bytes())?;");
    println!("      ");
    println!("      // 接收回显");
    println!("      let n = stream.read(&mut buffer)?;");
    println!("      println!(\"Echo: {{}}\", String::from_utf8_lossy(&buffer[..n]));");
    println!("  }}");
}

/// # 实战示例：HTTP 客户端
pub fn http_client_demo() {
    println!("\n=== 实战示例：HTTP 客户端 ===");
    
    println!("简单 HTTP 请求:");
    println!("  let mut stream = TcpStream::connect(\"example.com:80\")?;");
    println!("  ");
    println!("  let request = \"GET / HTTP/1.1\\r\\n\"");
    println!("                \"Host: example.com\\r\\n\"");
    println!("                \"Connection: close\\r\\n\"");
    println!("                \"\\r\\n\";");
    println!("  ");
    println!("  stream.write_all(request.as_bytes())?;");
    println!("  ");
    println!("  let mut response = String::new();");
    println!("  stream.read_to_string(&mut response)?;");
    println!("  println!(\"响应: {{}}\", response);");
}

/// # TCP 客户端最佳实践
pub fn tcp_client_best_practices_demo() {
    println!("\n=== TCP 客户端最佳实践 ===");
    
    println!("1. 连接管理:");
    println!("   - 设置合理的超时");
    println!("   - 处理连接失败");
    println!("   - 实现重连机制");
    
    println!("\n2. 错误处理:");
    println!("   - 处理网络错误");
    println!("   - 处理超时");
    println!("   - 优雅降级");
    
    println!("\n3. 数据处理:");
    println!("   - 使用缓冲");
    println!("   - 处理部分读写");
    println!("   - 正确处理二进制数据");
    
    println!("\n4. 资源清理:");
    println!("   - 正确关闭连接");
    println!("   - 释放资源");
    
    println!("\n5. 推荐库:");
    println!("   - reqwest: HTTP 客户端");
    println!("   - hyper: 底层 HTTP");
    println!("   - tokio: 异步 I/O");
}

/// 运行所有 TCP 客户端示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust TCP 客户端详解         ║");
    println!("╚════════════════════════════════════╝");
    
    basic_tcp_client_demo();
    timeout_demo();
    persistent_connection_demo();
    async_tcp_client_demo();
    echo_client_demo();
    http_client_demo();
    tcp_client_best_practices_demo();
}
