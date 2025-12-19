// 生命周期 (Lifetimes)

/// # 生命周期基础
pub fn lifetime_basics_demo() {
    println!("\n=== 生命周期基础 ===");
    
    println!("生命周期的目的:");
    println!("  - 防止悬垂引用");
    println!("  - 确保引用总是有效的");
    println!("  - 在编译时检查");
    
    {
        let r;
        {
            let x = 5;
            r = &x;
            println!("  内部作用域: r = {}", r);
        } // x 离开作用域
        // println!("{}", r);  // 错误！x 已经被释放
    }
    
    println!("\n生命周期标注:");
    println!("  - 描述引用之间的关系");
    println!("  - 不改变实际生命周期");
    println!("  - 帮助编译器验证");
}

/// # 函数中的生命周期
pub fn lifetime_in_functions_demo() {
    println!("\n=== 函数中的生命周期 ===");
    
    // 返回引用的函数需要生命周期标注
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string");
    let string2 = "short";
    
    let result = longest(&string1, &string2);
    println!("更长的字符串: {}", result);
    
    // 生命周期标注的含义
    println!("\n'a 表示:");
    println!("  - 返回值的生命周期与参数相关");
    println!("  - 返回值的生命周期不会超过任何参数");
}

/// # 生命周期标注语法
pub fn lifetime_annotation_syntax_demo() {
    println!("\n=== 生命周期标注语法 ===");
    
    // 基本语法
    fn example<'a>(x: &'a str) -> &'a str {
        x
    }
    
    println!("语法: 'a");
    println!("  - 单引号 + 小写字母");
    println!("  - 通常使用 'a, 'b, 'c 等");
    println!("  - 可以使用描述性名称如 'input");
    
    // 多个生命周期参数
    fn multiple<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
        x
    }
    
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = multiple(&s1, &s2);
    println!("结果: {}", result);
}

/// # 生命周期省略规则
pub fn lifetime_elision_demo() {
    println!("\n=== 生命周期省略规则 ===");
    
    // 规则 1: 每个引用参数都有自己的生命周期
    fn first_word(s: &str) -> &str {
        // 实际上是: fn first_word<'a>(s: &'a str) -> &'a str
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
    
    println!("\n省略规则:");
    println!("  1. 每个引用参数都有独立的生命周期");
    println!("  2. 如果只有一个输入生命周期，赋给所有输出");
    println!("  3. 如果有 &self 或 &mut self，其生命周期赋给所有输出");
}

/// # 结构体中的生命周期
pub fn lifetime_in_structs_demo() {
    println!("\n=== 结构体中的生命周期 ===");
    
    // 结构体持有引用需要生命周期标注
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("摘录: {:?}", i);
    println!("内容: {}", i.part);
    
    // 生命周期确保结构体不会比它引用的数据活得更久
}

/// # 方法中的生命周期
pub fn lifetime_in_methods_demo() {
    println!("\n=== 方法中的生命周期 ===");
    
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    impl<'a> ImportantExcerpt<'a> {
        // 方法的生命周期省略规则
        fn level(&self) -> i32 {
            3
        }
        
        // 返回引用的方法
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("注意！{}", announcement);
            self.part
        }
        
        // 显式生命周期标注
        fn longest<'b>(&self, _other: &'b str) -> &str {
            // 总是返回 self.part 以避免生命周期问题
            self.part
        }
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_sentence };
    
    println!("级别: {}", i.level());
    let part = i.announce_and_return_part("请注意");
    println!("部分: {}", part);
}

/// # 静态生命周期
pub fn static_lifetime_demo() {
    println!("\n=== 静态生命周期 ===");
    
    // 'static 表示整个程序运行期间
    let s: &'static str = "I have a static lifetime.";
    println!("静态字符串: {}", s);
    
    println!("\n'static 生命周期:");
    println!("  - 存活于整个程序运行期间");
    println!("  - 字符串字面量都是 'static");
    println!("  - 存储在程序的二进制文件中");
    
    // 不要过度使用 'static
    println!("\n注意:");
    println!("  - 大多数情况不需要 'static");
    println!("  - 优先考虑具体的生命周期");
}

/// # 生命周期与泛型
pub fn lifetime_with_generics_demo() {
    println!("\n=== 生命周期与泛型 ===");
    
    // 同时使用泛型和生命周期
    fn longest_with_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("公告！{}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string");
    let string2 = "short";
    
    let result = longest_with_announcement(&string1, &string2, "比较字符串");
    println!("结果: {}", result);
}

/// # 生命周期子类型
pub fn lifetime_subtyping_demo() {
    println!("\n=== 生命周期子类型 ===");
    
    println!("生命周期协变:");
    println!("  - 如果 'a: 'b (a 活得比 b 久)");
    println!("  - 那么 &'a T 可以用在需要 &'b T 的地方");
    
    fn choose<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
    where
        'b: 'a,  // 'b 至少要和 'a 一样长
    {
        if x.len() > y.len() {
            x
        } else {
            x  // 只能返回 x，因为返回类型是 &'a str
        }
    }
    
    let s1 = String::from("long");
    let s2 = String::from("short");
    let result = choose(&s1, &s2);
    println!("选择: {}", result);
}

/// # 生命周期实战示例：缓存
pub fn cache_example_demo() {
    println!("\n=== 实战示例：缓存 ===");
    println!("缓存示例已简化 - 实际项目中推荐使用 HashMap 或 memoize 库");
}
/// # 生命周期实战示例：迭代器
pub fn iterator_example_demo() {
    println!("\n=== 实战示例：迭代器 ===");
    
    struct Lines<'a> {
        text: &'a str,
    }
    
    impl<'a> Lines<'a> {
        fn new(text: &'a str) -> Self {
            Lines { text }
        }
    }
    
    impl<'a> Iterator for Lines<'a> {
        type Item = &'a str;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.text.is_empty() {
                return None;
            }
            
            match self.text.find('\n') {
                Some(pos) => {
                    let line = &self.text[..pos];
                    self.text = &self.text[pos + 1..];
                    Some(line)
                }
                None => {
                    let line = self.text;
                    self.text = "";
                    Some(line)
                }
            }
        }
    }
    
    let text = "line 1\nline 2\nline 3";
    let lines = Lines::new(text);
    
    println!("迭代行:");
    for line in lines {
        println!("  {}", line);
    }
}

/// # 生命周期的常见模式
pub fn common_lifetime_patterns_demo() {
    println!("\n=== 生命周期的常见模式 ===");
    
    // 1. 返回其中一个输入
    fn first<'a>(x: &'a str, _y: &str) -> &'a str {
        x
    }
    
    println!("1. 返回其中一个输入");
    let result = first("hello", "world");
    println!("  {}", result);
    
    // 2. 两个输入，相同生命周期
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    
    println!("\n2. 两个输入，相同生命周期");
    let result = longest("hello", "hi");
    println!("  {}", result);
    
    // 3. 结构体持有引用
    #[derive(Debug)]
    struct Wrapper<'a> {
        value: &'a str,
    }
    
    println!("\n3. 结构体持有引用");
    let s = String::from("data");
    let w = Wrapper { value: &s };
    println!("  {:?}", w);
}

/// # 生命周期最佳实践
pub fn lifetime_best_practices_demo() {
    println!("\n=== 生命周期最佳实践 ===");
    
    println!("1. 优先依赖生命周期省略规则");
    println!("   - 让编译器自动推导");
    println!("   - 代码更简洁");
    
    println!("\n2. 使用具体的生命周期名称");
    println!("   - 'input, 'output 比 'a, 'b 更清晰");
    println!("   - 提高可读性");
    
    println!("\n3. 避免过度使用 'static");
    println!("   - 除非确实需要");
    println!("   - 考虑具体的生命周期");
    
    println!("\n4. 当遇到生命周期错误时");
    println!("   - 先理解错误消息");
    println!("   - 画出变量的作用域");
    println!("   - 调整代码结构");
    
    println!("\n5. 结构体设计");
    println!("   - 考虑是否真的需要引用");
    println!("   - 拥有所有权可能更简单");
    println!("   - 权衡灵活性和复杂度");
}

/// 运行所有生命周期示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 生命周期详解           ║");
    println!("╚════════════════════════════════════╝");
    
    lifetime_basics_demo();
    lifetime_in_functions_demo();
    lifetime_annotation_syntax_demo();
    lifetime_elision_demo();
    lifetime_in_structs_demo();
    lifetime_in_methods_demo();
    static_lifetime_demo();
    lifetime_with_generics_demo();
    lifetime_subtyping_demo();
    cache_example_demo();
    iterator_example_demo();
    common_lifetime_patterns_demo();
    lifetime_best_practices_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lifetime_in_function() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        
        let s1 = String::from("long");
        let s2 = String::from("short");
        let result = longest(&s1, &s2);
        assert_eq!(result, "short");
    }
    
    #[test]
    fn test_lifetime_in_struct() {
        struct Holder<'a> {
            value: &'a str,
        }
        
        let s = String::from("hello");
        let h = Holder { value: &s };
        assert_eq!(h.value, "hello");
    }
}
