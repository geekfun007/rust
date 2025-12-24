// 用户模型

pub struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }
    
    // Getter 方法
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    // 验证方法
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.email.contains('@')
    }
}
