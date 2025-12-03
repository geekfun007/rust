use crate::db::UserRepository;
use crate::models::{AppError, AppResult, CreateUserRequest, UpdateUserRequest, User, Validate};

/// 用户服务层（业务逻辑）
#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
    
    /// 创建用户（带验证）
    pub async fn create_user(&self, req: CreateUserRequest) -> AppResult<User> {
        // 验证输入
        req.validate()
            .map_err(|e| AppError::ValidationError(e))?;
        
        // 检查用户名是否已存在
        if let Ok(_) = self.repository.find_by_username(&req.username).await {
            return Err(AppError::ValidationError(
                "用户名已存在".to_string(),
            ));
        }
        
        // 创建用户
        self.repository.create(&req).await
    }
    
    /// 获取用户
    pub async fn get_user(&self, id: i64) -> AppResult<User> {
        self.repository.find_by_id(id).await
    }
    
    /// 获取用户列表
    pub async fn list_users(&self, page: i64, page_size: i64) -> AppResult<Vec<User>> {
        let offset = (page - 1) * page_size;
        self.repository.find_all(page_size, offset).await
    }
    
    /// 更新用户
    pub async fn update_user(&self, id: i64, req: UpdateUserRequest) -> AppResult<User> {
        // 验证输入
        req.validate()
            .map_err(|e| AppError::ValidationError(e))?;
        
        // 如果更新用户名，检查是否已存在
        if let Some(ref username) = req.username {
            if let Ok(existing_user) = self.repository.find_by_username(username).await {
                if existing_user.id != id {
                    return Err(AppError::ValidationError(
                        "用户名已存在".to_string(),
                    ));
                }
            }
        }
        
        self.repository.update(id, &req).await
    }
    
    /// 删除用户
    pub async fn delete_user(&self, id: i64) -> AppResult<()> {
        self.repository.delete(id).await
    }
    
    /// 获取用户总数
    pub async fn count_users(&self) -> AppResult<i64> {
        self.repository.count().await
    }
}
