// String vs &str 详解

/// # String 和 &str 的区别
/// 
/// - String: 可增长的、堆分配的字符串类型（拥有所有权）
/// - &str: 字符串切片，不可变引用（借用）
pub fn string_vs_str_demo() {
    println!("\n=== String vs &str ===");
    
    // 字符串字面量是 &str 类型
    let s1: &str = "Hello, world!";
    println!("&str 字面量: {}", s1);
    println!("存储位置: 程序只读数据段");
    println!("大小: {} 字节 (指针+长度)", std::mem::size_of_val(&s1));
    
    // String 是堆分配的
    let s2: String = String::from("Hello, world!");
    println!("\nString: {}", s2);
    println!("存储位置: 堆");
    println!("大小: {} 字节 (指针+长度+容量)", std::mem::size_of_val(&s2));
    println!("容量: {}", s2.capacity());
    println!("长度: {}", s2.len());
}

/// # 创建字符串
pub fn creating_strings_demo() {
    println!("\n=== 创建字符串 ===");
    
    // 从字面量创建 String
    let s1 = String::from("Hello");
    let s2 = "World".to_string();
    let s3 = "Rust".to_owned();
    
    println!("String::from(): {}", s1);
    println!("to_string(): {}", s2);
    println!("to_owned(): {}", s3);
    
    // 空字符串
    let mut s = String::new();
    println!("空字符串: \"{}\"", s);
    
    // 预分配容量
    let mut s = String::with_capacity(10);
    println!("预分配容量: {}", s.capacity());
    s.push_str("hello");
    println!("添加后: \"{}\" (容量: {})", s, s.capacity());
    
    // 重复字符串
    let s = "ha".repeat(3);
    println!("重复: {}", s);
}

/// # String 方法
pub fn string_methods_demo() {
    println!("\n=== String 方法 ===");
    
    let mut s = String::from("Hello");
    
    // 追加字符串
    s.push_str(", world");
    println!("push_str: {}", s);
    
    // 追加字符
    s.push('!');
    println!("push: {}", s);
    
    // 插入
    let mut s = String::from("Hello world");
    s.insert(5, ',');
    println!("insert: {}", s);
    
    s.insert_str(6, " beautiful");
    println!("insert_str: {}", s);
    
    // 替换
    let s = String::from("Hello world");
    let s2 = s.replace("world", "Rust");
    println!("replace: {}", s2);
    
    let s2 = s.replacen("l", "L", 2);
    println!("replacen: {}", s2);
    
    // 删除
    let mut s = String::from("Hello, world!");
    s.remove(5);  // 删除逗号
    println!("remove: {}", s);
    
    s.pop();  // 删除最后一个字符
    println!("pop: {}", s);
    
    let mut s = String::from("Hello, world!");
    s.truncate(5);  // 截断到指定长度
    println!("truncate: {}", s);
    
    let mut s = String::from("Hello, world!");
    s.clear();  // 清空
    println!("clear: \"{}\"", s);
}

/// # &str 方法
pub fn str_methods_demo() {
    println!("\n=== &str 方法 ===");
    
    let s = "Hello, world!";
    
    // 长度
    println!("len: {}", s.len());
    println!("is_empty: {}", s.is_empty());
    
    // 包含
    println!("contains(\"world\"): {}", s.contains("world"));
    println!("starts_with(\"Hello\"): {}", s.starts_with("Hello"));
    println!("ends_with(\"!\"): {}", s.ends_with("!"));
    
    // 查找
    match s.find("world") {
        Some(index) => println!("find(\"world\"): {}", index),
        None => println!("find(\"world\"): 未找到"),
    }
    
    // 分割
    println!("\nsplit:");
    for word in s.split(", ") {
        println!("  {}", word);
    }
    
    let parts: Vec<&str> = s.split(", ").collect();
    println!("collect: {:?}", parts);
    
    // 分割行
    let text = "line 1\nline 2\nline 3";
    println!("\nlines:");
    for line in text.lines() {
        println!("  {}", line);
    }
    
    // 分割空白
    let text = "one  two\tthree\nfour";
    println!("\nsplit_whitespace:");
    for word in text.split_whitespace() {
        println!("  {}", word);
    }
}

/// # 字符串切片
pub fn string_slicing_demo() {
    println!("\n=== 字符串切片 ===");
    
    let s = String::from("Hello, world!");
    
    // 切片
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("切片: \"{}\" \"{}\"", hello, world);
    
    // 简写
    let hello = &s[..5];
    let world = &s[7..];
    let all = &s[..];
    println!("简写: \"{}\" \"{}\" \"{}\"", hello, world, all);
    
    // 注意：必须在字符边界上切片
    let s = "你好世界";
    let hello = &s[0..6];  // 每个汉字占3字节
    println!("中文切片: {}", hello);
    
    // 错误示例（会panic）：
    // let bad = &s[0..1];  // 不是字符边界！
}

/// # 字符串遍历
pub fn string_iteration_demo() {
    println!("\n=== 字符串遍历 ===");
    
    let s = "Hello, 世界!";
    
    // 按字符遍历
    println!("按字符:");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();
    
    // 按字节遍历
    println!("\n按字节:");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();
    
    // 字符数量
    let char_count = s.chars().count();
    let byte_count = s.len();
    println!("\n字符数: {}", char_count);
    println!("字节数: {}", byte_count);
    
    // 字符索引
    println!("\n字符索引:");
    for (i, c) in s.char_indices() {
        println!("  索引 {}: '{}'", i, c);
    }
}

/// # 字符串格式化
pub fn string_formatting_demo() {
    println!("\n=== 字符串格式化 ===");
    
    let name = "Alice";
    let age = 30;
    
    // format! 宏
    let s = format!("{} is {} years old", name, age);
    println!("{}", s);
    
    // 位置参数
    let s = format!("{0} {1} {0}", "A", "B");
    println!("{}", s);
    
    // 命名参数
    let s = format!("{name} is {age} years old", name=name, age=age);
    println!("{}", s);
    
    // 格式化选项
    let pi = 3.14159;
    println!("保留2位小数: {:.2}", pi);
    println!("宽度10: {:>10}", "text");
    println!("填充0: {:0>10}", 42);
    
    // 连接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 被移动
    println!("+ 连接: {}", s3);
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format! 连接: {}", s);
}

/// # 字符串转换
pub fn string_conversion_demo() {
    println!("\n=== 字符串转换 ===");
    
    // &str -> String
    let s1: &str = "hello";
    let s2: String = s1.to_string();
    let s3: String = String::from(s1);
    let s4: String = s1.to_owned();
    println!("&str -> String: {}", s2);
    
    // String -> &str
    let s: String = String::from("hello");
    let s_ref: &str = &s;
    let s_ref2: &str = s.as_str();
    println!("String -> &str: {}", s_ref);
    
    // 大小写转换
    let s = "Hello, World!";
    println!("to_lowercase: {}", s.to_lowercase());
    println!("to_uppercase: {}", s.to_uppercase());
    
    // 去除空白
    let s = "  hello  ";
    println!("trim: \"{}\"", s.trim());
    println!("trim_start: \"{}\"", s.trim_start());
    println!("trim_end: \"{}\"", s.trim_end());
    
    // 解析
    let s = "42";
    let num: i32 = s.parse().unwrap();
    println!("parse: {}", num);
    
    let s = "3.14";
    let float: f64 = s.parse().unwrap();
    println!("parse: {}", float);
    
    let s = "true";
    let boolean: bool = s.parse().unwrap();
    println!("parse: {}", boolean);
}

/// # Unicode 和 UTF-8
pub fn unicode_demo() {
    println!("\n=== Unicode 和 UTF-8 ===");
    
    let s = "你好，世界！";
    
    println!("字符串: {}", s);
    println!("字符数: {}", s.chars().count());
    println!("字节数: {}", s.len());
    
    // UTF-8 编码
    println!("\nUTF-8 字节:");
    for (i, byte) in s.bytes().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 3 == 0 {
            println!();
        }
    }
    println!();
    
    // Unicode 码点
    println!("\nUnicode 码点:");
    for c in s.chars() {
        println!("  '{}' = U+{:04X}", c, c as u32);
    }
    
    // emoji 处理
    let emoji = "👨‍👩‍👧‍👦";  // 家庭 emoji（由多个码点组成）
    println!("\nEmoji: {}", emoji);
    println!("字符数: {}", emoji.chars().count());
    println!("字节数: {}", emoji.len());
}

/// # 字符串性能考虑
pub fn string_performance_demo() {
    println!("\n=== 字符串性能 ===");
    
    // 预分配容量
    let mut s = String::with_capacity(100);
    println!("初始容量: {}", s.capacity());
    
    for i in 0..10 {
        s.push_str("hello");
    }
    
    println!("添加后容量: {}", s.capacity());
    println!("长度: {}", s.len());
    
    // 避免不必要的分配
    // 不好：多次分配
    let mut s = String::new();
    s = s + "a";
    s = s + "b";
    s = s + "c";
    
    // 好：一次分配
    let mut s = String::new();
    s.push('a');
    s.push('b');
    s.push('c');
    
    println!("最终字符串: {}", s);
    
    // 使用 &str 避免不必要的拥有权
    fn print_string(s: &str) {
        println!("{}", s);
    }
    
    let s = String::from("hello");
    print_string(&s);  // 借用而不是移动
    println!("仍然可以使用: {}", s);
}

/// # 常见字符串操作
pub fn common_operations_demo() {
    println!("\n=== 常见字符串操作 ===");
    
    let s = "Hello, World!";
    
    // 反转字符串
    let reversed: String = s.chars().rev().collect();
    println!("反转: {}", reversed);
    
    // 统计字符
    let count = s.chars().filter(|c| c.is_alphabetic()).count();
    println!("字母数量: {}", count);
    
    // 查找并替换
    let s = "one two three";
    let s = s.replace("two", "2");
    println!("替换: {}", s);
    
    // 分割并收集
    let s = "apple,banana,cherry";
    let fruits: Vec<&str> = s.split(',').collect();
    println!("分割: {:?}", fruits);
    
    // 连接
    let fruits = vec!["apple", "banana", "cherry"];
    let s = fruits.join(", ");
    println!("连接: {}", s);
    
    // 重复
    let s = "ha".repeat(3);
    println!("重复: {}", s);
    
    // 填充
    let s = "42";
    let padded = format!("{:0>5}", s);
    println!("左填充: {}", padded);
}

/// 运行所有字符串示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║       String vs &str 详解          ║");
    println!("╚════════════════════════════════════╝");
    
    string_vs_str_demo();
    creating_strings_demo();
    string_methods_demo();
    str_methods_demo();
    string_slicing_demo();
    string_iteration_demo();
    string_formatting_demo();
    string_conversion_demo();
    unicode_demo();
    string_performance_demo();
    common_operations_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_creation() {
        let s1 = String::from("hello");
        let s2 = "hello".to_string();
        assert_eq!(s1, s2);
    }
    
    #[test]
    fn test_string_methods() {
        let mut s = String::from("hello");
        s.push_str(", world");
        assert_eq!(s, "hello, world");
    }
    
    #[test]
    fn test_string_slicing() {
        let s = "hello";
        assert_eq!(&s[0..2], "he");
        assert_eq!(&s[2..], "llo");
    }
}
