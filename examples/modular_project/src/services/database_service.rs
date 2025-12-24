// 数据库服务
// 处理数据库连接和操作

pub struct Database {
    connection_string: String,
}

impl Database {
    pub fn new(connection_string: String) -> Self {
        Database { connection_string }
    }
    
    pub fn is_connected(&self) -> bool {
        !self.connection_string.is_empty()
    }
}

pub fn connect(connection_string: &str) -> Result<Database, String> {
    if connection_string.is_empty() {
        Err("Invalid connection string".to_string())
    } else {
        Ok(Database::new(connection_string.to_string()))
    }
}
