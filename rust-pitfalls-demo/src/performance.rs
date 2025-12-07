//! 性能优化实战示例

use std::borrow::Cow;

/// 字符串处理器
pub struct StringProcessor;

impl StringProcessor {
    /// 低效：不必要的克隆
    #[allow(dead_code)]
    fn process_strings_bad(strings: Vec<String>) -> Vec<String> {
        strings.iter().map(|s| s.clone()).collect()
    }
    
    /// 高效：使用引用
    pub fn process_strings_ref(strings: &[String]) -> Vec<&str> {
        strings.iter().map(|s| s.as_str()).collect()
    }
    
    /// 高效：转移所有权
    pub fn process_strings_move(mut strings: Vec<String>) -> Vec<String> {
        strings.retain(|s| !s.is_empty());
        strings
    }
    
    /// 可能修改可能不修改：使用 Cow
    pub fn maybe_modify<'a>(s: &'a str, should_modify: bool) -> Cow<'a, str> {
        if should_modify {
            Cow::Owned(format!("{} modified", s))
        } else {
            Cow::Borrowed(s)
        }
    }
}

/// 向量构建器
pub struct VectorBuilder;

impl VectorBuilder {
    /// 低效：多次重新分配
    #[allow(dead_code)]
    fn build_vector_bad(size: usize) -> Vec<i32> {
        let mut v = Vec::new();
        for i in 0..size {
            v.push(i as i32);
        }
        v
    }
    
    /// 高效：预分配容量
    pub fn build_vector_optimized(size: usize) -> Vec<i32> {
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            v.push(i as i32);
        }
        v
    }
    
    /// 更好：使用迭代器
    pub fn build_vector_iter(size: usize) -> Vec<i32> {
        (0..size as i32).collect()
    }
}

/// 数组求和
pub struct ArraySummer;

impl ArraySummer {
    /// 低效：索引访问
    #[allow(dead_code)]
    fn sum_vector_bad(v: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..v.len() {
            sum += v[i];
        }
        sum
    }
    
    /// 高效：使用迭代器
    pub fn sum_vector_iter(v: &[i32]) -> i32 {
        v.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_processing() {
        let strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        
        let refs = StringProcessor::process_strings_ref(&strings);
        assert_eq!(refs, vec!["a", "b", "c"]);
        
        let moved = StringProcessor::process_strings_move(strings);
        assert_eq!(moved.len(), 3);
    }
    
    #[test]
    fn test_cow() {
        let s = "hello";
        
        let unmodified = StringProcessor::maybe_modify(s, false);
        assert_eq!(unmodified, "hello");
        
        let modified = StringProcessor::maybe_modify(s, true);
        assert_eq!(modified, "hello modified");
    }
    
    #[test]
    fn test_vector_building() {
        let v1 = VectorBuilder::build_vector_optimized(100);
        let v2 = VectorBuilder::build_vector_iter(100);
        
        assert_eq!(v1.len(), 100);
        assert_eq!(v2.len(), 100);
        assert_eq!(v1, v2);
    }
    
    #[test]
    fn test_array_sum() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(ArraySummer::sum_vector_iter(&v), 15);
    }
}
