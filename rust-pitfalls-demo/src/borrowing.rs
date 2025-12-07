//! 借用检查器实战示例

use std::cell::RefCell;
use std::collections::HashMap;

/// 缓存系统 - 展示可变借用问题的解决
pub struct Cache {
    data: HashMap<String, String>,
    access_count: HashMap<String, usize>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
            access_count: HashMap::new(),
        }
    }
    
    /// 获取并追踪访问（返回克隆）
    pub fn get_and_track(&mut self, key: &str) -> Option<String> {
        // 先更新计数
        let count = self.access_count.entry(key.to_string()).or_insert(0);
        *count += 1;
        
        // 再获取数据
        self.data.get(key).cloned()
    }
    
    /// 只读获取
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    /// 追踪访问
    pub fn track_access(&mut self, key: &str) {
        *self.access_count.entry(key.to_string()).or_insert(0) += 1;
    }
    
    /// 设置值
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> &HashMap<String, usize> {
        &self.access_count
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据库（内部可变性） - 展示 RefCell 的使用
pub struct Database {
    data: RefCell<Vec<String>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: RefCell::new(Vec::new()),
        }
    }
    
    /// 不可变 self，但可以修改内部数据
    pub fn add(&self, item: String) {
        self.data.borrow_mut().push(item);
    }
    
    pub fn get(&self, index: usize) -> Option<String> {
        self.data.borrow().get(index).cloned()
    }
    
    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

/// 字符串切片工具 - 展示悬垂引用的避免
pub struct StringSlicer {
    content: String,
}

impl StringSlicer {
    pub fn new(content: String) -> Self {
        StringSlicer { content }
    }
    
    /// 返回 content 的切片
    pub fn get_first_word(&self) -> &str {
        self.content
            .split_whitespace()
            .next()
            .unwrap_or("")
    }
    
    /// 返回所有单词的引用
    pub fn get_words(&self) -> Vec<&str> {
        self.content.split_whitespace().collect()
    }
    
    /// 返回大写副本（所有权）
    pub fn get_uppercase(&self) -> String {
        self.content.to_uppercase()
    }
    
    /// 就地修改为大写
    pub fn to_uppercase_inplace(&mut self) {
        self.content = self.content.to_uppercase();
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_borrowing() {
        let mut cache = Cache::new();
        
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        
        // 获取并追踪
        assert_eq!(cache.get_and_track("key1"), Some("value1".to_string()));
        assert_eq!(cache.get_and_track("key1"), Some("value1".to_string()));
        
        // 检查统计
        let stats = cache.get_stats();
        assert_eq!(stats.get("key1"), Some(&2));
    }
    
    #[test]
    fn test_database_interior_mutability() {
        let db = Database::new();
        
        // 不可变引用，但可以修改内部数据
        db.add("record 1".to_string());
        db.add("record 2".to_string());
        
        assert_eq!(db.len(), 2);
        assert_eq!(db.get(0), Some("record 1".to_string()));
    }
    
    #[test]
    fn test_string_slicer() {
        let mut slicer = StringSlicer::new("Hello Rust World".to_string());
        
        assert_eq!(slicer.get_first_word(), "Hello");
        assert_eq!(slicer.get_words(), vec!["Hello", "Rust", "World"]);
        
        let upper = slicer.get_uppercase();
        assert_eq!(upper, "HELLO RUST WORLD");
        
        // 原字符串未改变
        assert_eq!(slicer.content(), "Hello Rust World");
        
        // 就地修改
        slicer.to_uppercase_inplace();
        assert_eq!(slicer.content(), "HELLO RUST WORLD");
    }
}
