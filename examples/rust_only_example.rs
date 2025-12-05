// Rust 纯库使用示例
use rust_py_example::{math, text, User};

fn main() {
    println!("=== Rust 库使用示例 ===\n");

    // 用户示例
    println!("1. 用户管理:");
    let user1 = User::new("张三".to_string(), 25, "zhangsan@example.com".to_string());
    let user2 = User::new("李四".to_string(), 16, "lisi@example.com".to_string());

    println!("  {}", user1.description());
    println!("  是否成年: {}", user1.is_adult());
    println!("  {}", user2.description());
    println!("  是否成年: {}\n", user2.is_adult());

    // 数学计算示例
    println!("2. 数学计算:");
    println!("  斐波那契数列 (n=10): {}", math::fibonacci(10));
    println!("  阶乘 (5!): {}", math::factorial(5));
    println!("  17 是质数吗? {}", math::is_prime(17));
    println!("  18 是质数吗? {}\n", math::is_prime(18));

    // 字符串处理示例
    println!("3. 字符串处理:");
    let text = "hello world from rust";
    println!("  原始文本: {}", text);
    println!("  反转: {}", text::reverse(text));
    println!("  单词数: {}", text::word_count(text));
    println!("  首字母大写: {}", text::capitalize_words(text));
}
