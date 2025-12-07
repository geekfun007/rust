//! 生命周期实战示例

/// 比较两个字符串，返回较长的一个
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 持有引用的结构体
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        ImportantExcerpt { part: text }
    }
    
    pub fn level(&self) -> i32 {
        3
    }
    
    /// 生命周期省略规则适用
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

/// 字符串分析器（生命周期与所有权结合）
pub struct StringAnalyzer {
    content: String,
}

impl StringAnalyzer {
    pub fn new(content: String) -> Self {
        StringAnalyzer { content }
    }
    
    /// 返回第一行的引用
    pub fn first_line(&self) -> &str {
        self.content.lines().next().unwrap_or("")
    }
    
    /// 返回最后一个单词的引用
    pub fn last_word(&self) -> &str {
        self.content
            .split_whitespace()
            .last()
            .unwrap_or("")
    }
    
    /// 查找关键词并返回包含它的行
    pub fn find_line_containing<'a>(&'a self, keyword: &str) -> Option<&'a str> {
        self.content
            .lines()
            .find(|line| line.contains(keyword))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longest() {
        let s1 = String::from("long string");
        let s2 = "short";
        
        let result = longest(&s1, s2);
        assert_eq!(result, "long string");
    }
    
    #[test]
    fn test_important_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        
        let excerpt = ImportantExcerpt::new(first_sentence);
        assert_eq!(excerpt.level(), 3);
        assert_eq!(excerpt.part, "Call me Ishmael");
    }
    
    #[test]
    fn test_string_analyzer() {
        let analyzer = StringAnalyzer::new(
            "First line\nSecond line with keyword\nThird line".to_string()
        );
        
        assert_eq!(analyzer.first_line(), "First line");
        assert_eq!(analyzer.last_word(), "line");
        assert_eq!(
            analyzer.find_line_containing("keyword"),
            Some("Second line with keyword")
        );
    }
}
