use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建用户请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(max = 100))]
    pub full_name: Option<String>,
}

/// 更新用户请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(length(max = 100))]
    pub full_name: Option<String>,
}

// 简单的验证 trait（手动实现）
pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

impl Validate for CreateUserRequest {
    fn validate(&self) -> Result<(), String> {
        if self.username.len() < 3 || self.username.len() > 50 {
            return Err("用户名长度必须在 3-50 之间".to_string());
        }
        
        if !self.email.contains('@') {
            return Err("邮箱格式不正确".to_string());
        }
        
        if let Some(ref name) = self.full_name {
            if name.len() > 100 {
                return Err("全名长度不能超过 100".to_string());
            }
        }
        
        Ok(())
    }
}

impl Validate for UpdateUserRequest {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref username) = self.username {
            if username.len() < 3 || username.len() > 50 {
                return Err("用户名长度必须在 3-50 之间".to_string());
            }
        }
        
        if let Some(ref email) = self.email {
            if !email.contains('@') {
                return Err("邮箱格式不正确".to_string());
            }
        }
        
        if let Some(ref name) = self.full_name {
            if name.len() > 100 {
                return Err("全名长度不能超过 100".to_string());
            }
        }
        
        Ok(())
    }
}
