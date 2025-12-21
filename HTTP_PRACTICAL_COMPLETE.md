# HTTP 客户端/服务器 实战完成

## 📚 已完成的内容

### 1. 详细文档

#### 📖 完整指南: `docs/HTTP_PRACTICAL_GUIDE.md`
- **HTTP 客户端详解**
  - 基础概念和核心特性
  - GET/POST 请求
  - JSON 处理
  - 高级功能（超时、Cookie、表单、错误处理）
  - 并发请求
  - 流式下载
  - 实战示例

- **HTTP 服务器详解**
  - 基础概念（Axum 框架）
  - 路由系统（路径参数、查询参数）
  - JSON 处理
  - 状态共享
  - 中间件
  - 错误处理
  - 文件操作（上传/下载）
  - WebSocket 支持

- **最佳实践**
  - 客户端最佳实践
  - 服务器最佳实践
  - 安全性建议
  - 性能优化技巧

#### 📝 速查表: `docs/HTTP_CHEATSHEET.md`
- HTTP 客户端快速参考
- HTTP 服务器快速参考
- WebSocket 快速参考
- 完整示例代码
- 常用命令
- 调试技巧

### 2. 可运行的实战示例

#### 🚀 HTTP 客户端: `examples/http_client_practical.rs`
包含 11 个完整示例：

1. **基础 GET 请求** - 使用便捷方法发送简单请求
2. **带查询参数的 GET** - 添加 URL 查询参数
3. **POST JSON** - 发送 JSON 数据
4. **自定义请求头** - 设置 Authorization、User-Agent 等
5. **表单提交** - URL 编码表单数据
6. **超时设置** - 配置连接和总超时
7. **错误处理** - 处理不同类型的错误
8. **Cookie 处理** - Cookie 管理演示
9. **并发请求** - 同时发送多个请求
10. **高级客户端配置** - 连接池、重定向等
11. **API 客户端封装** - 实战 API 客户端设计

**运行命令:**
```bash
cargo run --example http_client_practical --features full
```

#### 🌐 HTTP 服务器: `examples/http_server_practical.rs`
完整的 RESTful API 实现：

**功能特性:**
- ✅ CRUD 操作（创建、读取、更新、删除）
- ✅ 路由系统（路径参数、查询参数）
- ✅ JSON 请求/响应
- ✅ 状态共享（Arc + Mutex）
- ✅ 错误处理（自定义错误类型）
- ✅ 输入验证
- ✅ 统计信息端点
- ✅ 健康检查
- ✅ 单元测试

**API 端点:**
- `GET /api/` - API 信息
- `GET /api/health` - 健康检查
- `GET /api/todos` - 获取所有 Todo
- `GET /api/todos/:id` - 获取单个 Todo
- `POST /api/todos` - 创建新 Todo
- `PUT /api/todos/:id` - 更新 Todo
- `DELETE /api/todos/:id` - 删除 Todo
- `GET /api/stats` - 统计信息

**运行命令:**
```bash
cargo run --example http_server_practical --features full
```

**测试 API:**
```bash
# 获取所有 Todo
curl http://localhost:3000/api/todos

# 创建新 Todo
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"学习 Rust","description":"深入学习 Rust 编程"}'

# 更新 Todo
curl -X PUT http://localhost:3000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed":true}'

# 删除 Todo
curl -X DELETE http://localhost:3000/api/todos/1

# 获取统计信息
curl http://localhost:3000/api/stats
```

#### 💬 WebSocket 聊天室: `examples/websocket_practical.rs`
实时聊天服务器实现：

**功能特性:**
- ✅ 实时双向通信
- ✅ 广播系统（所有用户接收消息）
- ✅ 用户管理（在线列表）
- ✅ 美观的 Web UI（内置 HTML/CSS/JavaScript）
- ✅ 连接状态显示
- ✅ 时间戳
- ✅ 系统消息（用户加入/离开）

**运行命令:**
```bash
cargo run --example websocket_practical --features full
```

**使用方法:**
1. 在浏览器中打开 `http://localhost:3000`
2. 输入用户名进入聊天室
3. 可以打开多个浏览器窗口测试多用户聊天
4. 访问 `http://localhost:3000/stats` 查看在线用户

### 3. 项目配置

#### Cargo.toml 依赖
```toml
[dependencies]
# 异步运行时
tokio = { version = "1.35", features = ["full"] }

# HTTP 客户端
reqwest = { version = "0.11", features = ["json", "stream", "multipart", "rustls-tls"], optional = true }

# HTTP 服务器
axum = { version = "0.7", features = ["ws", "multipart"], optional = true }
tower-http = { version = "0.5", features = ["trace", "cors"], optional = true }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 其他工具
chrono = "0.4"
futures = "0.3"
tokio-util = { version = "0.7", features = ["io"] }

[features]
default = []
full = ["reqwest", "axum", "tower-http"]
```

## 🎯 学习路径

### 初学者
1. 阅读 `docs/HTTP_PRACTICAL_GUIDE.md` 的 **HTTP 客户端详解** 部分
2. 运行 `http_client_practical` 示例，理解每个示例
3. 查看 `docs/HTTP_CHEATSHEET.md` 作为快速参考

### 进阶
4. 阅读 **HTTP 服务器详解** 部分
5. 运行 `http_server_practical` 示例，使用 curl 测试 API
6. 修改示例代码，添加自己的功能

### 高级
7. 运行 `websocket_practical` 示例，理解实时通信
8. 阅读 **最佳实践** 部分
9. 构建自己的 REST API 项目

## 📊 代码统计

- **文档**: 2 个详细文档，超过 1500 行
- **示例代码**: 3 个完整示例，超过 1200 行
- **测试覆盖**: HTTP 服务器包含单元测试
- **注释**: 详细的中文注释和说明

## 🛠️ 技术栈

- **语言**: Rust 1.92+
- **异步运行时**: Tokio
- **HTTP 客户端**: reqwest（使用 rustls-tls）
- **HTTP 服务器**: Axum 0.7
- **WebSocket**: Axum WebSocket 支持
- **序列化**: Serde + serde_json
- **时间处理**: chrono

## ✨ 特色功能

### HTTP 客户端
- ✅ 支持所有 HTTP 方法（GET、POST、PUT、DELETE 等）
- ✅ JSON 自动序列化/反序列化
- ✅ 表单提交（URL 编码和多部分）
- ✅ 文件上传
- ✅ Cookie 管理
- ✅ 自定义请求头
- ✅ 超时控制
- ✅ 错误处理
- ✅ 并发请求
- ✅ 流式下载
- ✅ 连接池
- ✅ HTTPS 支持（rustls）

### HTTP 服务器
- ✅ RESTful API 设计
- ✅ 路由系统
- ✅ 路径参数和查询参数
- ✅ JSON 请求/响应
- ✅ 状态共享
- ✅ 错误处理
- ✅ 中间件支持
- ✅ 文件上传/下载
- ✅ WebSocket 支持
- ✅ CORS 支持
- ✅ 单元测试

### WebSocket
- ✅ 实时双向通信
- ✅ 广播系统
- ✅ 用户管理
- ✅ Web UI
- ✅ 连接管理

## 🧪 测试

### 编译检查
```bash
# 检查所有示例
cargo check --example http_client_practical --features full
cargo check --example http_server_practical --features full
cargo check --example websocket_practical --features full
```

### 运行测试
```bash
# 运行服务器单元测试
cargo test --example http_server_practical --features full
```

## 📝 示例输出

### HTTP 客户端示例
```
╔══════════════════════════════════════════════╗
║      Rust HTTP 客户端实战示例               ║
╚══════════════════════════════════════════════╝

============================================================
运行示例: 基础 GET 请求
============================================================

=== 示例 1: 基础 GET 请求 ===
响应内容: {
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Accept-Encoding": "gzip, br", 
...
```

### HTTP 服务器示例
```
╔══════════════════════════════════════════════╗
║      Rust HTTP 服务器实战示例               ║
║         完整的 REST API                      ║
╚══════════════════════════════════════════════╝

🚀 服务器启动成功！
📍 地址: http://127.0.0.1:3000

📚 可用的 API 端点:
   GET    /api/              - API 信息
   GET    /api/todos         - 获取所有 Todo
   POST   /api/todos         - 创建新 Todo
...

按 Ctrl+C 停止服务器
```

## 🔗 相关资源

### 官方文档
- [Tokio](https://tokio.rs/)
- [reqwest](https://docs.rs/reqwest/)
- [Axum](https://docs.rs/axum/)
- [Serde](https://serde.rs/)

### 学习资源
- Rust 异步编程书：https://rust-lang.github.io/async-book/
- Tokio 教程：https://tokio.rs/tokio/tutorial
- Axum 示例：https://github.com/tokio-rs/axum/tree/main/examples

## 🎉 总结

本项目提供了完整的 Rust HTTP 客户端和服务器实战指南，包括：

1. **详细的文档** - 从基础到高级的完整教程
2. **可运行的示例** - 11 个客户端示例 + 完整的 REST API + WebSocket 聊天室
3. **最佳实践** - 生产级别的代码建议
4. **中文注释** - 易于理解的中文说明

无论你是 Rust 初学者还是有经验的开发者，都可以从这个项目中学到实用的 HTTP 编程技能。

**开始使用:**
```bash
# 1. 运行 HTTP 客户端示例
cargo run --example http_client_practical --features full

# 2. 在另一个终端运行 HTTP 服务器
cargo run --example http_server_practical --features full

# 3. 测试 API
curl http://localhost:3000/api/todos

# 4. 运行 WebSocket 聊天室
cargo run --example websocket_practical --features full
```

祝学习愉快！🚀
