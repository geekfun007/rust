# HTTP + ORM + DAL 服务端开发完全指南

## 目录
- [1. 架构设计](#1-架构设计)
- [2. 数据访问层（DAL）深入](#2-数据访问层dal深入)
- [3. ORM 详解（SQLx）](#3-orm-详解sqlx)
- [4. HTTP 服务详解（Axum）](#4-http-服务详解axum)
- [5. 完整的 CRUD 实现](#5-完整的-crud-实现)
- [6. 高级特性](#6-高级特性)

---

## 1. 架构设计

### 1.1 分层架构

```
┌──────────────────────────────────────────────┐
│         HTTP Layer (Handlers)                │
│  - 请求验证                                   │
│  - 响应序列化                                 │
│  - 错误转换                                   │
├──────────────────────────────────────────────┤
│      Business Logic Layer (Services)         │
│  - 业务规则                                   │
│  - 数据验证                                   │
│  - 事务协调                                   │
├──────────────────────────────────────────────┤
│    Data Access Layer (Repository/DAL)        │
│  - 数据库操作                                 │
│  - 查询构建                                   │
│  - 数据映射                                   │
├──────────────────────────────────────────────┤
│           Database (PostgreSQL/MySQL)        │
│  - 数据存储                                   │
│  - 约束和索引                                 │
│  - 事务管理                                   │
└──────────────────────────────────────────────┘
```

### 1.2 依赖关系

```rust
// main.rs - 依赖注入示例

use sqlx::PgPool;
use axum::Router;

// 应用状态
#[derive(Clone)]
struct AppState {
    db: PgPool,
    // 其他共享状态
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建数据库连接池
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    
    // 2. 创建 Repository 层
    let user_repo = UserRepository::new(pool.clone());
    let post_repo = PostRepository::new(pool.clone());
    
    // 3. 创建 Service 层
    let user_service = UserService::new(user_repo);
    let post_service = PostService::new(post_repo, user_service.clone());
    
    // 4. 创建应用状态
    let state = AppState {
        user_service,
        post_service,
    };
    
    // 5. 构建路由
    let app = Router::new()
        .nest("/api/users", user_routes())
        .nest("/api/posts", post_routes())
        .with_state(state);
    
    // 6. 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

---

## 2. 数据访问层（DAL）深入

### 2.1 Repository 模式完整实现

```rust
// src/db/user_repository.rs

use sqlx::{PgPool, Postgres, Transaction};
use chrono::{DateTime, Utc};

/// 用户实体（数据库模型）
#[derive(Debug, sqlx::FromRow)]
pub struct UserEntity {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

/// 创建用户参数
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
}

/// 更新用户参数
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: Option<bool>,
}

/// 查询条件
pub struct UserFilter {
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

/// 分页参数
pub struct Pagination {
    pub page: i64,
    pub page_size: i64,
}

impl Pagination {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }
}

/// 用户 Repository
#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// 创建用户
    pub async fn create(&self, params: CreateUser) -> Result<UserEntity, sqlx::Error> {
        let user = sqlx::query_as::<_, UserEntity>(
            r#"
            INSERT INTO users (username, email, password_hash, full_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(&params.username)
        .bind(&params.email)
        .bind(&params.password_hash)
        .bind(&params.full_name)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// 通过 ID 查找用户
    pub async fn find_by_id(&self, id: i64) -> Result<Option<UserEntity>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserEntity>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// 通过用户名查找
    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserEntity>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// 通过邮箱查找
    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserEntity>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// 查询用户列表（带过滤和分页）
    pub async fn find_many(
        &self,
        filter: Option<UserFilter>,
        pagination: Pagination,
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM users WHERE 1=1");
        let mut bindings = Vec::new();
        
        // 动态构建查询条件
        if let Some(filter) = filter {
            if let Some(username) = filter.username {
                bindings.push(username);
                query.push_str(&format!(" AND username LIKE '%' || ${} || '%'", bindings.len()));
            }
            if let Some(email) = filter.email {
                bindings.push(email);
                query.push_str(&format!(" AND email = ${}", bindings.len()));
            }
            if let Some(is_active) = filter.is_active {
                bindings.push(is_active.to_string());
                query.push_str(&format!(" AND is_active = ${}", bindings.len()));
            }
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT $1 OFFSET $2");
        
        let mut db_query = sqlx::query_as::<_, UserEntity>(&query);
        
        // 绑定参数
        for binding in bindings {
            db_query = db_query.bind(binding);
        }
        
        let users = db_query
            .bind(pagination.page_size)
            .bind(pagination.offset())
            .fetch_all(&self.pool)
            .await?;
        
        Ok(users)
    }
    
    /// 更新用户
    pub async fn update(&self, id: i64, params: UpdateUser) -> Result<UserEntity, sqlx::Error> {
        let mut updates = Vec::new();
        let mut bindings = Vec::new();
        
        // 动态构建 UPDATE 语句
        if let Some(username) = params.username {
            bindings.push(username);
            updates.push(format!("username = ${}", bindings.len()));
        }
        if let Some(email) = params.email {
            bindings.push(email);
            updates.push(format!("email = ${}", bindings.len()));
        }
        if let Some(full_name) = params.full_name {
            bindings.push(full_name);
            updates.push(format!("full_name = ${}", bindings.len()));
        }
        if let Some(avatar_url) = params.avatar_url {
            bindings.push(avatar_url);
            updates.push(format!("avatar_url = ${}", bindings.len()));
        }
        if let Some(is_active) = params.is_active {
            bindings.push(is_active.to_string());
            updates.push(format!("is_active = ${}", bindings.len()));
        }
        
        if updates.is_empty() {
            return self.find_by_id(id).await?.ok_or(sqlx::Error::RowNotFound);
        }
        
        updates.push("updated_at = NOW()".to_string());
        
        let query = format!(
            "UPDATE users SET {} WHERE id = ${} RETURNING *",
            updates.join(", "),
            bindings.len() + 1
        );
        
        let mut db_query = sqlx::query_as::<_, UserEntity>(&query);
        for binding in bindings {
            db_query = db_query.bind(binding);
        }
        
        let user = db_query.bind(id).fetch_one(&self.pool).await?;
        
        Ok(user)
    }
    
    /// 删除用户（软删除）
    pub async fn soft_delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// 删除用户（硬删除）
    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// 统计用户数量
    pub async fn count(&self, filter: Option<UserFilter>) -> Result<i64, sqlx::Error> {
        let mut query = String::from("SELECT COUNT(*) FROM users WHERE 1=1");
        let mut bindings = Vec::new();
        
        if let Some(filter) = filter {
            if let Some(username) = filter.username {
                bindings.push(username);
                query.push_str(&format!(" AND username LIKE '%' || ${} || '%'", bindings.len()));
            }
            if let Some(email) = filter.email {
                bindings.push(email);
                query.push_str(&format!(" AND email = ${}", bindings.len()));
            }
            if let Some(is_active) = filter.is_active {
                bindings.push(is_active.to_string());
                query.push_str(&format!(" AND is_active = ${}", bindings.len()));
            }
        }
        
        let mut db_query = sqlx::query_scalar::<_, i64>(&query);
        for binding in bindings {
            db_query = db_query.bind(binding);
        }
        
        let count = db_query.fetch_one(&self.pool).await?;
        
        Ok(count)
    }
    
    /// 批量创建
    pub async fn create_many(&self, users: Vec<CreateUser>) -> Result<Vec<UserEntity>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let mut created_users = Vec::new();
        
        for user in users {
            let created = sqlx::query_as::<_, UserEntity>(
                r#"
                INSERT INTO users (username, email, password_hash, full_name, created_at, updated_at)
                VALUES ($1, $2, $3, $4, NOW(), NOW())
                RETURNING *
                "#,
            )
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.full_name)
            .fetch_one(&mut *tx)
            .await?;
            
            created_users.push(created);
        }
        
        tx.commit().await?;
        
        Ok(created_users)
    }
    
    /// 事务支持
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        self.pool.begin().await
    }
    
    /// 在事务中创建用户
    pub async fn create_in_tx(
        tx: &mut Transaction<'_, Postgres>,
        params: CreateUser,
    ) -> Result<UserEntity, sqlx::Error> {
        let user = sqlx::query_as::<_, UserEntity>(
            r#"
            INSERT INTO users (username, email, password_hash, full_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(&params.username)
        .bind(&params.email)
        .bind(&params.password_hash)
        .bind(&params.full_name)
        .fetch_one(&mut **tx)
        .await?;
        
        Ok(user)
    }
}
```

### 2.2 查询构建器模式

```rust
// src/db/query_builder.rs

use sqlx::{PgPool, Postgres, QueryBuilder};

pub struct UserQueryBuilder<'a> {
    qb: QueryBuilder<'a, Postgres>,
    has_where: bool,
}

impl<'a> UserQueryBuilder<'a> {
    pub fn new() -> Self {
        let mut qb = QueryBuilder::new("SELECT * FROM users");
        Self {
            qb,
            has_where: false,
        }
    }
    
    fn add_where(&mut self) {
        if !self.has_where {
            self.qb.push(" WHERE ");
            self.has_where = true;
        } else {
            self.qb.push(" AND ");
        }
    }
    
    pub fn filter_username(mut self, username: &'a str) -> Self {
        self.add_where();
        self.qb.push("username = ");
        self.qb.push_bind(username);
        self
    }
    
    pub fn filter_email(mut self, email: &'a str) -> Self {
        self.add_where();
        self.qb.push("email = ");
        self.qb.push_bind(email);
        self
    }
    
    pub fn filter_active(mut self, is_active: bool) -> Self {
        self.add_where();
        self.qb.push("is_active = ");
        self.qb.push_bind(is_active);
        self
    }
    
    pub fn order_by(mut self, field: &str, direction: &str) -> Self {
        self.qb.push(" ORDER BY ");
        self.qb.push(field);
        self.qb.push(" ");
        self.qb.push(direction);
        self
    }
    
    pub fn limit(mut self, limit: i64) -> Self {
        self.qb.push(" LIMIT ");
        self.qb.push_bind(limit);
        self
    }
    
    pub fn offset(mut self, offset: i64) -> Self {
        self.qb.push(" OFFSET ");
        self.qb.push_bind(offset);
        self
    }
    
    pub async fn fetch_all(self, pool: &PgPool) -> Result<Vec<UserEntity>, sqlx::Error> {
        self.qb
            .build_query_as::<UserEntity>()
            .fetch_all(pool)
            .await
    }
    
    pub async fn fetch_one(self, pool: &PgPool) -> Result<UserEntity, sqlx::Error> {
        self.qb
            .build_query_as::<UserEntity>()
            .fetch_one(pool)
            .await
    }
}

// 使用示例
async fn query_builder_example(pool: &PgPool) -> Result<Vec<UserEntity>, sqlx::Error> {
    UserQueryBuilder::new()
        .filter_active(true)
        .order_by("created_at", "DESC")
        .limit(10)
        .offset(0)
        .fetch_all(pool)
        .await
}
```

---

## 3. ORM 详解（SQLx）

### 3.1 数据库连接和配置

```rust
// src/db/config.rs

use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Pool, Postgres,
};
use std::time::Duration;

#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/mydb".to_string()),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
        }
    }
}

pub async fn create_pool(config: DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .connect(&config.url)
        .await
}

// 健康检查
pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    Ok(())
}
```

### 3.2 数据库迁移

```rust
// migrations/001_create_users_table.sql
/*
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(100),
    avatar_url TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT username_length CHECK (char_length(username) >= 3),
    CONSTRAINT email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at DESC);
*/

// src/db/migrations.rs

use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    tracing::info!("数据库迁移完成");
    Ok(())
}
```

### 3.3 复杂查询示例

```rust
// src/db/user_queries.rs

use sqlx::PgPool;
use chrono::{DateTime, Utc};

/// 关联查询：用户及其帖子
#[derive(Debug, sqlx::FromRow)]
pub struct UserWithPosts {
    pub user_id: i64,
    pub username: String,
    pub post_count: i64,
}

pub async fn get_users_with_post_count(
    pool: &PgPool,
    limit: i64,
) -> Result<Vec<UserWithPosts>, sqlx::Error> {
    sqlx::query_as::<_, UserWithPosts>(
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            COUNT(p.id) as post_count
        FROM users u
        LEFT JOIN posts p ON u.id = p.user_id
        GROUP BY u.id, u.username
        ORDER BY post_count DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// 子查询：最近活跃用户
pub async fn get_recently_active_users(
    pool: &PgPool,
    days: i32,
) -> Result<Vec<UserEntity>, sqlx::Error> {
    sqlx::query_as::<_, UserEntity>(
        r#"
        SELECT * FROM users
        WHERE id IN (
            SELECT DISTINCT user_id 
            FROM posts 
            WHERE created_at > NOW() - INTERVAL '$1 days'
        )
        ORDER BY last_login_at DESC NULLS LAST
        "#,
    )
    .bind(days)
    .fetch_all(pool)
    .await
}

/// CTE（公共表表达式）示例
pub async fn get_user_statistics(
    pool: &PgPool,
) -> Result<Vec<UserStatistics>, sqlx::Error> {
    #[derive(Debug, sqlx::FromRow)]
    pub struct UserStatistics {
        pub username: String,
        pub total_posts: i64,
        pub total_comments: i64,
        pub total_likes: i64,
    }
    
    sqlx::query_as::<_, UserStatistics>(
        r#"
        WITH user_posts AS (
            SELECT user_id, COUNT(*) as post_count
            FROM posts
            GROUP BY user_id
        ),
        user_comments AS (
            SELECT user_id, COUNT(*) as comment_count
            FROM comments
            GROUP BY user_id
        ),
        user_likes AS (
            SELECT user_id, COUNT(*) as like_count
            FROM likes
            GROUP BY user_id
        )
        SELECT 
            u.username,
            COALESCE(up.post_count, 0) as total_posts,
            COALESCE(uc.comment_count, 0) as total_comments,
            COALESCE(ul.like_count, 0) as total_likes
        FROM users u
        LEFT JOIN user_posts up ON u.id = up.user_id
        LEFT JOIN user_comments uc ON u.id = uc.user_id
        LEFT JOIN user_likes ul ON u.id = ul.user_id
        ORDER BY total_posts DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

/// 全文搜索
pub async fn search_users(
    pool: &PgPool,
    search_term: &str,
) -> Result<Vec<UserEntity>, sqlx::Error> {
    sqlx::query_as::<_, UserEntity>(
        r#"
        SELECT * FROM users
        WHERE 
            to_tsvector('english', username || ' ' || COALESCE(full_name, ''))
            @@ plainto_tsquery('english', $1)
        ORDER BY 
            ts_rank(
                to_tsvector('english', username || ' ' || COALESCE(full_name, '')),
                plainto_tsquery('english', $1)
            ) DESC
        "#,
    )
    .bind(search_term)
    .fetch_all(pool)
    .await
}
```

---

## 4. HTTP 服务详解（Axum）

### 4.1 路由组织

```rust
// src/routes/mod.rs

use axum::{
    Router,
    routing::{get, post, put, delete},
};
use crate::handlers::{user, post, auth};
use crate::middleware::{auth_middleware, logging_middleware};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // 公开路由
        .nest("/api/v1/public", public_routes())
        
        // 需要认证的路由
        .nest("/api/v1/protected", protected_routes())
        
        // 管理员路由
        .nest("/api/v1/admin", admin_routes())
        
        // 健康检查
        .route("/health", get(health_check))
        
        // 状态和中间件
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/users/:id", get(user::get_user))
        .route("/posts", get(post::list_posts))
        .route("/posts/:id", get(post::get_post))
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(user::get_current_user))
        .route("/users/me", put(user::update_current_user))
        .route("/posts", post(post::create_post))
        .route("/posts/:id", put(post::update_post))
        .route("/posts/:id", delete(post::delete_post))
        .layer(axum::middleware::from_fn_with_state(auth_middleware))
}

fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(user::list_all_users))
        .route("/users/:id", delete(user::delete_user))
        .route("/stats", get(stats::get_statistics))
        .layer(axum::middleware::from_fn_with_state(admin_middleware))
}
```

### 4.2 处理器详解

```rust
// src/handlers/user.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 请求 DTO
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    #[validate(length(max = 100))]
    pub full_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(length(max = 100))]
    pub full_name: Option<String>,
    
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    
    #[serde(default = "default_page_size")]
    pub page_size: i64,
    
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 20 }

/// 响应 DTO
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<UserEntity> for UserResponse {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            username: entity.username,
            email: entity.email,
            full_name: entity.full_name,
            avatar_url: entity.avatar_url,
            is_active: entity.is_active,
            created_at: entity.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

/// 处理器实现
pub async fn create_user(
    State(service): State<UserService>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // 验证输入
    req.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;
    
    // 调用服务层
    let user = service.create_user(req).await?;
    
    // 返回响应
    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn get_user(
    State(service): State<UserService>,
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, AppError> {
    let user = service.get_user(id).await?;
    Ok(Json(user.into()))
}

pub async fn list_users(
    State(service): State<UserService>,
    Query(query): Query<UserListQuery>,
) -> Result<Json<UserListResponse>, AppError> {
    let filter = UserFilter {
        username: query.username,
        email: query.email,
        is_active: query.is_active,
    };
    
    let pagination = Pagination {
        page: query.page,
        page_size: query.page_size,
    };
    
    let (users, total) = service.list_users(Some(filter), pagination).await?;
    
    let response = UserListResponse {
        users: users.into_iter().map(Into::into).collect(),
        total,
        page: query.page,
        page_size: query.page_size,
        total_pages: (total + query.page_size - 1) / query.page_size,
    };
    
    Ok(Json(response))
}

pub async fn update_user(
    State(service): State<UserService>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    req.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;
    
    let user = service.update_user(id, req).await?;
    Ok(Json(user.into()))
}

pub async fn delete_user(
    State(service): State<UserService>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

### 4.3 提取器（Extractors）

```rust
// src/extractors/mod.rs

use axum::{
    async_trait,
    extract::{FromRef, FromRequest, FromRequestParts},
    http::{Request, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

/// 验证的 JSON 提取器
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: Deserialize<'static> + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = AppError;
    
    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| AppError::ValidationError("无效的 JSON".to_string()))?;
        
        value.validate()
            .map_err(|e| AppError::ValidationError(format!("{}", e)))?;
        
        Ok(ValidatedJson(value))
    }
}

/// 当前用户提取器
pub struct CurrentUser(pub UserEntity);

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;
    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 从请求头获取 token
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;
        
        // 验证 token 并获取用户
        let app_state = AppState::from_ref(state);
        let user = app_state.auth_service
            .verify_token_and_get_user(token)
            .await?;
        
        Ok(CurrentUser(user))
    }
}

// 使用示例
pub async fn get_current_user(
    CurrentUser(user): CurrentUser,
) -> Json<UserResponse> {
    Json(user.into())
}

pub async fn create_post(
    State(service): State<PostService>,
    CurrentUser(user): CurrentUser,
    ValidatedJson(req): ValidatedJson<CreatePostRequest>,
) -> Result<Json<PostResponse>, AppError> {
    let post = service.create_post(user.id, req).await?;
    Ok(Json(post.into()))
}
```

---

## 5. 完整的 CRUD 实现

### 5.1 服务层实现

```rust
// src/services/user_service.rs

use crate::db::{UserRepository, CreateUser, UpdateUser, UserFilter, Pagination};
use crate::models::{AppError, AppResult};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
    
    /// 创建用户
    pub async fn create_user(&self, req: CreateUserRequest) -> AppResult<UserEntity> {
        // 1. 验证用户名唯一性
        if let Some(_) = self.repository.find_by_username(&req.username).await? {
            return Err(AppError::Conflict("用户名已存在".to_string()));
        }
        
        // 2. 验证邮箱唯一性
        if let Some(_) = self.repository.find_by_email(&req.email).await? {
            return Err(AppError::Conflict("邮箱已被使用".to_string()));
        }
        
        // 3. 密码加密
        let password_hash = hash(req.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?;
        
        // 4. 创建用户
        let params = CreateUser {
            username: req.username,
            email: req.email,
            password_hash,
            full_name: req.full_name,
        };
        
        let user = self.repository.create(params).await?;
        
        tracing::info!("用户创建成功: {}", user.id);
        
        Ok(user)
    }
    
    /// 获取用户
    pub async fn get_user(&self, id: i64) -> AppResult<UserEntity> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::NotFound(format!("用户 {} 不存在", id)))
    }
    
    /// 查询用户列表
    pub async fn list_users(
        &self,
        filter: Option<UserFilter>,
        pagination: Pagination,
    ) -> AppResult<(Vec<UserEntity>, i64)> {
        // 获取用户列表
        let users = self.repository
            .find_many(filter.clone(), pagination)
            .await?;
        
        // 获取总数
        let total = self.repository.count(filter).await?;
        
        Ok((users, total))
    }
    
    /// 更新用户
    pub async fn update_user(&self, id: i64, req: UpdateUserRequest) -> AppResult<UserEntity> {
        // 1. 验证用户存在
        let _ = self.get_user(id).await?;
        
        // 2. 如果更新用户名，验证唯一性
        if let Some(ref username) = req.username {
            if let Some(existing) = self.repository.find_by_username(username).await? {
                if existing.id != id {
                    return Err(AppError::Conflict("用户名已存在".to_string()));
                }
            }
        }
        
        // 3. 如果更新邮箱，验证唯一性
        if let Some(ref email) = req.email {
            if let Some(existing) = self.repository.find_by_email(email).await? {
                if existing.id != id {
                    return Err(AppError::Conflict("邮箱已被使用".to_string()));
                }
            }
        }
        
        // 4. 更新用户
        let params = UpdateUser {
            username: req.username,
            email: req.email,
            full_name: req.full_name,
            avatar_url: req.avatar_url,
            is_active: None,
        };
        
        let user = self.repository.update(id, params).await?;
        
        tracing::info!("用户更新成功: {}", id);
        
        Ok(user)
    }
    
    /// 删除用户
    pub async fn delete_user(&self, id: i64) -> AppResult<()> {
        // 验证用户存在
        let _ = self.get_user(id).await?;
        
        // 软删除
        self.repository.soft_delete(id).await?;
        
        tracing::info!("用户删除成功: {}", id);
        
        Ok(())
    }
    
    /// 验证登录
    pub async fn authenticate(&self, username: &str, password: &str) -> AppResult<UserEntity> {
        // 1. 查找用户
        let user = self.repository
            .find_by_username(username)
            .await?
            .ok_or(AppError::Unauthorized)?;
        
        // 2. 验证密码
        let is_valid = verify(password.as_bytes(), &user.password_hash)
            .map_err(|e| AppError::InternalError(format!("密码验证失败: {}", e)))?;
        
        if !is_valid {
            return Err(AppError::Unauthorized);
        }
        
        // 3. 检查账号是否激活
        if !user.is_active {
            return Err(AppError::Forbidden("账号已被禁用".to_string()));
        }
        
        Ok(user)
    }
    
    /// 修改密码
    pub async fn change_password(
        &self,
        user_id: i64,
        old_password: &str,
        new_password: &str,
    ) -> AppResult<()> {
        // 1. 获取用户
        let user = self.get_user(user_id).await?;
        
        // 2. 验证旧密码
        let is_valid = verify(old_password.as_bytes(), &user.password_hash)
            .map_err(|e| AppError::InternalError(format!("密码验证失败: {}", e)))?;
        
        if !is_valid {
            return Err(AppError::BadRequest("旧密码不正确".to_string()));
        }
        
        // 3. 加密新密码
        let new_hash = hash(new_password.as_bytes(), DEFAULT_COST)
            .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?;
        
        // 4. 更新密码（直接使用 SQL）
        sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
            .bind(&new_hash)
            .bind(user_id)
            .execute(&self.repository.pool)
            .await?;
        
        tracing::info!("用户 {} 修改密码成功", user_id);
        
        Ok(())
    }
}
```

### 5.2 认证服务

```rust
// src/services/auth_service.rs

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,           // 用户 ID
    pub username: String,
    pub exp: i64,           // 过期时间
    pub iat: i64,           // 签发时间
}

#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    jwt_secret: String,
    jwt_expiration: i64,    // 秒
}

impl AuthService {
    pub fn new(user_service: UserService, jwt_secret: String) -> Self {
        Self {
            user_service,
            jwt_secret,
            jwt_expiration: 86400,  // 24小时
        }
    }
    
    /// 用户注册
    pub async fn register(&self, req: CreateUserRequest) -> AppResult<(UserEntity, String)> {
        // 创建用户
        let user = self.user_service.create_user(req).await?;
        
        // 生成 token
        let token = self.generate_token(&user)?;
        
        Ok((user, token))
    }
    
    /// 用户登录
    pub async fn login(&self, username: String, password: String) -> AppResult<(UserEntity, String)> {
        // 验证用户
        let user = self.user_service.authenticate(&username, &password).await?;
        
        // 生成 token
        let token = self.generate_token(&user)?;
        
        // 更新最后登录时间
        sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
            .bind(user.id)
            .execute(&self.user_service.repository.pool)
            .await?;
        
        Ok((user, token))
    }
    
    /// 生成 JWT token
    fn generate_token(&self, user: &UserEntity) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.jwt_expiration);
        
        let claims = Claims {
            sub: user.id,
            username: user.username.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("生成 token 失败: {}", e)))
    }
    
    /// 验证 token
    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
    }
    
    /// 验证 token 并获取用户
    pub async fn verify_token_and_get_user(&self, token: &str) -> AppResult<UserEntity> {
        let claims = self.verify_token(token)?;
        self.user_service.get_user(claims.sub).await
    }
}
```

### 5.3 认证中间件

```rust
// src/middleware/auth.rs

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. 从请求头获取 token
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;
    
    // 2. 验证 token 并获取用户
    let user = state.auth_service
        .verify_token_and_get_user(token)
        .await?;
    
    // 3. 将用户信息存入请求扩展
    request.extensions_mut().insert(user);
    
    // 4. 继续处理请求
    Ok(next.run(request).await)
}

pub async fn admin_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 先执行认证
    let response = auth_middleware(State(state.clone()), request, next).await?;
    
    // 检查是否为管理员
    if let Some(user) = response.extensions().get::<UserEntity>() {
        if user.is_admin {
            Ok(response)
        } else {
            Err(AppError::Forbidden("需要管理员权限".to_string()))
        }
    } else {
        Err(AppError::Unauthorized)
    }
}
```

---

## 6. 高级特性

### 6.1 事务管理

```rust
// src/services/post_service.rs

use sqlx::Postgres;

#[derive(Clone)]
pub struct PostService {
    post_repo: PostRepository,
    user_repo: UserRepository,
}

impl PostService {
    /// 创建帖子（带事务）
    pub async fn create_post_with_tags(
        &self,
        user_id: i64,
        title: String,
        content: String,
        tags: Vec<String>,
    ) -> AppResult<PostEntity> {
        // 开启事务
        let mut tx = self.post_repo.pool.begin().await?;
        
        // 1. 创建帖子
        let post = sqlx::query_as::<_, PostEntity>(
            "INSERT INTO posts (user_id, title, content, created_at) VALUES ($1, $2, $3, NOW()) RETURNING *"
        )
        .bind(user_id)
        .bind(&title)
        .bind(&content)
        .fetch_one(&mut *tx)
        .await?;
        
        // 2. 创建标签关联
        for tag in tags {
            sqlx::query(
                "INSERT INTO post_tags (post_id, tag_name) VALUES ($1, $2)"
            )
            .bind(post.id)
            .bind(&tag)
            .execute(&mut *tx)
            .await?;
        }
        
        // 3. 更新用户帖子数
        sqlx::query(
            "UPDATE users SET post_count = post_count + 1 WHERE id = $1"
        )
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
        
        // 提交事务
        tx.commit().await?;
        
        Ok(post)
    }
    
    /// 删除帖子（带级联）
    pub async fn delete_post(&self, post_id: i64, user_id: i64) -> AppResult<()> {
        let mut tx = self.post_repo.pool.begin().await?;
        
        // 1. 验证帖子所有权
        let post = sqlx::query_as::<_, PostEntity>(
            "SELECT * FROM posts WHERE id = $1 AND user_id = $2"
        )
        .bind(post_id)
        .bind(user_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(AppError::NotFound("帖子不存在".to_string()))?;
        
        // 2. 删除评论
        sqlx::query("DELETE FROM comments WHERE post_id = $1")
            .bind(post_id)
            .execute(&mut *tx)
            .await?;
        
        // 3. 删除标签关联
        sqlx::query("DELETE FROM post_tags WHERE post_id = $1")
            .bind(post_id)
            .execute(&mut *tx)
            .await?;
        
        // 4. 删除帖子
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(post_id)
            .execute(&mut *tx)
            .await?;
        
        // 5. 更新用户帖子数
        sqlx::query("UPDATE users SET post_count = post_count - 1 WHERE id = $1")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        
        tx.commit().await?;
        
        Ok(())
    }
}
```

### 6.2 缓存层

```rust
// src/cache/mod.rs

use redis::{Client, AsyncCommands};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct CacheService {
    client: Client,
}

impl CacheService {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }
    
    /// 获取缓存
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, redis::RedisError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.client.get_async_connection().await?;
        let data: Option<String> = conn.get(key).await?;
        
        match data {
            Some(json) => {
                let value: T = serde_json::from_str(&json)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "反序列化失败", e.to_string())))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    
    /// 设置缓存
    pub async fn set<T>(
        &self,
        key: &str,
        value: &T,
        expiration: usize,
    ) -> Result<(), redis::RedisError>
    where
        T: Serialize,
    {
        let mut conn = self.client.get_async_connection().await?;
        let json = serde_json::to_string(value)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "序列化失败", e.to_string())))?;
        
        conn.set_ex(key, json, expiration).await?;
        Ok(())
    }
    
    /// 删除缓存
    pub async fn delete(&self, key: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del(key).await?;
        Ok(())
    }
    
    /// 批量删除（通过模式）
    pub async fn delete_pattern(&self, pattern: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_async_connection().await?;
        let keys: Vec<String> = conn.keys(pattern).await?;
        
        if !keys.is_empty() {
            conn.del(keys).await?;
        }
        
        Ok(())
    }
}

// 带缓存的服务层
impl UserService {
    /// 获取用户（带缓存）
    pub async fn get_user_cached(&self, id: i64) -> AppResult<UserEntity> {
        let cache_key = format!("user:{}", id);
        
        // 1. 尝试从缓存获取
        if let Some(user) = self.cache.get::<UserEntity>(&cache_key).await? {
            tracing::debug!("缓存命中: {}", cache_key);
            return Ok(user);
        }
        
        // 2. 从数据库获取
        let user = self.repository.find_by_id(id).await?
            .ok_or(AppError::NotFound(format!("用户 {} 不存在", id)))?;
        
        // 3. 写入缓存（5分钟过期）
        self.cache.set(&cache_key, &user, 300).await?;
        
        Ok(user)
    }
    
    /// 更新用户（清除缓存）
    pub async fn update_user_cached(&self, id: i64, req: UpdateUserRequest) -> AppResult<UserEntity> {
        // 1. 更新数据库
        let user = self.update_user(id, req).await?;
        
        // 2. 删除缓存
        let cache_key = format!("user:{}", id);
        self.cache.delete(&cache_key).await?;
        
        Ok(user)
    }
}
```

### 6.3 分页助手

```rust
// src/utils/pagination.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 20 }

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.page < 1 {
            return Err("页码必须大于 0".to_string());
        }
        if self.page_size < 1 || self.page_size > 100 {
            return Err("每页数量必须在 1-100 之间".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, params: PaginationParams, total: i64) -> Self {
        let total_pages = (total + params.page_size - 1) / params.page_size;
        
        Self {
            data,
            pagination: PaginationMeta {
                page: params.page,
                page_size: params.page_size,
                total,
                total_pages,
                has_next: params.page < total_pages,
                has_prev: params.page > 1,
            },
        }
    }
}
```

### 6.4 批量操作

```rust
// src/services/batch_operations.rs

impl UserService {
    /// 批量创建用户
    pub async fn create_users_batch(
        &self,
        users: Vec<CreateUserRequest>,
    ) -> AppResult<Vec<UserEntity>> {
        let mut tx = self.repository.pool.begin().await?;
        let mut created_users = Vec::new();
        
        for user_req in users {
            // 验证
            if let Some(_) = self.repository.find_by_username(&user_req.username).await? {
                continue;  // 跳过已存在的用户
            }
            
            // 加密密码
            let password_hash = hash(user_req.password.as_bytes(), DEFAULT_COST)
                .map_err(|e| AppError::InternalError(format!("密码加密失败: {}", e)))?;
            
            // 创建用户
            let user = sqlx::query_as::<_, UserEntity>(
                "INSERT INTO users (username, email, password_hash, full_name, created_at, updated_at) VALUES ($1, $2, $3, $4, NOW(), NOW()) RETURNING *"
            )
            .bind(&user_req.username)
            .bind(&user_req.email)
            .bind(&password_hash)
            .bind(&user_req.full_name)
            .fetch_one(&mut *tx)
            .await?;
            
            created_users.push(user);
        }
        
        tx.commit().await?;
        
        Ok(created_users)
    }
    
    /// 批量更新用户状态
    pub async fn update_users_status(
        &self,
        user_ids: Vec<i64>,
        is_active: bool,
    ) -> AppResult<i64> {
        let result = sqlx::query(
            "UPDATE users SET is_active = $1, updated_at = NOW() WHERE id = ANY($2)"
        )
        .bind(is_active)
        .bind(&user_ids)
        .execute(&self.repository.pool)
        .await?;
        
        Ok(result.rows_affected() as i64)
    }
    
    /// 批量删除用户
    pub async fn delete_users_batch(&self, user_ids: Vec<i64>) -> AppResult<i64> {
        let result = sqlx::query(
            "UPDATE users SET is_active = false, updated_at = NOW() WHERE id = ANY($1)"
        )
        .bind(&user_ids)
        .execute(&self.repository.pool)
        .await?;
        
        Ok(result.rows_affected() as i64)
    }
}
```

### 6.5 文件上传

```rust
// src/handlers/upload.rs

use axum::extract::Multipart;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn upload_avatar(
    CurrentUser(user): CurrentUser,
    State(service): State<UserService>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        
        if name == "avatar" {
            let filename = field.file_name()
                .ok_or(AppError::BadRequest("缺少文件名".to_string()))?
                .to_string();
            
            // 验证文件类型
            let content_type = field.content_type()
                .ok_or(AppError::BadRequest("缺少文件类型".to_string()))?
                .to_string();
            
            if !content_type.starts_with("image/") {
                return Err(AppError::BadRequest("只允许上传图片".to_string()));
            }
            
            // 生成唯一文件名
            let ext = filename.split('.').last().unwrap_or("jpg");
            let new_filename = format!("{}_{}.{}", user.id, Uuid::new_v4(), ext);
            let file_path = format!("uploads/avatars/{}", new_filename);
            
            // 确保目录存在
            fs::create_dir_all("uploads/avatars").await?;
            
            // 保存文件
            let data = field.bytes().await.unwrap();
            
            // 验证文件大小（2MB）
            if data.len() > 2 * 1024 * 1024 {
                return Err(AppError::BadRequest("文件大小不能超过 2MB".to_string()));
            }
            
            let mut file = fs::File::create(&file_path).await?;
            file.write_all(&data).await?;
            
            // 更新用户头像
            let avatar_url = format!("/static/avatars/{}", new_filename);
            service.update_avatar(user.id, avatar_url.clone()).await?;
            
            return Ok(Json(UploadResponse {
                url: avatar_url,
                filename: new_filename,
            }));
        }
    }
    
    Err(AppError::BadRequest("未找到文件字段".to_string()))
}

#[derive(Serialize)]
struct UploadResponse {
    url: String,
    filename: String,
}
```

### 6.6 导出功能

```rust
// src/handlers/export.rs

use csv::Writer;
use axum::response::IntoResponse;

pub async fn export_users_csv(
    State(service): State<UserService>,
) -> Result<impl IntoResponse, AppError> {
    // 获取所有用户
    let users = service.get_all_users().await?;
    
    // 创建 CSV
    let mut wtr = Writer::from_writer(vec![]);
    
    // 写入表头
    wtr.write_record(&["ID", "Username", "Email", "Full Name", "Created At"])?;
    
    // 写入数据
    for user in users {
        wtr.write_record(&[
            user.id.to_string(),
            user.username,
            user.email,
            user.full_name.unwrap_or_default(),
            user.created_at.to_rfc3339(),
        ])?;
    }
    
    let data = wtr.into_inner()?;
    
    // 返回 CSV 响应
    let headers = [
        ("Content-Type", "text/csv"),
        ("Content-Disposition", "attachment; filename=users.csv"),
    ];
    
    Ok((headers, data))
}
```

---

## 7. 性能优化

### 7.1 连接池优化

```rust
// 连接池配置
let pool = PgPoolOptions::new()
    .max_connections(20)                    // 最大连接数
    .min_connections(5)                     // 最小连接数
    .acquire_timeout(Duration::from_secs(3)) // 获取连接超时
    .idle_timeout(Duration::from_secs(600))  // 空闲超时
    .max_lifetime(Duration::from_secs(1800)) // 连接最大生命周期
    .test_before_acquire(true)              // 获取前测试连接
    .connect(&database_url)
    .await?;
```

### 7.2 索引优化

```sql
-- 为常用查询字段创建索引
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at DESC);
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = true;

-- 复合索引
CREATE INDEX idx_posts_user_created ON posts(user_id, created_at DESC);

-- 全文搜索索引
CREATE INDEX idx_users_fulltext ON users USING gin(to_tsvector('english', username || ' ' || COALESCE(full_name, '')));
```

### 7.3 查询优化

```rust
// ❌ N+1 查询问题
async fn get_users_with_posts_bad(pool: &PgPool) -> Result<Vec<UserWithPosts>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(pool).await?;
    
    let mut result = Vec::new();
    for user in users {
        // 每个用户都执行一次查询！
        let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE user_id = $1")
            .bind(user.id)
            .fetch_all(pool)
            .await?;
        
        result.push(UserWithPosts { user, posts });
    }
    
    Ok(result)
}

// ✅ 使用 JOIN 优化
async fn get_users_with_posts_good(pool: &PgPool) -> Result<Vec<UserWithPosts>> {
    sqlx::query_as::<_, UserWithPosts>(
        r#"
        SELECT 
            u.*,
            json_agg(p.*) as posts
        FROM users u
        LEFT JOIN posts p ON u.id = p.user_id
        GROUP BY u.id
        "#
    )
    .fetch_all(pool)
    .await
}
```

---

## 8. 错误处理最佳实践

```rust
// src/models/error.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("验证错误: {0}")]
    ValidationError(String),
    
    #[error("未找到: {0}")]
    NotFound(String),
    
    #[error("冲突: {0}")]
    Conflict(String),
    
    #[error("未授权")]
    Unauthorized,
    
    #[error("禁止访问: {0}")]
    Forbidden(String),
    
    #[error("请求错误: {0}")]
    BadRequest(String),
    
    #[error("内部错误: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(ref e) => {
                tracing::error!("数据库错误: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误")
            }
            AppError::ValidationError(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::Conflict(ref msg) => (StatusCode::CONFLICT, msg.as_str()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未授权"),
            AppError::Forbidden(ref msg) => (StatusCode::FORBIDDEN, msg.as_str()),
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalError(ref msg) => {
                tracing::error!("内部错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "内部错误")
            }
        };
        
        let body = Json(json!({
            "error": error_message,
            "details": self.to_string(),
        }));
        
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
```

---

## 总结

本指南涵盖了 Rust HTTP + ORM + DAL 服务端开发的核心内容：

✅ **架构设计** - 清晰的三层架构  
✅ **数据访问层** - Repository 模式和查询构建器  
✅ **ORM 使用** - SQLx 完整实践  
✅ **HTTP 服务** - Axum 框架深入  
✅ **完整 CRUD** - 从数据库到 API 的完整流程  
✅ **高级特性** - 事务、缓存、批量操作、文件上传  
✅ **性能优化** - 连接池、索引、查询优化  
✅ **错误处理** - 统一的错误处理机制  

这是一个生产级的服务端开发指南！🚀
