use crate::models::{AppResult, CreateUserRequest, UpdateUserRequest, User};
use sqlx::SqlitePool;
use chrono::Utc;

/// 用户数据访问层（DAL/Repository）
#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// 创建用户
    pub async fn create(&self, req: &CreateUserRequest) -> AppResult<User> {
        let now = Utc::now();
        
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, full_name, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id, username, email, full_name, created_at, updated_at
            "#,
        )
        .bind(&req.username)
        .bind(&req.email)
        .bind(&req.full_name)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;
        
        tracing::info!("创建用户成功: {}", result.id);
        Ok(result)
    }
    
    /// 通过 ID 查找用户
    pub async fn find_by_id(&self, id: i64) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, full_name, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// 通过用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, full_name, created_at, updated_at
            FROM users
            WHERE username = ?
            "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// 获取所有用户
    pub async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, full_name, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(users)
    }
    
    /// 更新用户
    pub async fn update(&self, id: i64, req: &UpdateUserRequest) -> AppResult<User> {
        // 先获取现有用户
        let mut user = self.find_by_id(id).await?;
        
        // 更新字段
        if let Some(ref username) = req.username {
            user.username = username.clone();
        }
        if let Some(ref email) = req.email {
            user.email = email.clone();
        }
        if req.full_name.is_some() {
            user.full_name = req.full_name.clone();
        }
        
        user.updated_at = Utc::now();
        
        // 执行更新
        sqlx::query(
            r#"
            UPDATE users
            SET username = ?, email = ?, full_name = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(user.updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        tracing::info!("更新用户成功: {}", id);
        Ok(user)
    }
    
    /// 删除用户
    pub async fn delete(&self, id: i64) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        if result.rows_affected() == 0 {
            return Err(crate::models::AppError::NotFound(format!(
                "用户 ID {} 不存在",
                id
            )));
        }
        
        tracing::info!("删除用户成功: {}", id);
        Ok(())
    }
    
    /// 统计用户数量
    pub async fn count(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(count.0)
    }
}
