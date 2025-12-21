// TCP 客户端实现

use std::io::{self, Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

/// TCP 客户端
pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    /// 连接到服务器
    pub fn connect(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        println!("已连接到 {}", addr);
        Ok(TcpClient { stream })
    }
    
    /// 带超时的连接
    pub fn connect_timeout(addr: &str, timeout: Duration) -> io::Result<Self> {
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        
        let stream = TcpStream::connect_timeout(&socket_addr, timeout)?;
        println!("已连接到 {} (超时: {:?})", addr, timeout);
        Ok(TcpClient { stream })
    }
    
    /// 设置读取超时
    pub fn set_read_timeout(&mut self, timeout: Option<Duration>) -> io::Result<()> {
        self.stream.set_read_timeout(timeout)
    }
    
    /// 设置写入超时
    pub fn set_write_timeout(&mut self, timeout: Option<Duration>) -> io::Result<()> {
        self.stream.set_write_timeout(timeout)
    }
    
    /// 发送数据
    pub fn send(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data)?;
        self.stream.flush()?;
        Ok(())
    }
    
    /// 发送字符串
    pub fn send_string(&mut self, message: &str) -> io::Result<()> {
        self.send(message.as_bytes())
    }
    
    /// 接收数据
    pub fn receive(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buffer)
    }
    
    /// 接收所有数据直到连接关闭
    pub fn receive_all(&mut self) -> io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.stream.read_to_end(&mut data)?;
        Ok(data)
    }
    
    /// 接收字符串
    pub fn receive_string(&mut self) -> io::Result<String> {
        let data = self.receive_all()?;
        String::from_utf8(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
    
    /// 获取本地地址
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.stream.local_addr()
    }
    
    /// 获取对端地址
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}

/// Echo 客户端 - 用于测试 Echo 服务器
pub struct EchoClient {
    client: TcpClient,
}

impl EchoClient {
    /// 连接到 Echo 服务器
    pub fn connect(addr: &str) -> io::Result<Self> {
        let client = TcpClient::connect(addr)?;
        Ok(EchoClient { client })
    }
    
    /// 发送消息并接收回显
    pub fn echo(&mut self, message: &str) -> io::Result<String> {
        println!("发送: {}", message);
        
        // 发送消息
        self.client.send_string(message)?;
        
        // 接收回显
        let mut buffer = vec![0u8; message.len()];
        let n = self.client.receive(&mut buffer)?;
        
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("收到: {}", response);
        
        Ok(response)
    }
    
    /// 交互式 Echo 会话
    pub fn interactive_session(&mut self) -> io::Result<()> {
        println!("Echo 客户端启动 (输入 'quit' 退出)");
        
        let stdin = io::stdin();
        let mut input = String::new();
        
        loop {
            print!("> ");
            io::stdout().flush()?;
            
            input.clear();
            stdin.read_line(&mut input)?;
            
            let trimmed = input.trim();
            if trimmed == "quit" {
                break;
            }
            
            match self.echo(trimmed) {
                Ok(response) => {
                    println!("回显: {}", response);
                }
                Err(e) => {
                    eprintln!("错误: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
}

/// 简单的 HTTP 客户端
pub struct SimpleHttpClient;

impl SimpleHttpClient {
    /// 发送 GET 请求
    pub fn get(url: &str) -> io::Result<String> {
        // 解析 URL
        let (host, path) = Self::parse_url(url)?;
        
        // 连接到服务器
        let addr = format!("{}:80", host);
        let mut client = TcpClient::connect(&addr)?;
        
        // 构建 HTTP 请求
        let request = format!(
            "GET {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: close\r\n\
             User-Agent: Rust-SimpleHttpClient/1.0\r\n\
             \r\n",
            path, host
        );
        
        // 发送请求
        client.send_string(&request)?;
        
        // 接收响应
        client.receive_string()
    }
    
    /// 简单的 URL 解析
    fn parse_url(url: &str) -> io::Result<(String, String)> {
        // 移除协议
        let url = url.trim_start_matches("http://");
        
        // 分割 host 和 path
        let parts: Vec<&str> = url.splitn(2, '/').collect();
        let host = parts[0].to_string();
        let path = if parts.len() > 1 {
            format!("/{}", parts[1])
        } else {
            "/".to_string()
        };
        
        Ok((host, path))
    }
}

/// 聊天客户端
pub struct ChatClient {
    client: TcpClient,
}

impl ChatClient {
    /// 连接到聊天服务器
    pub fn connect(addr: &str) -> io::Result<Self> {
        let client = TcpClient::connect(addr)?;
        println!("已连接到聊天服务器");
        Ok(ChatClient { client })
    }
    
    /// 发送消息
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.client.send_string(&format!("{}\n", message))
    }
    
    /// 启动交互式聊天会话
    pub fn start_session(&mut self) -> io::Result<()> {
        println!("聊天客户端启动 (输入 '/quit' 退出)");
        println!("您可以开始发送消息...\n");
        
        // 克隆 stream 用于接收线程
        let mut stream_clone = self.client.stream.try_clone()?;
        
        // 启动接收线程
        std::thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                match stream_clone.read(&mut buffer) {
                    Ok(0) => {
                        println!("\n服务器断开连接");
                        break;
                    }
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        print!("{}", message);
                        io::stdout().flush().ok();
                    }
                    Err(e) => {
                        eprintln!("\n接收错误: {}", e);
                        break;
                    }
                }
            }
        });
        
        // 主线程处理用户输入
        let stdin = io::stdin();
        let mut input = String::new();
        
        loop {
            input.clear();
            stdin.read_line(&mut input)?;
            
            let trimmed = input.trim();
            if trimmed == "/quit" {
                println!("退出聊天...");
                break;
            }
            
            if !trimmed.is_empty() {
                self.send_message(trimmed)?;
            }
        }
        
        Ok(())
    }
}

/// 运行示例
pub fn run_examples() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust TCP 客户端示例         ║");
    println!("╚════════════════════════════════════╝\n");
    
    println!("可用的客户端:");
    println!("1. 基础 TCP 客户端 - 通用 TCP 连接");
    println!("2. Echo 客户端 - 测试 Echo 服务器");
    println!("3. HTTP 客户端 - 简单的 HTTP GET 请求");
    println!("4. 聊天客户端 - 连接到聊天服务器");
    println!("\n使用示例:");
    println!("  let mut client = TcpClient::connect(\"127.0.0.1:7878\")?;");
    println!("  client.send_string(\"Hello, Server!\")?;");
    println!("  let mut buffer = [0u8; 1024];");
    println!("  let n = client.receive(&mut buffer)?;");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_url_parsing() {
        let (host, path) = SimpleHttpClient::parse_url("http://example.com/test").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(path, "/test");
    }
    
    #[test]
    fn test_url_parsing_no_path() {
        let (host, path) = SimpleHttpClient::parse_url("http://example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(path, "/");
    }
}
