// 借用 (Borrowing)

/// # 借用规则
/// 
/// 1. 在任意时刻，要么只有一个可变引用，要么有任意数量的不可变引用
/// 2. 引用必须总是有效的
pub fn borrowing_rules_demo() {
    println!("\n=== 借用规则 ===");
    
    println!("规则 1: 要么一个可变引用，要么多个不可变引用");
    let s = String::from("hello");
    
    let r1 = &s;  // 不可变引用
    let r2 = &s;  // 可以有多个不可变引用
    println!("  不可变引用: {}, {}", r1, r2);
    
    // let r3 = &mut s;  // 错误！不能在有不可变引用时创建可变引用
    
    println!("\n规则 2: 引用必须总是有效的");
    println!("  - Rust 防止悬垂引用");
    println!("  - 编译器确保数据不会在引用之前被释放");
}

/// # 不可变引用（不可变借用）
pub fn immutable_references_demo() {
    println!("\n=== 不可变引用 ===");
    
    let s = String::from("hello");
    
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    
    let len = calculate_length(&s);
    println!("字符串 '{}' 的长度是 {}", s, len);
    
    // 可以有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("多个不可变引用: {}, {}, {}", r1, r2, r3);
    
    // 不可变引用不能修改数据
    fn try_modify(s: &String) {
        // s.push_str(" world");  // 错误！不能通过不可变引用修改
        println!("  只能读取: {}", s);
    }
    
    try_modify(&s);
}

/// # 可变引用（可变借用）
pub fn mutable_references_demo() {
    println!("\n=== 可变引用 ===");
    
    let mut s = String::from("hello");
    
    fn append_world(s: &mut String) {
        s.push_str(" world");
    }
    
    println!("修改前: {}", s);
    append_world(&mut s);
    println!("修改后: {}", s);
    
    // 同一时刻只能有一个可变引用
    let r1 = &mut s;
    // let r2 = &mut s;  // 错误！不能同时有两个可变引用
    r1.push_str("!");
    println!("可变引用修改: {}", r1);
    
    // 可变引用的作用域结束后，可以创建新的可变引用
    let r2 = &mut s;
    r2.push_str("!");
    println!("新的可变引用: {}", r2);
}

/// # 可变与不可变引用的规则
pub fn mixed_references_demo() {
    println!("\n=== 可变与不可变引用的规则 ===");
    
    let mut s = String::from("hello");
    
    {
        let r1 = &s;  // 不可变引用
        let r2 = &s;  // 不可变引用
        println!("不可变引用: {}, {}", r1, r2);
        // r1 和 r2 的作用域结束
    }
    
    let r3 = &mut s;  // 可变引用
    r3.push_str(" world");
    println!("可变引用: {}", r3);
    
    // 注意：不能在不可变引用存在时创建可变引用
    let s2 = String::from("hello");
    let _r1 = &s2;
    // let r2 = &mut s2;  // 错误！
    // println!("{}, {}", _r1, r2);
    println!("示例说明：不可变与可变引用不能同时存在");
}

/// # 引用的作用域
pub fn reference_scope_demo() {
    println!("\n=== 引用的作用域 ===");
    
    let mut s = String::from("hello");
    
    {
        let r1 = &s;
        println!("作用域内的不可变引用: {}", r1);
    } // r1 离开作用域
    
    let r2 = &mut s;
    r2.push_str(" world");
    println!("可变引用: {}", r2);
    
    // 非词法作用域生命周期（NLL）
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    println!("最后使用不可变引用: {} {}", r1, r2);
    // r1 和 r2 在这里之后不再使用
    
    let r3 = &mut s;  // 这里可以创建可变引用了
    r3.push_str("!");
    println!("可变引用: {}", r3);
}

/// # 悬垂引用
pub fn dangling_references_demo() {
    println!("\n=== 悬垂引用 ===");
    
    // Rust 防止悬垂引用
    /*
    fn dangle() -> &String {
        let s = String::from("hello");
        &s  // 错误！返回局部变量的引用
    } // s 离开作用域被丢弃，引用无效
    */
    
    // 正确做法：返回所有权
    fn no_dangle() -> String {
        let s = String::from("hello");
        s  // 返回所有权
    }
    
    let s = no_dangle();
    println!("正确返回: {}", s);
    
    println!("\nRust 编译器保证：");
    println!("  - 引用总是有效的");
    println!("  - 不会有悬垂指针");
    println!("  - 在编译时检查");
}

/// # 借用与函数
pub fn borrowing_with_functions_demo() {
    println!("\n=== 借用与函数 ===");
    
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    println!("原字符串仍可用: {}", s);
    
    // 借用检查器防止错误
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();  // 错误！不能在有不可变借用时修改
    println!("单词: {}", word);
}

/// # 多个参数的借用
pub fn multiple_borrows_demo() {
    println!("\n=== 多个参数的借用 ===");
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let s1 = String::from("long string");
    let s2 = String::from("short");
    
    let result = longest(&s1, &s2);
    println!("更长的字符串: {}", result);
    
    // 可以同时借用多个不可变引用
    fn compare_strings(s1: &String, s2: &String) -> bool {
        s1 == s2
    }
    
    println!("字符串相等: {}", compare_strings(&s1, &s2));
}

/// # 借用切片
pub fn borrowing_slices_demo() {
    println!("\n=== 借用切片 ===");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("切片: '{}' '{}'", hello, world);
    println!("原字符串: {}", s);
    
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    
    println!("\n数组切片: {:?}", slice);
    println!("原数组: {:?}", a);
}

/// # 结构体中的借用
pub fn struct_borrowing_demo() {
    println!("\n=== 结构体中的借用 ===");
    
    // 结构体持有引用需要生命周期标注
    #[derive(Debug)]
    struct TextAnalyzer<'a> {
        text: &'a str,
    }
    
    impl<'a> TextAnalyzer<'a> {
        fn new(text: &'a str) -> Self {
            TextAnalyzer { text }
        }
        
        fn word_count(&self) -> usize {
            self.text.split_whitespace().count()
        }
        
        fn char_count(&self) -> usize {
            self.text.chars().count()
        }
    }
    
    let text = String::from("Hello, world! How are you?");
    let analyzer = TextAnalyzer::new(&text);
    
    println!("分析文本: {:?}", analyzer);
    println!("单词数: {}", analyzer.word_count());
    println!("字符数: {}", analyzer.char_count());
}

/// # 借用的实际应用
pub fn practical_borrowing_demo() {
    println!("\n=== 借用的实际应用 ===");
    
    // 1. 避免不必要的复制
    fn print_vec(v: &Vec<i32>) {
        for item in v {
            print!("{} ", item);
        }
        println!();
    }
    
    let v = vec![1, 2, 3, 4, 5];
    println!("1. 避免复制:");
    print_vec(&v);
    print_vec(&v);  // 可以多次使用
    
    // 2. 提供只读访问
    fn get_max(v: &Vec<i32>) -> Option<&i32> {
        v.iter().max()
    }
    
    if let Some(max) = get_max(&v) {
        println!("\n2. 只读访问 - 最大值: {}", max);
    }
    
    // 3. 安全的迭代
    println!("\n3. 安全的迭代:");
    for item in &v {
        print!("{} ", item);
    }
    println!();
    
    // 4. 链式方法调用
    let s = String::from("  hello world  ");
    let result = s.trim().to_uppercase();
    println!("\n4. 链式调用: {}", result);
}

/// # 借用的性能优势
pub fn borrowing_performance_demo() {
    println!("\n=== 借用的性能优势 ===");
    
    println!("借用 vs 复制:");
    
    let large_vec = vec![1; 1000000];
    
    // 借用：只传递引用（指针大小）
    fn process_borrow(v: &Vec<i32>) -> usize {
        v.len()
    }
    
    println!("  借用: {} 字节（指针大小）", std::mem::size_of::<&Vec<i32>>());
    let _len = process_borrow(&large_vec);
    
    // 克隆：复制整个 Vec（昂贵）
    println!("  克隆: 复制 {} 个元素", large_vec.len());
    
    println!("\n性能建议:");
    println!("  - 默认使用借用");
    println!("  - 只在必要时克隆");
    println!("  - 考虑使用 Rc/Arc 共享所有权");
}

/// 运行所有借用示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║          Rust 借用详解             ║");
    println!("╚════════════════════════════════════╝");
    
    borrowing_rules_demo();
    immutable_references_demo();
    mutable_references_demo();
    mixed_references_demo();
    reference_scope_demo();
    dangling_references_demo();
    borrowing_with_functions_demo();
    multiple_borrows_demo();
    borrowing_slices_demo();
    struct_borrowing_demo();
    practical_borrowing_demo();
    borrowing_performance_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_immutable_borrow() {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        assert_eq!(r1, r2);
    }
    
    #[test]
    fn test_mutable_borrow() {
        let mut s = String::from("hello");
        let r = &mut s;
        r.push_str(" world");
        assert_eq!(r, "hello world");
    }
}
