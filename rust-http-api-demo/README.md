# Rust HTTP API 实战示例

这是一个完整的 Rust HTTP API 示例项目，展示了使用 Rust 构建现代 Web 服务的最佳实践。

## 技术栈

- **Web 框架**: Axum - 高性能、类型安全的 Web 框架
- **ORM/数据库**: SQLx - 异步、编译时检查的 SQL 工具包，使用 SQLite
- **异步运行时**: Tokio - Rust 最流行的异步运行时
- **序列化**: Serde - Rust 生态的序列化/反序列化库
- **日志**: Tracing - 结构化、异步的日志框架
- **错误处理**: 自定义错误类型 + anyhow + thiserror

## 项目结构

```
rust-http-api-demo/
├── Cargo.toml              # 项目配置和依赖
├── .env                    # 环境变量
├── README.md              # 项目文档
├── src/
│   ├── main.rs            # 应用入口
│   ├── models/            # 数据模型
│   │   ├── mod.rs
│   │   ├── user.rs        # 用户模型和请求/响应结构
│   │   └── error.rs       # 错误类型定义
│   ├── db/                # 数据访问层 (DAL)
│   │   ├── mod.rs
│   │   ├── pool.rs        # 数据库连接池
│   │   └── user_repository.rs  # 用户 Repository
│   ├── services/          # 业务逻辑层
│   │   ├── mod.rs
│   │   └── user_service.rs     # 用户服务
│   ├── handlers/          # HTTP 处理器
│   │   ├── mod.rs
│   │   ├── health_handler.rs   # 健康检查
│   │   └── user_handler.rs     # 用户 API 处理器
│   └── middleware/        # 中间件
│       ├── mod.rs
│       ├── logging.rs     # 日志中间件
│       └── request_id.rs  # 请求 ID 中间件
```

## 架构设计

本项目采用经典的三层架构：

1. **表示层 (Handlers)**: 处理 HTTP 请求和响应
2. **业务逻辑层 (Services)**: 实现核心业务逻辑
3. **数据访问层 (Repository/DAL)**: 封装数据库操作

### 数据流

```
HTTP Request
    ↓
Middleware (Logging, RequestId)
    ↓
Handler (user_handler.rs)
    ↓
Service (user_service.rs) - 业务逻辑、验证
    ↓
Repository (user_repository.rs) - 数据库操作
    ↓
Database (SQLite)
```

## 快速开始

### 1. 安装依赖

确保已安装 Rust (1.70+):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 克隆项目

```bash
git clone <your-repo>
cd rust-http-api-demo
```

### 3. 配置环境变量

编辑 `.env` 文件：

```env
DATABASE_URL=sqlite://./app.db
RUST_LOG=debug
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

### 4. 构建项目

```bash
cargo build
```

### 5. 运行项目

```bash
cargo run
```

服务器将在 `http://127.0.0.1:3000` 启动。

## API 文档

### 健康检查

**GET /health**

检查服务是否运行正常。

响应示例：
```json
{
  "status": "ok",
  "message": "服务运行正常"
}
```

### 用户 API

#### 1. 创建用户

**POST /api/users**

请求体：
```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "full_name": "John Doe"
}
```

响应（201 Created）：
```json
{
  "id": 1,
  "username": "johndoe",
  "email": "john@example.com",
  "full_name": "John Doe",
  "created_at": "2025-12-03T10:00:00Z",
  "updated_at": "2025-12-03T10:00:00Z"
}
```

#### 2. 获取用户列表

**GET /api/users?page=1&page_size=10**

查询参数：
- `page`: 页码（默认：1）
- `page_size`: 每页数量（默认：10）

响应：
```json
{
  "users": [
    {
      "id": 1,
      "username": "johndoe",
      "email": "john@example.com",
      "full_name": "John Doe",
      "created_at": "2025-12-03T10:00:00Z",
      "updated_at": "2025-12-03T10:00:00Z"
    }
  ],
  "total": 100,
  "page": 1,
  "page_size": 10
}
```

#### 3. 获取单个用户

**GET /api/users/{id}**

响应：
```json
{
  "id": 1,
  "username": "johndoe",
  "email": "john@example.com",
  "full_name": "John Doe",
  "created_at": "2025-12-03T10:00:00Z",
  "updated_at": "2025-12-03T10:00:00Z"
}
```

#### 4. 更新用户

**PUT /api/users/{id}**

请求体（所有字段可选）：
```json
{
  "username": "johndoe_updated",
  "email": "john.new@example.com",
  "full_name": "John Doe Updated"
}
```

响应：
```json
{
  "id": 1,
  "username": "johndoe_updated",
  "email": "john.new@example.com",
  "full_name": "John Doe Updated",
  "created_at": "2025-12-03T10:00:00Z",
  "updated_at": "2025-12-03T11:00:00Z"
}
```

#### 5. 删除用户

**DELETE /api/users/{id}**

响应（200 OK）：
```json
{
  "message": "用户删除成功"
}
```

### 错误响应

所有错误响应都遵循统一格式：

```json
{
  "error": "错误类型",
  "details": "详细错误信息"
}
```

HTTP 状态码：
- 400 Bad Request - 请求验证失败
- 404 Not Found - 资源未找到
- 500 Internal Server Error - 服务器内部错误

## 测试 API

### 使用 curl

```bash
# 健康检查
curl http://localhost:3000/health

# 创建用户
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "full_name": "Test User"
  }'

# 获取用户列表
curl http://localhost:3000/api/users?page=1&page_size=10

# 获取单个用户
curl http://localhost:3000/api/users/1

# 更新用户
curl -X PUT http://localhost:3000/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{
    "username": "updateduser"
  }'

# 删除用户
curl -X DELETE http://localhost:3000/api/users/1
```

### 使用 HTTPie

```bash
# 创建用户
http POST localhost:3000/api/users \
  username=testuser \
  email=test@example.com \
  full_name="Test User"

# 获取用户列表
http GET localhost:3000/api/users page==1 page_size==10

# 获取单个用户
http GET localhost:3000/api/users/1

# 更新用户
http PUT localhost:3000/api/users/1 username=updateduser

# 删除用户
http DELETE localhost:3000/api/users/1
```

## 核心特性

### 1. Repository 模式 (DAL)

使用 Repository 模式封装数据访问逻辑：

```rust
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub async fn create(&self, req: &CreateUserRequest) -> AppResult<User> {
        // 数据库操作
    }
}
```

### 2. Service 层

业务逻辑与数据访问分离：

```rust
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub async fn create_user(&self, req: CreateUserRequest) -> AppResult<User> {
        // 验证
        req.validate()?;
        // 调用 repository
        self.repository.create(&req).await
    }
}
```

### 3. 统一错误处理

自定义错误类型，实现 `IntoResponse`：

```rust
pub enum AppError {
    DatabaseError(sqlx::Error),
    NotFound(String),
    ValidationError(String),
    // ...
}
```

### 4. 中间件

- **日志中间件**: 记录所有请求和响应
- **请求 ID 中间件**: 为每个请求生成唯一 ID

### 5. CORS 支持

配置 CORS 以支持跨域请求。

## 开发指南

### 运行测试

```bash
cargo test
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 性能测试

```bash
cargo build --release
./target/release/rust-http-api-demo
```

## 生产部署建议

1. **使用环境变量**: 敏感信息不要硬编码
2. **启用日志**: 使用结构化日志便于调试
3. **数据库连接池**: 根据负载调整连接池大小
4. **错误处理**: 生产环境不要暴露敏感错误信息
5. **HTTPS**: 使用反向代理（如 Nginx）配置 HTTPS
6. **监控**: 集成监控和告警系统

## 扩展建议

### 1. 添加认证授权

使用 JWT 或 OAuth2：

```toml
[dependencies]
jsonwebtoken = "9.2"
```

### 2. 使用 PostgreSQL

替换 SQLite：

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
```

### 3. 添加缓存

使用 Redis：

```toml
[dependencies]
redis = { version = "0.24", features = ["tokio-comp"] }
```

### 4. API 文档

使用 utoipa 生成 OpenAPI 文档：

```toml
[dependencies]
utoipa = "4.1"
utoipa-swagger-ui = "6.0"
```

## 学习资源

- [Axum 文档](https://docs.rs/axum/)
- [SQLx 文档](https://docs.rs/sqlx/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)
- [Rust Book](https://doc.rust-lang.org/book/)

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
