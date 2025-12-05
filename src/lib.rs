// Rust 核心库代码
use serde::{Deserialize, Serialize};

/// 用户信息结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
}

impl User {
    /// 创建新用户
    pub fn new(name: String, age: u32, email: String) -> Self {
        User { name, age, email }
    }

    /// 获取用户描述
    pub fn description(&self) -> String {
        format!(
            "{} is {} years old, email: {}",
            self.name, self.age, self.email
        )
    }

    /// 检查是否成年
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

/// 数学计算模块
pub mod math {
    /// 计算斐波那契数列
    pub fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0u64;
                let mut b = 1u64;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    /// 判断是否为质数
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let sqrt_n = (n as f64).sqrt() as u64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    /// 计算阶乘
    pub fn factorial(n: u32) -> u64 {
        if n == 0 || n == 1 {
            1
        } else {
            (2..=n as u64).product()
        }
    }
}

/// 字符串处理模块
pub mod text {
    /// 反转字符串
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// 统计单词数量
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }

    /// 首字母大写
    pub fn capitalize_words(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// PyO3 Python 绑定（仅在启用 python 功能时编译）
#[cfg(feature = "python")]
pub mod python_bindings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User::new("Alice".to_string(), 25, "alice@example.com".to_string());
        assert_eq!(user.name, "Alice");
        assert!(user.is_adult());
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(math::fibonacci(0), 0);
        assert_eq!(math::fibonacci(1), 1);
        assert_eq!(math::fibonacci(10), 55);
    }

    #[test]
    fn test_is_prime() {
        assert!(!math::is_prime(0));
        assert!(!math::is_prime(1));
        assert!(math::is_prime(2));
        assert!(math::is_prime(17));
        assert!(!math::is_prime(18));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(text::reverse("hello"), "olleh");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(text::word_count("hello world"), 2);
    }
}
