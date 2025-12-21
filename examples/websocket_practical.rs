// WebSocket 实战示例 - 实时聊天服务器
// 运行命令: cargo run --example websocket_practical --features full

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Html,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

// ============================================================================
// 数据结构
// ============================================================================

/// 聊天消息
#[derive(Clone, Debug)]
struct ChatMessage {
    username: String,
    content: String,
    timestamp: String,
}

/// 应用状态
struct AppState {
    // 广播通道，用于向所有客户端发送消息
    tx: broadcast::Sender<ChatMessage>,
    // 在线用户列表
    users: Mutex<HashMap<usize, String>>,
    // 用户 ID 计数器
    next_user_id: Mutex<usize>,
}

// ============================================================================
// HTML 客户端页面
// ============================================================================

async fn index() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>WebSocket 聊天室</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }
        
        .container {
            background: white;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            max-width: 800px;
            width: 100%;
            overflow: hidden;
        }
        
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            text-align: center;
        }
        
        .header h1 {
            font-size: 24px;
            margin-bottom: 5px;
        }
        
        .status {
            font-size: 14px;
            opacity: 0.9;
        }
        
        .status.connected {
            color: #4ade80;
        }
        
        .status.disconnected {
            color: #f87171;
        }
        
        .login-form {
            padding: 40px;
            text-align: center;
        }
        
        .login-form input {
            width: 100%;
            max-width: 300px;
            padding: 15px;
            font-size: 16px;
            border: 2px solid #e5e7eb;
            border-radius: 10px;
            margin-bottom: 15px;
        }
        
        .login-form button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 15px 40px;
            font-size: 16px;
            border-radius: 10px;
            cursor: pointer;
            transition: transform 0.2s;
        }
        
        .login-form button:hover {
            transform: translateY(-2px);
        }
        
        .chat-container {
            display: none;
        }
        
        .chat-container.active {
            display: block;
        }
        
        .messages {
            height: 400px;
            overflow-y: auto;
            padding: 20px;
            background: #f9fafb;
        }
        
        .message {
            margin-bottom: 15px;
            padding: 12px 15px;
            border-radius: 10px;
            background: white;
            box-shadow: 0 2px 5px rgba(0,0,0,0.05);
        }
        
        .message.system {
            background: #fef3c7;
            text-align: center;
            font-style: italic;
        }
        
        .message-header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 5px;
            font-size: 12px;
        }
        
        .username {
            font-weight: bold;
            color: #667eea;
        }
        
        .timestamp {
            color: #9ca3af;
        }
        
        .message-content {
            color: #374151;
        }
        
        .input-area {
            padding: 20px;
            background: white;
            border-top: 1px solid #e5e7eb;
            display: flex;
            gap: 10px;
        }
        
        .input-area input {
            flex: 1;
            padding: 12px 15px;
            font-size: 14px;
            border: 2px solid #e5e7eb;
            border-radius: 10px;
        }
        
        .input-area button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 12px 30px;
            font-size: 14px;
            border-radius: 10px;
            cursor: pointer;
            transition: transform 0.2s;
        }
        
        .input-area button:hover {
            transform: translateY(-2px);
        }
        
        .input-area button:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 Rust WebSocket 聊天室</h1>
            <div class="status" id="status">未连接</div>
        </div>
        
        <div class="login-form" id="loginForm">
            <h2 style="margin-bottom: 20px; color: #374151;">请输入您的用户名</h2>
            <input type="text" id="usernameInput" placeholder="用户名" maxlength="20">
            <br>
            <button onclick="connect()">进入聊天室</button>
        </div>
        
        <div class="chat-container" id="chatContainer">
            <div class="messages" id="messages"></div>
            <div class="input-area">
                <input type="text" id="messageInput" placeholder="输入消息..." onkeypress="if(event.key==='Enter')sendMessage()">
                <button onclick="sendMessage()">发送</button>
            </div>
        </div>
    </div>

    <script>
        let ws = null;
        let username = '';
        
        function connect() {
            username = document.getElementById('usernameInput').value.trim();
            
            if (!username) {
                alert('请输入用户名');
                return;
            }
            
            const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            const wsUrl = `${protocol}//${window.location.host}/ws?username=${encodeURIComponent(username)}`;
            
            ws = new WebSocket(wsUrl);
            
            ws.onopen = () => {
                console.log('WebSocket 连接已建立');
                document.getElementById('status').textContent = '✓ 已连接';
                document.getElementById('status').className = 'status connected';
                document.getElementById('loginForm').style.display = 'none';
                document.getElementById('chatContainer').classList.add('active');
                document.getElementById('messageInput').focus();
            };
            
            ws.onmessage = (event) => {
                const data = JSON.parse(event.data);
                addMessage(data);
            };
            
            ws.onerror = (error) => {
                console.error('WebSocket 错误:', error);
                document.getElementById('status').textContent = '✗ 连接错误';
                document.getElementById('status').className = 'status disconnected';
            };
            
            ws.onclose = () => {
                console.log('WebSocket 连接已关闭');
                document.getElementById('status').textContent = '✗ 已断开';
                document.getElementById('status').className = 'status disconnected';
            };
        }
        
        function sendMessage() {
            const input = document.getElementById('messageInput');
            const message = input.value.trim();
            
            if (!message || !ws) return;
            
            ws.send(message);
            input.value = '';
        }
        
        function addMessage(data) {
            const messagesDiv = document.getElementById('messages');
            const messageDiv = document.createElement('div');
            
            if (data.type === 'system') {
                messageDiv.className = 'message system';
                messageDiv.innerHTML = `<div class="message-content">${data.content}</div>`;
            } else {
                messageDiv.className = 'message';
                messageDiv.innerHTML = `
                    <div class="message-header">
                        <span class="username">${data.username}</span>
                        <span class="timestamp">${data.timestamp}</span>
                    </div>
                    <div class="message-content">${data.content}</div>
                `;
            }
            
            messagesDiv.appendChild(messageDiv);
            messagesDiv.scrollTop = messagesDiv.scrollHeight;
        }
        
        // 自动聚焦用户名输入框
        document.getElementById('usernameInput').focus();
    </script>
</body>
</html>
    "#)
}

// ============================================================================
// WebSocket 处理
// ============================================================================

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    
    // 生成用户 ID
    let user_id = {
        let mut next_id = state.next_user_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    };
    
    // 订阅广播通道
    let mut rx = state.tx.subscribe();
    
    let username = format!("用户{}", user_id);
    
    // 发送欢迎消息给新用户
    let welcome_msg = serde_json::json!({
        "type": "system",
        "content": format!("欢迎来到聊天室! 你是 {}", username),
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
    });
    
    if sender.send(Message::Text(welcome_msg.to_string())).await.is_err() {
        return;
    }
    
    // 广播用户加入消息
    let join_msg = ChatMessage {
        username: "系统".to_string(),
        content: format!("{} 加入了聊天室", username),
        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
    };
    let _ = state.tx.send(join_msg);
    
    // 添加用户到在线列表
    state.users.lock().unwrap().insert(user_id, username.clone());
    
    println!("✓ 用户 {} (ID: {}) 已连接", username, user_id);
    
    // 发送任务：从广播通道接收消息并发送给客户端
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json_msg = serde_json::json!({
                "username": msg.username,
                "content": msg.content,
                "timestamp": msg.timestamp,
            });
            
            if sender.send(Message::Text(json_msg.to_string())).await.is_err() {
                break;
            }
        }
    });
    
    // 接收任务：从客户端接收消息并广播
    let tx = state.tx.clone();
    let username_clone = username.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // 处理文本消息
                    let chat_msg = ChatMessage {
                        username: username_clone.clone(),
                        content: text,
                        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                    };
                    
                    println!("📨 {} 发送: {}", chat_msg.username, chat_msg.content);
                    
                    // 广播消息
                    let _ = tx.send(chat_msg);
                }
                Message::Close(_) => {
                    println!("✗ 用户 {} 断开连接", username_clone);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // 等待任一任务完成
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
    
    // 清理：从在线列表移除用户
    state.users.lock().unwrap().remove(&user_id);
    
    // 广播用户离开消息
    let leave_msg = ChatMessage {
        username: "系统".to_string(),
        content: format!("{} 离开了聊天室", username),
        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
    };
    let _ = state.tx.send(leave_msg);
    
    println!("✗ 用户 {} (ID: {}) 已断开", username, user_id);
}

// ============================================================================
// 统计信息
// ============================================================================

async fn stats(State(state): State<Arc<AppState>>) -> axum::Json<serde_json::Value> {
    let users = state.users.lock().unwrap();
    let online_users: Vec<String> = users.values().cloned().collect();
    
    axum::Json(serde_json::json!({
        "online_count": users.len(),
        "online_users": online_users,
    }))
}

// ============================================================================
// 主函数
// ============================================================================

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║      WebSocket 实战示例                      ║");
    println!("║         实时聊天服务器                       ║");
    println!("╚══════════════════════════════════════════════╝\n");
    
    // 创建广播通道（容量 100）
    let (tx, _rx) = broadcast::channel(100);
    
    // 创建共享状态
    let app_state = Arc::new(AppState {
        tx,
        users: Mutex::new(HashMap::new()),
        next_user_id: Mutex::new(1),
    });
    
    // 构建应用
    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(websocket_handler))
        .route("/stats", get(stats))
        .with_state(app_state);
    
    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 服务器启动成功！");
    println!("📍 地址: http://{}", addr);
    println!("\n💡 使用说明:");
    println!("   1. 在浏览器中打开 http://localhost:3000");
    println!("   2. 输入用户名进入聊天室");
    println!("   3. 可以打开多个浏览器窗口测试多用户聊天");
    println!("   4. 访问 http://localhost:3000/stats 查看在线用户");
    println!("\n按 Ctrl+C 停止服务器\n");
    println!("{}", "=".repeat(60));
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
