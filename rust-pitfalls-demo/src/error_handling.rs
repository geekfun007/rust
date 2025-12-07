//! 错误处理实战示例

use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

/// 应用错误类型
#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    ParseError(ParseIntError),
    NotFound(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO 错误: {}", e),
            AppError::ParseError(e) => write!(f, "解析错误: {}", e),
            AppError::NotFound(msg) => write!(f, "未找到: {}", msg),
            AppError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

pub type AppResult<T> = Result<T, AppError>;

/// 文件处理器
pub struct FileProcessor;

impl FileProcessor {
    /// 读取并解析数字文件
    pub fn read_number_from_file(path: &str) -> AppResult<i32> {
        let content = fs::read_to_string(path)?;
        let number: i32 = content.trim().parse()?;
        Ok(number)
    }
    
    /// 验证并处理用户输入
    pub fn validate_age(age: i32) -> AppResult<i32> {
        if age < 0 {
            return Err(AppError::ValidationError("年龄不能为负数".to_string()));
        }
        if age > 150 {
            return Err(AppError::ValidationError("年龄不能超过 150".to_string()));
        }
        Ok(age)
    }
    
    /// 查找用户
    pub fn find_user(id: u32, users: &[(u32, String)]) -> AppResult<String> {
        users
            .iter()
            .find(|(uid, _)| *uid == id)
            .map(|(_, name)| name.clone())
            .ok_or_else(|| AppError::NotFound(format!("用户 {} 未找到", id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_age() {
        assert!(FileProcessor::validate_age(25).is_ok());
        assert!(FileProcessor::validate_age(-1).is_err());
        assert!(FileProcessor::validate_age(200).is_err());
    }
    
    #[test]
    fn test_find_user() {
        let users = vec![
            (1, "Alice".to_string()),
            (2, "Bob".to_string()),
        ];
        
        assert_eq!(
            FileProcessor::find_user(1, &users).unwrap(),
            "Alice"
        );
        assert!(FileProcessor::find_user(99, &users).is_err());
    }
}
