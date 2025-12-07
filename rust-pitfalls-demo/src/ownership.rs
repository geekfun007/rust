//! 所有权系统实战示例

use std::collections::HashMap;

/// 配置管理器 - 展示所有权转移的正确用法
pub struct Config {
    settings: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            settings: HashMap::new(),
        }
    }
    
    /// 借用方式获取设置
    pub fn get_setting(&self, key: &str) -> Option<String> {
        self.settings.get(key).cloned()
    }
    
    /// 可变借用设置值
    pub fn set_setting(&mut self, key: String, value: String) {
        self.settings.insert(key, value);
    }
    
    /// 消费 self，返回所有设置
    pub fn into_settings(self) -> HashMap<String, String> {
        self.settings
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户数据 - 展示部分移动的处理
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub age: u32,
}

impl User {
    pub fn new(id: u32, username: String, email: String, age: u32) -> Self {
        User { id, username, email, age }
    }
    
    /// 返回字段的引用
    pub fn get_username(&self) -> &str {
        &self.username
    }
    
    /// 返回字段的克隆
    pub fn take_username(&self) -> String {
        self.username.clone()
    }
    
    /// 消费对象，返回字段
    pub fn into_username(self) -> String {
        self.username
    }
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: String,
    pub message: String,
}

impl LogEntry {
    pub fn new(timestamp: u64, level: &str, message: &str) -> Self {
        LogEntry {
            timestamp,
            level: level.to_string(),
            message: message.to_string(),
        }
    }
}

/// 日志处理器 - 展示循环中的所有权
pub struct LogProcessor {
    logs: Vec<LogEntry>,
}

impl LogProcessor {
    pub fn new() -> Self {
        LogProcessor { logs: Vec::new() }
    }
    
    pub fn add_log(&mut self, entry: LogEntry) {
        self.logs.push(entry);
    }
    
    /// 借用迭代：只读访问
    pub fn print_all(&self) {
        for log in &self.logs {
            println!("[{}] {}: {}", log.timestamp, log.level, log.message);
        }
    }
    
    /// 可变借用迭代：修改日志
    pub fn censor_sensitive_data(&mut self, keyword: &str) {
        for log in &mut self.logs {
            log.message = log.message.replace(keyword, "***");
        }
    }
    
    /// 借用迭代：过滤并收集
    pub fn filter_by_level(&self, level: &str) -> Vec<LogEntry> {
        self.logs.iter()
            .filter(|log| log.level == level)
            .cloned()
            .collect()
    }
    
    /// 消费迭代：转移所有权
    pub fn into_errors(self) -> Vec<LogEntry> {
        self.logs.into_iter()
            .filter(|log| log.level == "ERROR")
            .collect()
    }
    
    pub fn len(&self) -> usize {
        self.logs.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.logs.is_empty()
    }
}

impl Default for LogProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_ownership() {
        let mut config = Config::new();
        config.set_setting("host".to_string(), "localhost".to_string());
        
        // 可以多次借用
        assert_eq!(config.get_setting("host"), Some("localhost".to_string()));
        assert_eq!(config.get_setting("host"), Some("localhost".to_string()));
        
        // 消费配置
        let settings = config.into_settings();
        assert_eq!(settings.get("host"), Some(&"localhost".to_string()));
    }
    
    #[test]
    fn test_user_partial_move() {
        let user = User::new(
            1,
            "alice".to_string(),
            "alice@example.com".to_string(),
            25
        );
        
        // 借用字段
        assert_eq!(user.get_username(), "alice");
        
        // 克隆字段
        let username = user.take_username();
        assert_eq!(username, "alice");
        
        // user 仍然可用
        assert_eq!(user.id, 1);
    }
    
    #[test]
    fn test_log_processor_iteration() {
        let mut processor = LogProcessor::new();
        processor.add_log(LogEntry::new(1000, "INFO", "启动"));
        processor.add_log(LogEntry::new(1001, "ERROR", "错误"));
        processor.add_log(LogEntry::new(1002, "WARN", "警告"));
        
        assert_eq!(processor.len(), 3);
        
        // 借用迭代
        let errors = processor.filter_by_level("ERROR");
        assert_eq!(errors.len(), 1);
        
        // processor 仍可用
        assert_eq!(processor.len(), 3);
    }
}
