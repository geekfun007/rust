// Rust String & &str 使用场景 & 实战
//
// 本教程全面讲解 String 和 &str 的区别、使用场景和最佳实践

use std::collections::HashMap;

fn main() {
    println!("=== Rust String & &str 完全指南 ===\n");
    
    // 1. 基础概念
    demo_basic_concepts();
    
    // 2. String 详解
    demo_string_type();
    
    // 3. &str 详解
    demo_str_type();
    
    // 4. 相互转换
    demo_conversions();
    
    // 5. 使用场景对比
    demo_use_cases();
    
    // 6. 性能考虑
    demo_performance();
    
    // 7. 函数参数设计
    demo_function_parameters();
    
    // 8. 字符串操作
    demo_string_operations();
    
    // 9. 常见陷阱
    demo_common_pitfalls();
    
    // 10. 实战案例
    demo_real_world_examples();
    
    println!("\n✅ 所有示例运行成功！");
}

// ============================================
// 1. 基础概念
// ============================================
fn demo_basic_concepts() {
    println!("--- 1. 基础概念 ---\n");
    
    println!("String vs &str:");
    println!("┌──────────────┬────────────────┬────────────────┐");
    println!("│  特性         │  String        │  &str          │");
    println!("├──────────────┼────────────────┼────────────────┤");
    println!("│  所有权       │  拥有          │  借用          │");
    println!("│  可变性       │  可变          │  不可变        │");
    println!("│  内存         │  堆分配        │  静态/栈/堆    │");
    println!("│  大小         │  动态          │  已知          │");
    println!("│  修改         │  可以          │  不可以        │");
    println!("│  生命周期     │  owned         │  borrowed      │");
    println!("└──────────────┴────────────────┴────────────────┘");
    println!();
    
    println!("内存布局:");
    println!("  String:");
    println!("    栈: [指针 | 长度 | 容量]");
    println!("    堆: [实际字符串数据...]");
    println!();
    println!("  &str:");
    println!("    栈: [指针 | 长度]");
    println!("    数据: 指向某处的字符串切片");
    println!();
    
    // 示例
    println!("示例:");
    
    // String - 堆分配，拥有所有权
    let string = String::from("Hello, World!");
    println!("  String: \"{}\"", string);
    println!("    容量: {}", string.capacity());
    println!("    长度: {}", string.len());
    println!();
    
    // &str - 字符串切片，借用
    let str_slice: &str = "Hello, Rust!";
    println!("  &str: \"{}\"", str_slice);
    println!("    长度: {}", str_slice.len());
    println!();
}

// ============================================
// 2. String 详解
// ============================================
fn demo_string_type() {
    println!("--- 2. String 详解 ---\n");
    
    println!("什么是 String？");
    println!("  - 可增长的、可变的、拥有所有权的 UTF-8 编码字符串");
    println!("  - 存储在堆上");
    println!("  - 类似于 Vec<u8>，但保证有效 UTF-8");
    println!();
    
    // 创建 String
    println!("创建 String:");
    
    // 方法 1: String::new()
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("  String::new(): \"{}\"", s1);
    
    // 方法 2: String::from()
    let s2 = String::from("World");
    println!("  String::from(): \"{}\"", s2);
    
    // 方法 3: to_string()
    let s3 = "Rust".to_string();
    println!("  to_string(): \"{}\"", s3);
    
    // 方法 4: to_owned()
    let s4 = "Programming".to_owned();
    println!("  to_owned(): \"{}\"", s4);
    
    // 方法 5: format!
    let s5 = format!("Hello, {}!", "World");
    println!("  format!(): \"{}\"", s5);
    println!();
    
    // String 操作
    println!("String 操作:");
    
    let mut s = String::from("Hello");
    
    // 追加
    s.push_str(", World");
    println!("  push_str: \"{}\"", s);
    
    s.push('!');
    println!("  push: \"{}\"", s);
    
    // 插入
    s.insert(5, ' ');
    println!("  insert: \"{}\"", s);
    
    s.insert_str(6, "Rust ");
    println!("  insert_str: \"{}\"", s);
    
    // 替换
    let s2 = s.replace("World", "Everyone");
    println!("  replace: \"{}\"", s2);
    
    // 清空
    let mut s3 = String::from("Clear me");
    s3.clear();
    println!("  clear: \"{}\" (空)", s3);
    println!();
    
    // 容量管理
    println!("容量管理:");
    
    let mut s = String::new();
    println!("  初始容量: {}", s.capacity());
    
    s.push_str("Hello");
    println!("  添加后容量: {}", s.capacity());
    
    s.reserve(100);
    println!("  预留后容量: {}", s.capacity());
    
    s.shrink_to_fit();
    println!("  缩减后容量: {}", s.capacity());
    println!();
}

// ============================================
// 3. &str 详解
// ============================================
fn demo_str_type() {
    println!("--- 3. &str 详解 ---\n");
    
    println!("什么是 &str？");
    println!("  - 字符串切片（slice）");
    println!("  - 不可变引用");
    println!("  - 可以指向静态字符串、String 或其他 &str");
    println!("  - 固定大小（编译时已知）");
    println!();
    
    // &str 的来源
    println!("&str 的来源:");
    
    // 1. 字符串字面量
    let s1: &str = "字符串字面量";
    println!("  字面量: \"{}\"", s1);
    
    // 2. String 切片
    let string = String::from("Hello, World!");
    let s2: &str = &string;
    println!("  String 引用: \"{}\"", s2);
    
    // 3. String 部分切片
    let s3: &str = &string[0..5];
    println!("  String 切片: \"{}\"", s3);
    
    // 4. 切片的切片
    let s4: &str = &s2[7..12];
    println!("  切片的切片: \"{}\"", s4);
    println!();
    
    // &str 操作（只读）
    println!("&str 操作:");
    
    let s = "Hello, Rust!";
    
    // 长度
    println!("  len: {}", s.len());
    
    // 是否为空
    println!("  is_empty: {}", s.is_empty());
    
    // 包含
    println!("  contains 'Rust': {}", s.contains("Rust"));
    
    // 开始/结束
    println!("  starts_with 'Hello': {}", s.starts_with("Hello"));
    println!("  ends_with '!': {}", s.ends_with("!"));
    
    // 分割
    println!("  split:");
    for part in s.split(", ") {
        println!("    - \"{}\"", part);
    }
    
    // 转换
    println!("  to_uppercase: \"{}\"", s.to_uppercase());
    println!("  to_lowercase: \"{}\"", s.to_lowercase());
    println!();
}

// ============================================
// 4. 相互转换
// ============================================
fn demo_conversions() {
    println!("--- 4. 相互转换 ---\n");
    
    // &str → String
    println!("&str → String:");
    
    let str_slice = "hello";
    
    let s1 = str_slice.to_string();
    println!("  to_string(): {:?}", s1);
    
    let s2 = str_slice.to_owned();
    println!("  to_owned(): {:?}", s2);
    
    let s3 = String::from(str_slice);
    println!("  String::from(): {:?}", s3);
    
    let s4: String = str_slice.into();
    println!("  into(): {:?}", s4);
    println!();
    
    // String → &str
    println!("String → &str:");
    
    let string = String::from("world");
    
    let s1: &str = &string;
    println!("  &string: {:?}", s1);
    
    let s2: &str = string.as_str();
    println!("  as_str(): {:?}", s2);
    
    let s3: &str = &string[..];
    println!("  &string[..]: {:?}", s3);
    println!();
    
    // 选择建议
    println!("转换选择:");
    println!("  &str → String:");
    println!("    ✓ to_string()   - 最常用");
    println!("    ✓ to_owned()    - 语义清晰");
    println!("    ✓ String::from()- 显式");
    println!();
    println!("  String → &str:");
    println!("    ✓ &string       - 最简单");
    println!("    ✓ as_str()      - 显式");
    println!();
}

// ============================================
// 5. 使用场景对比
// ============================================
fn demo_use_cases() {
    println!("--- 5. 使用场景对比 ---\n");
    
    println!("使用 String 的场景:");
    println!("  ✓ 需要拥有字符串所有权");
    println!("  ✓ 需要修改字符串内容");
    println!("  ✓ 动态构建字符串");
    println!("  ✓ 从函数返回字符串");
    println!("  ✓ 存储在结构体中");
    println!();
    
    // String 场景示例
    println!("String 示例:");
    
    // 1. 动态构建
    fn build_greeting(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    let greeting = build_greeting("Alice");
    println!("  动态构建: \"{}\"", greeting);
    
    // 2. 修改
    let mut message = String::from("Hello");
    message.push_str(", World!");
    println!("  修改: \"{}\"", message);
    
    // 3. 结构体存储
    #[derive(Debug)]
    struct User {
        name: String,
        email: String,
    }
    
    let user = User {
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };
    println!("  结构体: {:?}", user);
    println!();
    
    println!("使用 &str 的场景:");
    println!("  ✓ 只读访问字符串");
    println!("  ✓ 函数参数（推荐）");
    println!("  ✓ 字符串字面量");
    println!("  ✓ 不需要所有权");
    println!("  ✓ 临时引用");
    println!();
    
    // &str 场景示例
    println!("&str 示例:");
    
    // 1. 只读函数
    fn print_message(msg: &str) {
        println!("  只读函数: \"{}\"", msg);
    }
    print_message("Hello");
    
    // 2. 字符串切片
    let text = "Hello, World!";
    let hello = &text[0..5];
    println!("  切片: \"{}\"", hello);
    
    // 3. 模式匹配
    fn classify(s: &str) -> &str {
        match s {
            "hello" => "greeting",
            "bye" => "farewell",
            _ => "unknown",
        }
    }
    println!("  匹配: \"{}\"", classify("hello"));
    println!();
}

// ============================================
// 6. 性能考虑
// ============================================
fn demo_performance() {
    println!("--- 6. 性能考虑 ---\n");
    
    println!("内存分配对比:");
    println!("  &str:");
    println!("    - 零成本（指针 + 长度）");
    println!("    - 不涉及堆分配");
    println!("    - 栈上 16 字节（64位系统）");
    println!();
    println!("  String:");
    println!("    - 堆分配（指针 + 长度 + 容量）");
    println!("    - 栈上 24 字节（64位系统）");
    println!("    - 堆上存储实际数据");
    println!();
    
    // 克隆成本
    println!("克隆成本:");
    
    let str_slice = "Hello, World!";
    let string = String::from("Hello, World!");
    
    // &str 克隆 - 只复制指针和长度
    let _cloned_str = str_slice;
    println!("  &str 克隆: 零成本（Copy）");
    
    // String 克隆 - 完整堆分配
    let _cloned_string = string.clone();
    println!("  String 克隆: 分配新内存");
    println!();
    
    // 性能建议
    println!("性能建议:");
    println!("  ✓ 函数参数优先使用 &str");
    println!("  ✓ 避免不必要的 to_string()");
    println!("  ✓ 使用 String 时预分配容量");
    println!("  ✓ 考虑使用 &'static str");
    println!();
    
    // 预分配示例
    println!("预分配示例:");
    
    // 差 - 多次重新分配
    let mut s1 = String::new();
    for i in 0..1000 {
        s1.push_str(&i.to_string());
    }
    println!("  无预分配: 容量 {}", s1.capacity());
    
    // 好 - 一次分配
    let mut s2 = String::with_capacity(10000);
    for i in 0..1000 {
        s2.push_str(&i.to_string());
    }
    println!("  有预分配: 容量 {}", s2.capacity());
    println!();
}

// ============================================
// 7. 函数参数设计
// ============================================
fn demo_function_parameters() {
    println!("--- 7. 函数参数设计 ---\n");
    
    println!("参数类型选择:");
    println!("┌─────────────┬────────────┬────────────────┐");
    println!("│  场景        │  推荐类型   │  原因          │");
    println!("├─────────────┼────────────┼────────────────┤");
    println!("│  只读        │  &str      │  最灵活        │");
    println!("│  需要所有权  │  String    │  拥有数据      │");
    println!("│  可能修改    │  &mut String│ 可变引用      │");
    println!("└─────────────┴────────────┴────────────────┘");
    println!();
    
    // 示例 1: 只读参数
    println!("示例 1: 只读参数");
    
    fn greet(name: &str) {
        println!("  Hello, {}!", name);
    }
    
    // 可以传入 &str
    greet("Alice");
    
    // 可以传入 String
    let name = String::from("Bob");
    greet(&name);
    
    // 可以传入 String 切片
    let full_name = String::from("Charlie Brown");
    greet(&full_name[0..7]);
    println!();
    
    // 示例 2: 需要所有权
    println!("示例 2: 需要所有权");
    
    fn take_ownership(s: String) {
        println!("  拥有: {}", s);
        // s 在这里被 drop
    }
    
    let owned = String::from("Owned String");
    take_ownership(owned);
    // println!("{}", owned); // 错误！已移动
    println!();
    
    // 示例 3: 修改参数
    println!("示例 3: 修改参数");
    
    fn append_suffix(s: &mut String, suffix: &str) {
        s.push_str(suffix);
    }
    
    let mut text = String::from("Hello");
    append_suffix(&mut text, ", World!");
    println!("  修改后: \"{}\"", text);
    println!();
    
    // 示例 4: 返回 String
    println!("示例 4: 返回 String");
    
    fn create_message(name: &str, age: u32) -> String {
        format!("{} is {} years old", name, age)
    }
    
    let msg = create_message("David", 30);
    println!("  返回: \"{}\"", msg);
    println!();
    
    // 最佳实践
    println!("最佳实践:");
    
    // ✅ 好 - 灵活的参数
    fn process_good(text: &str) -> String {
        text.to_uppercase()
    }
    
    let result1 = process_good("hello");
    let result2 = process_good(&String::from("world"));
    println!("  灵活参数: \"{}\", \"{}\"", result1, result2);
    
    // ❌ 差 - 不灵活的参数
    fn process_bad(text: String) -> String {
        text.to_uppercase()
    }
    
    // 必须传入 String
    let result3 = process_bad("rust".to_string());
    println!("  不灵活参数: \"{}\"", result3);
    println!();
}

// ============================================
// 8. 字符串操作
// ============================================
fn demo_string_operations() {
    println!("--- 8. 字符串操作 ---\n");
    
    // 拼接
    println!("拼接:");
    
    // 方法 1: + 运算符
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2;  // s1 被移动
    println!("  + 运算符: \"{}\"", s3);
    
    // 方法 2: format!
    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = format!("{} {}", s4, s5);  // 不移动
    println!("  format!: \"{}\"", s6);
    
    // 方法 3: push_str
    let mut s7 = String::from("Hello");
    s7.push_str(" World");
    println!("  push_str: \"{}\"", s7);
    println!();
    
    // 切片
    println!("切片:");
    
    let text = "Hello, World!";
    println!("  全部: \"{}\"", &text[..]);
    println!("  前5个: \"{}\"", &text[..5]);
    println!("  后6个: \"{}\"", &text[7..]);
    println!("  中间: \"{}\"", &text[7..12]);
    println!();
    
    // 迭代
    println!("迭代:");
    
    let s = "Hello";
    
    print!("  chars: ");
    for c in s.chars() {
        print!("'{}' ", c);
    }
    println!();
    
    print!("  bytes: ");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!("\n");
    
    // 搜索
    println!("搜索:");
    
    let text = "Hello, Rust Programming!";
    
    println!("  find 'Rust': {:?}", text.find("Rust"));
    println!("  rfind 'o': {:?}", text.rfind('o'));
    println!("  contains 'Rust': {}", text.contains("Rust"));
    println!("  starts_with 'Hello': {}", text.starts_with("Hello"));
    println!("  ends_with '!': {}", text.ends_with("!"));
    println!();
    
    // 分割和连接
    println!("分割和连接:");
    
    let text = "apple,banana,cherry";
    let fruits: Vec<&str> = text.split(',').collect();
    println!("  split: {:?}", fruits);
    
    let joined = fruits.join(" | ");
    println!("  join: \"{}\"", joined);
    println!();
}

// ============================================
// 9. 常见陷阱
// ============================================
fn demo_common_pitfalls() {
    println!("--- 9. 常见陷阱 ---\n");
    
    // 陷阱 1: 索引访问
    println!("陷阱 1: 不能直接索引");
    
    let s = "Hello";
    // let ch = s[0];  // 错误！
    
    println!("  正确方式:");
    let ch = s.chars().nth(0).unwrap();
    println!("    chars().nth(0): '{}'", ch);
    
    let slice = &s[0..1];
    println!("    切片 [0..1]: \"{}\"", slice);
    println!();
    
    // 陷阱 2: UTF-8 边界
    println!("陷阱 2: UTF-8 边界");
    
    let s = "你好";
    // let slice = &s[0..1];  // panic！不是字符边界
    
    let slice = &s[0..3];  // 一个中文字符是 3 字节
    println!("  正确切片: \"{}\"", slice);
    println!();
    
    // 陷阱 3: 所有权转移
    println!("陷阱 3: 所有权转移");
    
    let s1 = String::from("hello");
    let s2 = s1;  // s1 移动到 s2
    // println!("{}", s1);  // 错误！
    println!("  s2: \"{}\"", s2);
    println!();
    
    // 陷阱 4: 不必要的克隆
    println!("陷阱 4: 不必要的克隆");
    
    fn process_bad(s: String) {
        println!("  处理: {}", s);
    }
    
    fn process_good(s: &str) {
        println!("  处理: {}", s);
    }
    
    let text = String::from("test");
    
    // 差 - 不必要的克隆
    process_bad(text.clone());
    
    // 好 - 借用
    process_good(&text);
    println!();
}

// ============================================
// 10. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 10. 实战案例 ---\n");
    
    // 案例 1: 配置管理
    println!("案例 1: 配置管理\n");
    config_manager_example();
    
    // 案例 2: 日志系统
    println!("\n案例 2: 日志系统\n");
    logger_example();
    
    // 案例 3: 文本处理
    println!("\n案例 3: 文本处理\n");
    text_processor_example();
    
    // 案例 4: URL 构建器
    println!("\n案例 4: URL 构建器\n");
    url_builder_example();
}

// 案例 1: 配置管理
fn config_manager_example() {
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        api_key: String,
    }
    
    impl Config {
        // 使用 &str 作为参数（灵活）
        fn new(host: &str, port: u16, api_key: &str) -> Self {
            Config {
                host: host.to_string(),
                port,
                api_key: api_key.to_string(),
            }
        }
        
        // 返回 &str（高效）
        fn get_host(&self) -> &str {
            &self.host
        }
        
        // 返回 String（拥有所有权）
        fn get_url(&self) -> String {
            format!("http://{}:{}", self.host, self.port)
        }
        
        // 可变方法
        fn set_host(&mut self, host: &str) {
            self.host = host.to_string();
        }
    }
    
    let mut config = Config::new("localhost", 8080, "secret123");
    println!("  配置: {:?}", config);
    println!("  URL: {}", config.get_url());
    
    config.set_host("127.0.0.1");
    println!("  更新后: {}", config.get_host());
}

// 案例 2: 日志系统
fn logger_example() {
    struct Logger {
        prefix: String,
        messages: Vec<String>,
    }
    
    impl Logger {
        fn new(prefix: &str) -> Self {
            Logger {
                prefix: prefix.to_string(),
                messages: Vec::new(),
            }
        }
        
        fn log(&mut self, level: &str, message: &str) {
            let log_entry = format!("[{}] {}: {}", self.prefix, level, message);
            self.messages.push(log_entry);
        }
        
        fn get_logs(&self) -> Vec<&str> {
            self.messages.iter().map(|s| s.as_str()).collect()
        }
        
        fn dump(&self) -> String {
            self.messages.join("\n")
        }
    }
    
    let mut logger = Logger::new("APP");
    logger.log("INFO", "Application started");
    logger.log("WARN", "Low memory");
    logger.log("ERROR", "Connection failed");
    
    println!("  日志条数: {}", logger.get_logs().len());
    println!("  日志内容:\n{}", logger.dump());
}

// 案例 3: 文本处理
fn text_processor_example() {
    struct TextProcessor;
    
    impl TextProcessor {
        // 清理文本
        fn clean(text: &str) -> String {
            text.lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n")
        }
        
        // 统计单词
        fn word_count(text: &str) -> HashMap<&str, usize> {
            let mut counts = HashMap::new();
            
            for word in text.split_whitespace() {
                *counts.entry(word).or_insert(0) += 1;
            }
            
            counts
        }
        
        // 高亮关键词
        fn highlight(text: &str, keyword: &str) -> String {
            text.replace(keyword, &format!("**{}**", keyword))
        }
    }
    
    let text = "  hello world  \n\n  hello rust  \n  ";
    
    let cleaned = TextProcessor::clean(text);
    println!("  清理后:\n{}", cleaned);
    
    let counts = TextProcessor::word_count(&cleaned);
    println!("  单词统计: {:?}", counts);
    
    let highlighted = TextProcessor::highlight(&cleaned, "hello");
    println!("  高亮:\n{}", highlighted);
}

// 案例 4: URL 构建器
fn url_builder_example() {
    struct UrlBuilder {
        scheme: String,
        host: String,
        port: Option<u16>,
        path: String,
        query: Vec<(String, String)>,
    }
    
    impl UrlBuilder {
        fn new(host: &str) -> Self {
            UrlBuilder {
                scheme: "http".to_string(),
                host: host.to_string(),
                port: None,
                path: String::new(),
                query: Vec::new(),
            }
        }
        
        fn scheme(mut self, scheme: &str) -> Self {
            self.scheme = scheme.to_string();
            self
        }
        
        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        
        fn path(mut self, path: &str) -> Self {
            self.path = path.to_string();
            self
        }
        
        fn query(mut self, key: &str, value: &str) -> Self {
            self.query.push((key.to_string(), value.to_string()));
            self
        }
        
        fn build(&self) -> String {
            let mut url = format!("{}://{}", self.scheme, self.host);
            
            if let Some(port) = self.port {
                url.push_str(&format!(":{}", port));
            }
            
            if !self.path.is_empty() {
                url.push_str(&self.path);
            }
            
            if !self.query.is_empty() {
                url.push('?');
                let query_string: Vec<String> = self.query
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect();
                url.push_str(&query_string.join("&"));
            }
            
            url
        }
    }
    
    let url = UrlBuilder::new("api.example.com")
        .scheme("https")
        .port(443)
        .path("/v1/users")
        .query("page", "1")
        .query("limit", "10")
        .build();
    
    println!("  构建的 URL: {}", url);
}

/*
=== 总结 ===

1. String vs &str 核心区别:

   特性          String              &str
   ────────────────────────────────────────
   所有权        拥有                借用
   可变性        可变                不可变
   内存          堆分配              静态/栈/堆
   大小          动态                固定
   修改          可以                不可以

2. 使用场景:

   使用 String:
   ✓ 需要拥有所有权
   ✓ 需要修改字符串
   ✓ 动态构建
   ✓ 返回值
   ✓ 结构体字段
   
   使用 &str:
   ✓ 只读访问
   ✓ 函数参数
   ✓ 字符串字面量
   ✓ 临时引用
   ✓ 性能优先

3. 转换:

   &str → String:
   - to_string()   ✓ 推荐
   - to_owned()    ✓ 语义清晰
   - String::from()✓ 显式
   
   String → &str:
   - &string       ✓ 最简单
   - as_str()      ✓ 显式

4. 最佳实践:

   DO:
   ✓ 函数参数使用 &str
   ✓ 返回值使用 String
   ✓ 结构体字段使用 String
   ✓ 预分配 String 容量
   ✓ 避免不必要的克隆
   
   DON'T:
   ✗ 函数参数使用 String
   ✗ 不必要的 to_string()
   ✗ 直接索引访问
   ✗ 忽略 UTF-8 边界

5. 性能tips:

   - &str 零成本（Copy）
   - String 涉及堆分配
   - 使用 with_capacity 预分配
   - 避免频繁 clone
   - 考虑 &'static str

运行示例:
  cargo run --bin string_vs_str_detailed
*/
