// 正则表达式（使用 regex 库）

use regex::Regex;

/// # 基本匹配
pub fn basic_matching_demo() {
    println!("\n=== 基本匹配 ===");
    
    // 创建正则表达式
    let re = Regex::new(r"\d+").unwrap();
    
    // 检查是否匹配
    println!("\"abc123\" 包含数字: {}", re.is_match("abc123"));
    println!("\"abc\" 包含数字: {}", re.is_match("abc"));
    
    // 查找第一个匹配
    let text = "The price is 100 dollars";
    if let Some(mat) = re.find(text) {
        println!("\n找到数字: \"{}\" 在位置 {}", mat.as_str(), mat.start());
    }
    
    // 查找所有匹配
    let text = "Call me at 123-456-7890 or 098-765-4321";
    println!("\n所有数字:");
    for mat in re.find_iter(text) {
        println!("  {}", mat.as_str());
    }
}

/// # 捕获组
pub fn capture_groups_demo() {
    println!("\n=== 捕获组 ===");
    
    // 简单捕获
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "Today is 2024-12-25";
    
    if let Some(caps) = re.captures(text) {
        println!("完整匹配: {}", &caps[0]);
        println!("年: {}", &caps[1]);
        println!("月: {}", &caps[2]);
        println!("日: {}", &caps[3]);
    }
    
    // 命名捕获组
    let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    
    if let Some(caps) = re.captures(text) {
        println!("\n使用命名捕获:");
        println!("年: {}", &caps["year"]);
        println!("月: {}", &caps["month"]);
        println!("日: {}", &caps["day"]);
    }
    
    // 多次捕获
    let text = "Dates: 2024-01-15, 2024-06-30, 2024-12-25";
    println!("\n所有日期:");
    for caps in re.captures_iter(text) {
        println!("  {}-{}-{}", &caps["year"], &caps["month"], &caps["day"]);
    }
}

/// # 替换文本
pub fn replacement_demo() {
    println!("\n=== 替换文本 ===");
    
    let re = Regex::new(r"\d+").unwrap();
    
    // 替换第一个匹配
    let text = "I have 2 apples and 3 oranges";
    let result = re.replace(text, "many");
    println!("替换第一个: {}", result);
    
    // 替换所有匹配
    let result = re.replace_all(text, "X");
    println!("替换所有: {}", result);
    
    // 使用捕获组替换
    let re = Regex::new(r"(\w+)\s+(\w+)").unwrap();
    let text = "hello world";
    let result = re.replace(text, "$2 $1");  // 交换单词
    println!("\n交换单词: {}", result);
    
    // 使用闭包替换
    let re = Regex::new(r"\d+").unwrap();
    let text = "I have 2 apples and 3 oranges";
    let result = re.replace_all(text, |caps: &regex::Captures| {
        let num: i32 = caps[0].parse().unwrap();
        format!("{}", num * 2)
    });
    println!("数字加倍: {}", result);
}

/// # 分割字符串
pub fn splitting_demo() {
    println!("\n=== 分割字符串 ===");
    
    let re = Regex::new(r"\s+").unwrap();
    let text = "one  two\tthree\nfour";
    
    println!("分割空白:");
    for part in re.split(text) {
        println!("  \"{}\"", part);
    }
    
    // 限制分割次数
    let re = Regex::new(r",").unwrap();
    let text = "a,b,c,d,e";
    
    println!("\n限制分割次数 (最多3部分):");
    for part in re.splitn(text, 3) {
        println!("  \"{}\"", part);
    }
}

/// # 常用正则表达式模式
pub fn common_patterns_demo() {
    println!("\n=== 常用正则表达式模式 ===");
    
    // 邮箱验证
    let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    let emails = vec![
        "user@example.com",
        "invalid.email",
        "test+tag@domain.co.uk",
    ];
    
    println!("邮箱验证:");
    for email in emails {
        println!("  {}: {}", email, email_re.is_match(email));
    }
    
    // 手机号验证（中国）
    let phone_re = Regex::new(r"^1[3-9]\d{9}$").unwrap();
    let phones = vec!["13812345678", "19987654321", "12345678901"];
    
    println!("\n手机号验证:");
    for phone in phones {
        println!("  {}: {}", phone, phone_re.is_match(phone));
    }
    
    // URL 验证
    let url_re = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
    let urls = vec![
        "http://example.com",
        "https://example.com/path?query=value",
        "not a url",
    ];
    
    println!("\nURL 验证:");
    for url in urls {
        println!("  {}: {}", url, url_re.is_match(url));
    }
    
    // IP 地址提取
    let ip_re = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    let text = "Servers: 192.168.1.1, 10.0.0.1, 172.16.0.1";
    
    println!("\nIP 地址提取:");
    for mat in ip_re.find_iter(text) {
        println!("  {}", mat.as_str());
    }
}

/// # HTML 标签处理
pub fn html_demo() {
    println!("\n=== HTML 标签处理 ===");
    
    // 移除 HTML 标签
    let html_re = Regex::new(r"<[^>]+>").unwrap();
    let html = "<p>This is <strong>bold</strong> text.</p>";
    let plain_text = html_re.replace_all(html, "");
    
    println!("HTML: {}", html);
    println!("纯文本: {}", plain_text);
    
    // 提取链接
    let link_re = Regex::new(r#"<a\s+href="([^"]+)"[^>]*>([^<]+)</a>"#).unwrap();
    let html = r#"<a href="http://example.com">Example</a> and <a href="/page">Page</a>"#;
    
    println!("\n提取链接:");
    for caps in link_re.captures_iter(html) {
        println!("  文本: {}, URL: {}", &caps[2], &caps[1]);
    }
}

/// # 单词边界和锚点
pub fn boundaries_demo() {
    println!("\n=== 单词边界和锚点 ===");
    
    // 单词边界 \b
    let word_re = Regex::new(r"\bcat\b").unwrap();
    
    println!("单词边界匹配:");
    println!("  \"cat\": {}", word_re.is_match("cat"));
    println!("  \"the cat sat\": {}", word_re.is_match("the cat sat"));
    println!("  \"concatenate\": {}", word_re.is_match("concatenate"));
    
    // 行首 ^ 和行尾 $
    let start_re = Regex::new(r"^Hello").unwrap();
    let end_re = Regex::new(r"world$").unwrap();
    
    println!("\n行首行尾:");
    println!("  \"Hello world\" 以 Hello 开头: {}", start_re.is_match("Hello world"));
    println!("  \"Say Hello\" 以 Hello 开头: {}", start_re.is_match("Say Hello"));
    println!("  \"Hello world\" 以 world 结尾: {}", end_re.is_match("Hello world"));
}

/// # 贪婪与非贪婪匹配
pub fn greedy_demo() {
    println!("\n=== 贪婪与非贪婪匹配 ===");
    
    let text = "<div>content 1</div><div>content 2</div>";
    
    // 贪婪匹配
    let greedy_re = Regex::new(r"<div>.*</div>").unwrap();
    if let Some(mat) = greedy_re.find(text) {
        println!("贪婪匹配: {}", mat.as_str());
    }
    
    // 非贪婪匹配
    let lazy_re = Regex::new(r"<div>.*?</div>").unwrap();
    println!("\n非贪婪匹配:");
    for mat in lazy_re.find_iter(text) {
        println!("  {}", mat.as_str());
    }
}

/// # 实战示例：日志解析
pub fn log_parsing_demo() {
    println!("\n=== 实战示例：日志解析 ===");
    
    let log_re = Regex::new(
        r"(?P<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(?P<level>\w+)\] (?P<message>.*)"
    ).unwrap();
    
    let logs = vec![
        "2024-12-25 10:30:45 [INFO] Application started",
        "2024-12-25 10:31:02 [ERROR] Connection failed",
        "2024-12-25 10:31:15 [WARN] Retrying connection",
    ];
    
    println!("解析日志:");
    for log in logs {
        if let Some(caps) = log_re.captures(log) {
            println!("  时间: {}", &caps["timestamp"]);
            println!("  级别: {}", &caps["level"]);
            println!("  消息: {}", &caps["message"]);
            println!();
        }
    }
}

/// # 实战示例：数据提取
pub fn data_extraction_demo() {
    println!("\n=== 实战示例：数据提取 ===");
    
    // 提取价格
    let price_re = Regex::new(r"\$(\d+(?:\.\d{2})?)").unwrap();
    let text = "Items cost $19.99, $5.00, and $123.45";
    
    println!("提取价格:");
    let mut total = 0.0;
    for caps in price_re.captures_iter(text) {
        let price: f64 = caps[1].parse().unwrap();
        println!("  ${}", price);
        total += price;
    }
    println!("总价: ${:.2}", total);
    
    // 提取标签
    let tag_re = Regex::new(r"#(\w+)").unwrap();
    let text = "Check out #rust #programming #tutorial";
    
    println!("\n提取标签:");
    for caps in tag_re.captures_iter(text) {
        println!("  {}", &caps[1]);
    }
}

/// # 实战示例：输入验证
pub fn validation_demo() {
    println!("\n=== 实战示例：输入验证 ===");
    
    struct Validator {
        username_re: Regex,
        password_re: Regex,
        email_re: Regex,
    }
    
    impl Validator {
        fn new() -> Self {
            Validator {
                username_re: Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap(),
                password_re: Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).{8,}$").unwrap(),
                email_re: Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap(),
            }
        }
        
        fn validate_username(&self, username: &str) -> bool {
            self.username_re.is_match(username)
        }
        
        fn validate_password(&self, password: &str) -> bool {
            self.password_re.is_match(password)
        }
        
        fn validate_email(&self, email: &str) -> bool {
            self.email_re.is_match(email)
        }
    }
    
    let validator = Validator::new();
    
    let test_data = vec![
        ("user123", "Password1", "user@example.com"),
        ("ab", "weak", "invalid"),
    ];
    
    for (username, password, email) in test_data {
        println!("\n测试数据: {} / {} / {}", username, password, email);
        println!("  用户名有效: {}", validator.validate_username(username));
        println!("  密码有效: {}", validator.validate_password(password));
        println!("  邮箱有效: {}", validator.validate_email(email));
    }
}

/// 运行所有正则表达式示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 正则表达式详解           ║");
    println!("╚════════════════════════════════════╝");
    
    basic_matching_demo();
    capture_groups_demo();
    replacement_demo();
    splitting_demo();
    common_patterns_demo();
    html_demo();
    boundaries_demo();
    greedy_demo();
    log_parsing_demo();
    data_extraction_demo();
    validation_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_matching() {
        let re = Regex::new(r"\d+").unwrap();
        assert!(re.is_match("abc123"));
        assert!(!re.is_match("abc"));
    }
    
    #[test]
    fn test_capture_groups() {
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let caps = re.captures("2024-12-25").unwrap();
        assert_eq!(&caps[1], "2024");
        assert_eq!(&caps[2], "12");
        assert_eq!(&caps[3], "25");
    }
    
    #[test]
    fn test_replacement() {
        let re = Regex::new(r"\d+").unwrap();
        let result = re.replace("I have 2 apples", "X");
        assert_eq!(result, "I have X apples");
    }
}
