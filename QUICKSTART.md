# 快速开始指南

## 📦 安装

确保已安装 Rust (1.70+):

```bash
# 检查 Rust 版本
rustc --version

# 如未安装，运行:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🚀 运行示例

### 1. 类型转换 (.ok_/.to_/.as_)

```bash
cargo run --bin conversions
```

**学习内容**:
- `.ok()` - Result → Option 转换
- `.ok_or()` / `.ok_or_else()` - Option → Result 转换
- `.to_string()`, `.to_owned()`, `.to_vec()` - 创建新值
- `.as_str()`, `.as_ref()`, `.as_bytes()` - 引用转换
- `.into()` - 消费转换

### 2. 错误处理

```bash
cargo run --bin error_handling
```

**学习内容**:
- `Result<T, E>` 和 `Option<T>` 基础
- `?` 操作符的使用
- `unwrap`, `expect`, `unwrap_or` 系列
- 自定义错误类型
- `thiserror` 库（库开发）
- `anyhow` 库（应用开发）

### 3. HTTP 客户端

```bash
cargo run --bin http_client
```

**学习内容**:
- 基础 GET/POST 请求
- 查询参数和请求头
- JSON 序列化/反序列化
- 错误处理
- 并发请求
- 客户端配置

**注意**: 需要网络连接

### 4. HTTP 服务器

```bash
cargo run --bin http_server
```

启动后访问: http://localhost:3000

**学习内容**:
- RESTful API 设计
- 路由定义
- 提取器 (Path, Query, Json, State)
- 中间件 (CORS, 日志)
- 状态管理
- 错误处理

**测试命令**:
```bash
# 获取所有用户
curl http://localhost:3000/api/users

# 创建用户
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"测试","email":"test@example.com","age":25}'

# 获取单个用户
curl http://localhost:3000/api/users/1
```

### 5. 文件系统操作

```bash
cargo run --bin file_operations
```

**学习内容**:
- 文件读写（多种方式）
- `OpenOptions` 详解
- 缓冲 I/O
- 文件定位 (Seek)
- 目录操作
- 路径处理
- 实战案例

## 🛠️ 开发命令

```bash
# 检查代码（不生成二进制）
cargo check

# 构建所有示例
cargo build

# 构建发布版本
cargo build --release

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 清理构建产物
cargo clean
```

## 📁 项目结构

```
rust-core-concepts/
├── Cargo.toml              # 项目配置
├── README.md               # 完整文档
├── QUICKSTART.md          # 本文件
├── .gitignore
├── examples/
│   └── quick_test.sh      # 测试脚本
└── src/
    ├── conversions.rs     # 类型转换
    ├── error_handling.rs  # 错误处理
    ├── http_client.rs     # HTTP 客户端
    ├── http_server.rs     # HTTP 服务器
    └── file_operations.rs # 文件操作
```

## 🎯 推荐学习顺序

### 初学者
1. **conversions.rs** - 理解类型转换
2. **error_handling.rs** - 掌握错误处理
3. **file_operations.rs** - 练习 I/O 操作

### 进阶
4. **http_client.rs** - 异步编程
5. **http_server.rs** - Web 框架

## 💡 常见问题

### Q: 编译很慢怎么办？
A: 首次编译需要下载和编译依赖，可能需要几分钟。后续编译会快很多。可以使用 `cargo check` 进行快速检查。

### Q: 如何只编译某个示例？
A: 使用 `cargo build --bin <name>` 或 `cargo run --bin <name>`

### Q: 如何启用发布优化？
A: 添加 `--release` 标志：`cargo run --release --bin conversions`

### Q: HTTP 客户端示例失败？
A: 确保有网络连接。示例使用 jsonplaceholder.typicode.com 作为测试API。

### Q: 端口 3000 被占用？
A: 修改 `src/http_server.rs` 中的 `addr` 变量，改为其他端口。

## 🔗 相关资源

- [主文档](./README.md) - 完整的教程和API说明
- [Rust 官方书](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio 文档](https://tokio.rs/)
- [Axum 文档](https://docs.rs/axum/)

## 📝 下一步

1. 运行所有示例，理解输出
2. 阅读源代码中的注释
3. 尝试修改代码，观察结果
4. 查看 [README.md](./README.md) 获取详细说明

祝学习愉快！🦀✨
