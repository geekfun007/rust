# Rust 语言完整教程与实战

这是一个完整的 Rust 语言学习资源库，包含详细的理论教程和实战项目。

## 📚 教程内容

### 1. [Rust 语言详解](./Rust语言详解.md)

全面的 Rust 语言教程，涵盖：

- **命令行操作**: Cargo 工具链、项目管理
- **模块化**: 模块系统、可见性、代码组织
- **语法基础**: 变量、数据类型转换、注释
- **数据类型**: 
  - 标量类型（整数、浮点、布尔、字符）
  - 复合类型（元组、数组、切片）
  - 字符串类型（String 与 &str）
  - 集合类型（Vec, HashMap, HashSet）
  - 结构体与枚举
- **逻辑控制**: 
  - 条件语句（if、match、if let）
  - 循环（loop、while、for）
- **函数**: 
  - 函数定义与调用
  - 所有权与借用
  - 生命周期
  - 泛型
  - Trait（特征）
  - 闭包与迭代器
- **IO 操作**: 
  - 标准输入输出
  - 文件操作
  - 错误处理
- **HTTP 编程**: 
  - reqwest（HTTP 客户端）
  - axum（HTTP 服务器）
- **异步编程**: 
  - async/await 基础
  - Future trait
  - Tokio 运行时
  - 异步通道
  - 异步流
  - 异步互斥锁

### 2. [Rust 注意事项](./Rust注意事项.md)

Rust 编程的最佳实践和常见陷阱：

- **所有权系统陷阱**: 值移动、部分移动、循环中的所有权
- **借用检查器**: 多个可变借用、借用共存、悬垂引用
- **生命周期注意事项**: 显式标注、结构体生命周期、'static 误用
- **错误处理**: unwrap 使用、自定义错误、anyhow 应用
- **性能优化**: 避免克隆、预分配、迭代器、字符串选择、内联
- **并发编程**: 数据竞争预防、死锁避免、读写锁使用
- **类型系统**: Sized trait、Copy vs Clone、Deref 强制转换
- **异步编程**: 避免阻塞、Future 取消、Send/Sync 问题
- **依赖管理**: Cargo.toml 配置、workspace 使用
- **测试与调试**: 单元测试、集成测试、调试技巧

### 3. [实战项目说明](./实战项目说明.md)

详细的实战项目架构和实现说明。

## 🚀 实战项目

### [Rust HTTP API Demo](./rust-http-api-demo/)

一个完整的生产级 RESTful API 服务，展示现代 Rust Web 开发：

#### 技术栈
- **Web 框架**: Axum 0.7
- **数据库 ORM**: SQLx 0.7 (SQLite)
- **异步运行时**: Tokio 1.35
- **序列化**: Serde 1.0
- **日志**: Tracing 0.1
- **HTTP 客户端**: Reqwest 0.11

#### 项目特色

✅ **完整的三层架构**
```
Handlers (HTTP 层)
    ↓
Services (业务逻辑层)
    ↓
Repository (数据访问层 - DAL)
    ↓
Database (SQLite)
```

✅ **设计模式**
- Repository 模式（数据访问层）
- Service 模式（业务逻辑）
- 依赖注入
- 统一错误处理

✅ **功能特性**
- RESTful API（用户 CRUD）
- 数据验证
- 错误处理
- 日志中间件
- 请求 ID 追踪
- CORS 支持
- 分页查询

✅ **代码质量**
- 类型安全
- 异步高性能
- 清晰的代码结构
- 完整的注释
- 示例客户端
- 集成测试

#### API 端点

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /health | 健康检查 |
| POST | /api/users | 创建用户 |
| GET | /api/users | 获取用户列表（支持分页） |
| GET | /api/users/:id | 获取单个用户 |
| PUT | /api/users/:id | 更新用户 |
| DELETE | /api/users/:id | 删除用户 |

#### 快速开始

```bash
# 进入项目目录
cd rust-http-api-demo

# 运行服务器
cargo run

# 在另一个终端运行示例客户端
cargo run --example client

# 运行测试
cargo test
```

#### 测试 API

```bash
# 健康检查
curl http://localhost:3000/health

# 创建用户
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","full_name":"Alice Smith"}'

# 获取用户列表
curl http://localhost:3000/api/users?page=1&page_size=10

# 获取用户详情
curl http://localhost:3000/api/users/1

# 更新用户
curl -X PUT http://localhost:3000/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{"username":"alice_updated"}'

# 删除用户
curl -X DELETE http://localhost:3000/api/users/1
```

## 📖 学习路径

### 初学者路径

1. **阅读基础教程** (1-2 周)
   - 命令行操作
   - 模块化
   - 语法基础
   - 数据类型
   - 逻辑控制
   - 函数基础

2. **理解核心概念** (2-3 周)
   - 所有权系统
   - 借用与生命周期
   - 错误处理
   - Trait 系统

3. **实践项目** (1-2 周)
   - 运行实战项目
   - 理解代码结构
   - 修改和扩展功能

### 进阶路径

1. **深入异步编程** (1-2 周)
   - Tokio 运行时
   - async/await
   - Future trait
   - 异步 IO

2. **Web 开发实战** (2-3 周)
   - Axum 框架
   - SQLx ORM
   - 中间件开发
   - API 设计

3. **生产部署** (1 周)
   - 性能优化
   - 监控和日志
   - Docker 容器化
   - 部署实践

## 🛠️ 开发工具推荐

### 必备工具

```bash
# Rust 工具链
rustup           # Rust 版本管理
cargo            # 包管理和构建工具

# 代码质量工具
cargo fmt        # 代码格式化
cargo clippy     # 代码检查
cargo test       # 运行测试

# 开发辅助工具
cargo watch      # 文件监控自动重新编译
cargo edit       # 编辑 Cargo.toml
cargo tree       # 查看依赖树
```

### IDE 推荐

- **VS Code** + rust-analyzer 插件
- **IntelliJ IDEA** + Rust 插件
- **Cursor** (AI 辅助编程)

## 📝 资源链接

### 官方资源
- [The Rust Book](https://doc.rust-lang.org/book/) - 官方教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 示例学习
- [The Rustlings Course](https://github.com/rust-lang/rustlings) - 互动练习
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - API 设计指南

### 框架文档
- [Axum](https://docs.rs/axum/) - Web 框架
- [SQLx](https://docs.rs/sqlx/) - 异步 SQL 工具
- [Tokio](https://tokio.rs/) - 异步运行时
- [Serde](https://serde.rs/) - 序列化框架

### 社区资源
- [Rust 用户论坛](https://users.rust-lang.org/)
- [Rust 中文社区](https://rustcc.cn/)
- [This Week in Rust](https://this-week-in-rust.org/)

## 🎯 项目亮点

### 1. 全面性
- 从基础到高级的完整覆盖
- 理论与实践相结合
- 涵盖 Web 开发全流程

### 2. 实用性
- 生产级代码质量
- 真实的项目结构
- 可直接用于实际项目

### 3. 教学性
- 详细的代码注释
- 清晰的架构说明
- 完整的示例代码

### 4. 现代化
- 使用最新的 Rust 2021 edition
- 采用现代化的技术栈
- 遵循最佳实践

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献方向
- 改进文档说明
- 添加新的示例
- 修复代码错误
- 优化项目结构
- 翻译成其他语言

## 📄 许可证

MIT License

## 🌟 致谢

感谢 Rust 社区的所有贡献者！

---

**祝您学习愉快！Happy Coding! 🦀**
