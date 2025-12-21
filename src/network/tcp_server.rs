// TCP 服务器实现

use std::io::{self, Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// TCP Echo 服务器
/// 
/// 接收客户端发送的数据并原样返回
pub struct EchoServer {
    listener: TcpListener,
}

impl EchoServer {
    /// 创建新的 Echo 服务器
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(EchoServer { listener })
    }
    
    /// 启动服务器（单线程版本）
    pub fn run(&self) -> io::Result<()> {
        println!("Echo 服务器启动在 {}", self.listener.local_addr()?);
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = Self::handle_client(stream) {
                        eprintln!("处理客户端错误: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("连接错误: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// 启动服务器（多线程版本）
    pub fn run_multithreaded(&self) -> io::Result<()> {
        println!("多线程 Echo 服务器启动在 {}", self.listener.local_addr()?);
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_client(stream) {
                            eprintln!("处理客户端错误: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("连接错误: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// 处理单个客户端连接
    fn handle_client(mut stream: TcpStream) -> io::Result<()> {
        let peer_addr = stream.peer_addr()?;
        println!("新连接来自: {}", peer_addr);
        
        // 设置超时
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        stream.set_write_timeout(Some(Duration::from_secs(30)))?;
        
        let mut buffer = [0u8; 1024];
        
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("客户端 {} 断开连接", peer_addr);
                    break;
                }
                Ok(n) => {
                    println!("从 {} 收到 {} 字节", peer_addr, n);
                    
                    // 回显数据
                    if let Err(e) = stream.write_all(&buffer[..n]) {
                        eprintln!("写入错误: {}", e);
                        break;
                    }
                    
                    stream.flush()?;
                }
                Err(e) => {
                    eprintln!("读取错误: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
}

/// 简单的 HTTP 服务器
pub struct SimpleHttpServer {
    listener: TcpListener,
}

impl SimpleHttpServer {
    /// 创建新的 HTTP 服务器
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(SimpleHttpServer { listener })
    }
    
    /// 启动服务器
    pub fn run(&self) -> io::Result<()> {
        println!("HTTP 服务器启动在 http://{}", self.listener.local_addr()?);
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_http_request(stream) {
                            eprintln!("处理 HTTP 请求错误: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("连接错误: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// 处理 HTTP 请求
    fn handle_http_request(mut stream: TcpStream) -> io::Result<()> {
        let buf_reader = BufReader::new(&mut stream);
        let mut lines = buf_reader.lines();
        
        // 读取请求行
        let request_line = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No request line"))??;
        
        println!("请求: {}", request_line);
        
        // 解析请求
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Self::send_response(&mut stream, 400, "Bad Request", "Invalid request line");
        }
        
        let method = parts[0];
        let path = parts[1];
        
        // 简单路由
        match (method, path) {
            ("GET", "/") => {
                Self::send_response(&mut stream, 200, "OK", 
                    "<html><body><h1>欢迎使用 Rust HTTP 服务器</h1><p>这是首页</p></body></html>")
            }
            ("GET", "/about") => {
                Self::send_response(&mut stream, 200, "OK", 
                    "<html><body><h1>关于</h1><p>这是一个用 Rust 编写的简单 HTTP 服务器</p></body></html>")
            }
            ("GET", "/json") => {
                let json = r#"{"message": "Hello, JSON!", "status": "success"}"#;
                Self::send_json_response(&mut stream, 200, "OK", json)
            }
            _ => {
                Self::send_response(&mut stream, 404, "Not Found", 
                    "<html><body><h1>404 - 页面未找到</h1></body></html>")
            }
        }
    }
    
    /// 发送 HTTP 响应
    fn send_response(stream: &mut TcpStream, status_code: u16, status_text: &str, body: &str) -> io::Result<()> {
        let response = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: text/html; charset=utf-8\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            status_code,
            status_text,
            body.len(),
            body
        );
        
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
    
    /// 发送 JSON 响应
    fn send_json_response(stream: &mut TcpStream, status_code: u16, status_text: &str, body: &str) -> io::Result<()> {
        let response = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            status_code,
            status_text,
            body.len(),
            body
        );
        
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}

/// 聊天服务器
pub struct ChatServer {
    listener: TcpListener,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl ChatServer {
    /// 创建新的聊天服务器
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let clients = Arc::new(Mutex::new(Vec::new()));
        Ok(ChatServer { listener, clients })
    }
    
    /// 启动聊天服务器
    pub fn run(&self) -> io::Result<()> {
        println!("聊天服务器启动在 {}", self.listener.local_addr()?);
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let clients = Arc::clone(&self.clients);
                    
                    // 添加新客户端
                    if let Ok(stream_clone) = stream.try_clone() {
                        clients.lock().unwrap().push(stream_clone);
                    }
                    
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_chat_client(stream, clients) {
                            eprintln!("处理聊天客户端错误: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("连接错误: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// 处理聊天客户端
    fn handle_chat_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) -> io::Result<()> {
        let peer_addr = stream.peer_addr()?;
        println!("新用户加入: {}", peer_addr);
        
        // 广播加入消息
        let join_msg = format!("用户 {} 加入聊天室\n", peer_addr);
        Self::broadcast(&clients, &join_msg, Some(&peer_addr))?;
        
        let mut buffer = [0u8; 1024];
        
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("用户 {} 离开", peer_addr);
                    let leave_msg = format!("用户 {} 离开聊天室\n", peer_addr);
                    Self::broadcast(&clients, &leave_msg, Some(&peer_addr))?;
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    let formatted = format!("{}: {}", peer_addr, message);
                    println!("{}", formatted.trim());
                    
                    // 广播消息
                    Self::broadcast(&clients, &formatted, Some(&peer_addr))?;
                }
                Err(e) => {
                    eprintln!("读取错误: {}", e);
                    break;
                }
            }
        }
        
        // 从客户端列表移除
        let mut clients = clients.lock().unwrap();
        clients.retain(|s| s.peer_addr().ok() != Some(peer_addr));
        
        Ok(())
    }
    
    /// 广播消息给所有客户端
    fn broadcast(clients: &Arc<Mutex<Vec<TcpStream>>>, message: &str, exclude: Option<&std::net::SocketAddr>) -> io::Result<()> {
        let mut clients = clients.lock().unwrap();
        
        clients.retain_mut(|client| {
            // 跳过发送者
            if let Some(addr) = exclude {
                if let Ok(peer) = client.peer_addr() {
                    if &peer == addr {
                        return true;
                    }
                }
            }
            
            // 尝试发送消息
            if let Err(e) = client.write_all(message.as_bytes()) {
                eprintln!("发送消息失败: {}", e);
                return false; // 移除失败的连接
            }
            
            client.flush().is_ok()
        });
        
        Ok(())
    }
}

/// 运行示例
pub fn run_examples() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust TCP 服务器示例         ║");
    println!("╚════════════════════════════════════╝\n");
    
    println!("可用的服务器:");
    println!("1. Echo 服务器 - 回显客户端发送的数据");
    println!("2. HTTP 服务器 - 简单的 Web 服务器");
    println!("3. 聊天服务器 - 多用户聊天室");
    println!("\n使用示例:");
    println!("  let server = EchoServer::new(\"127.0.0.1:7878\")?;");
    println!("  server.run_multithreaded()?;");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_echo_server_creation() {
        let result = EchoServer::new("127.0.0.1:0");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_http_server_creation() {
        let result = SimpleHttpServer::new("127.0.0.1:0");
        assert!(result.is_ok());
    }
}
