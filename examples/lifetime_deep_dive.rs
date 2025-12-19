// 生命周期深度剖析

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              生命周期底层原理深度剖析                    ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    lifetime_basics_demo();
    lifetime_elision_demo();
    lifetime_in_structs_demo();
    lifetime_bounds_demo();
    lifetime_subtyping_demo();
    static_lifetime_demo();
    advanced_lifetime_patterns_demo();
}

/// 1. 生命周期基础
fn lifetime_basics_demo() {
    println!("═══ 1. 生命周期基础 ═══\n");
    
    println!("生命周期的本质：编译时的引用有效性证明");
    println!();
    
    // 最简单的例子
    let x = 5;
    let r = &x;
    println!("✓ r 引用 x: {}", r);
    println!("  编译器验证：r 的生命周期不超过 x");
    
    println!("\n生命周期标注示例：");
    println!("  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str");
    println!("  含义：返回值的生命周期 = min(x的生命周期, y的生命周期)");
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    
    let s1 = String::from("long string");
    let s2 = "short";
    let result = longest(&s1, s2);
    println!("\n实际调用：");
    println!("  longest(\"long string\", \"short\") = \"{}\"", result);
    
    println!("\n生命周期分析：");
    println!("  s1 ────────────────────────┐");
    println!("  s2 ────────────────────────┤");
    println!("  result = longest(&s1, &s2) │");
    println!("  ← result 有效              │");
    println!("  ← s1, s2 仍然有效 ─────────┘");
}

/// 2. 生命周期消除规则
fn lifetime_elision_demo() {
    println!("\n═══ 2. 生命周期消除规则 ═══\n");
    
    println!("编译器自动推导的三条规则：\n");
    
    println!("规则 1：每个引用参数都有独立的生命周期");
    println!("  源码：fn foo(x: &i32, y: &i32)");
    println!("  推导：fn foo<'a, 'b>(x: &'a i32, y: &'b i32)");
    
    println!("\n规则 2：只有一个输入生命周期时，赋给所有输出");
    println!("  源码：fn foo(x: &i32) -> &i32");
    println!("  推导：fn foo<'a>(x: &'a i32) -> &'a i32");
    
    fn first_word(s: &str) -> &str {
        &s[..1]
    }
    let s = "hello";
    println!("\n实际示例：first_word(\"hello\") = \"{}\"", first_word(s));
    
    println!("\n规则 3：方法中，&self 的生命周期赋给所有输出");
    println!("  源码：fn name(&self) -> &str");
    println!("  推导：fn name<'a>(&'a self) -> &'a str");
    
    struct Person {
        name: String,
    }
    
    impl Person {
        fn name(&self) -> &str {
            &self.name
        }
    }
    
    let person = Person {
        name: String::from("Alice"),
    };
    println!("\n实际示例：person.name() = \"{}\"", person.name());
}

/// 3. 结构体中的生命周期
fn lifetime_in_structs_demo() {
    println!("\n═══ 3. 结构体中的生命周期 ═══\n");
    
    println!("包含引用的结构体必须标注生命周期：\n");
    
    #[derive(Debug)]
    struct Excerpt<'a> {
        text: &'a str,
    }
    
    println!("  struct Excerpt<'a> {{");
    println!("      text: &'a str,");
    println!("  }}");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt {
        text: first_sentence,
    };
    
    println!("\n实际使用：");
    println!("  novel: \"{}\"", novel);
    println!("  excerpt.text: \"{}\"", excerpt.text);
    println!("  ✓ excerpt 不能比 novel 活得更久");
    
    println!("\n内存布局：");
    println!("  栈");
    println!("  novel ┌────┐         堆");
    println!("        │ptr │───────> \"Call me Ishmael...\"");
    println!("        ├────┤          ↑");
    println!("        │len │          │");
    println!("        └────┘          │");
    println!("                        │");
    println!("  excerpt ┌────┐        │");
    println!("          │ptr │────────┘");
    println!("          ├────┤");
    println!("          │len │= 15");
    println!("          └────┘");
    
    // 方法中的生命周期
    impl<'a> Excerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        
        fn announce_and_return(&self, announcement: &str) -> &str {
            println!("  公告: {}", announcement);
            self.text
        }
    }
    
    println!("\n结构体方法：");
    println!("  excerpt.level() = {}", excerpt.level());
    let ann = excerpt.announce_and_return("注意！");
    println!("  返回: \"{}\"", ann);
}

/// 4. 生命周期约束
fn lifetime_bounds_demo() {
    println!("\n═══ 4. 生命周期约束 ═══\n");
    
    println!("T: 'a 约束表示 T 必须至少活得和 'a 一样久\n");
    
    struct Ref<'a, T: 'a> {
        value: &'a T,
    }
    
    println!("  struct Ref<'a, T: 'a> {{");
    println!("      value: &'a T,");
    println!("  }}");
    println!("  含义：T 类型本身必须比 'a 活得久");
    
    let x = 42;
    let r = Ref { value: &x };
    println!("\n实例：Ref {{ value: &{} }}", r.value);
    
    println!("\n常见约束场景：");
    println!("  1. where T: 'a       T 比 'a 活得久");
    println!("  2. where 'a: 'b      'a 比 'b 活得久");
    println!("  3. where T: 'static  T 的生命周期是整个程序");
    
    // 实际例子
    fn print_ref<'a, T>(value: &'a T) 
    where
        T: std::fmt::Display + 'a,
    {
        println!("  值: {}", value);
    }
    
    let s = String::from("hello");
    println!("\n约束函数调用：");
    print_ref(&s);
}

/// 5. 生命周期子类型化
fn lifetime_subtyping_demo() {
    println!("\n═══ 5. 生命周期子类型化 ═══\n");
    
    println!("生命周期的协变与逆变：\n");
    
    println!("如果 'long: 'short（'long 至少和 'short 一样长）");
    println!("那么：");
    println!("  ✓ &'long T    可以用在需要 &'short T 的地方（协变）");
    println!("  ✗ &'short T   不能用在需要 &'long T 的地方");
    
    println!("\n可视化：");
    println!("  'static ──────────────────────────────────");
    println!("    'long ──────────────────────────");
    println!("      'short ──────────");
    println!();
    println!("  &'static str 可以转为 &'long str");
    println!("  &'long str   可以转为 &'short str");
    println!("  但反过来不行！");
    
    fn choose<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str 
    where
        'b: 'a,  // 'b 至少和 'a 一样长
    {
        x
    }
    
    let s1 = String::from("longer");
    {
        let s2 = String::from("short");
        let result = choose(&s1, &s2);
        println!("\n实际例子：");
        println!("  choose(\"longer\", \"short\") = \"{}\"", result);
        println!("  ✓ 约束 'b: 'a 确保了类型安全");
    }
}

/// 6. 'static 生命周期
fn static_lifetime_demo() {
    println!("\n═══ 6. 'static 生命周期 ═══\n");
    
    println!("'static 表示引用在整个程序运行期间有效\n");
    
    let s: &'static str = "hello";
    println!("字符串字面量：");
    println!("  let s: &'static str = \"hello\";");
    println!("  s = \"{}\"", s);
    println!("  存储在二进制的 .rodata 段");
    
    println!("\n内存布局：");
    println!("  二进制文件");
    println!("  ┌──────────────┐");
    println!("  │ .rodata 段   │");
    println!("  │ ┌──────────┐ │");
    println!("  │ │ \"hello\\0\"│ │ ← 编译时写入");
    println!("  │ └──────────┘ │");
    println!("  └──────────────┘");
    println!("         ↑");
    println!("         │");
    println!("  栈    ┌┴──┐");
    println!("  s     │ptr│");
    println!("        └───┘");
    
    println!("\n'static 的两种含义：");
    println!("  1. 生命周期：&'static T");
    println!("     引用在整个程序期间有效");
    println!("  2. 约束：T: 'static");
    println!("     类型不包含非 'static 引用");
    
    static GLOBAL: i32 = 42;
    println!("\n全局静态变量：");
    println!("  static GLOBAL: i32 = {};", GLOBAL);
    println!("  地址: {:p}", &GLOBAL);
    println!("  ✓ 永久有效，固定地址");
}

/// 7. 高级生命周期模式
fn advanced_lifetime_patterns_demo() {
    println!("\n═══ 7. 高级生命周期模式 ═══\n");
    
    // 多个生命周期参数
    println!("模式 1：多个生命周期参数");
    fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
    where
        'b: 'a,
    {
        println!("  x (长): \"{}\", y (短): \"{}\"", x, y);
        x
    }
    
    let s1 = String::from("longer");
    let s2 = String::from("short");
    let result = complex(&s1, &s2);
    println!("  返回: \"{}\"", result);
    
    // 生命周期在返回类型中
    println!("\n模式 2：返回闭包");
    fn make_adder() -> Box<dyn Fn(i32) -> i32> {
        let add = Box::new(|x| x + 1);
        println!("  创建闭包: |x| x + 1");
        add
    }
    
    let adder = make_adder();
    println!("  调用: adder(5) = {}", adder(5));
    
    // 高阶trait约束
    println!("\n模式 3：高阶 trait 约束 (HRTB)");
    println!("  for<'a> F: Fn(&'a str) -> &'a str");
    println!("  表示：对于任意生命周期 'a，F 都满足该约束");
    
    fn apply<F>(f: F, s: &str) -> &str 
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        f(s)
    }
    
    let s = "hello";
    let result = apply(|x| x, s);
    println!("  apply(identity, \"hello\") = \"{}\"", result);
    
    println!("\n生命周期总结：");
    println!("  ✓ 编译时概念，零运行时开销");
    println!("  ✓ 保证引用有效性");
    println!("  ✓ 防止悬垂指针");
    println!("  ✓ 消除规则减少手动标注");
    println!("  ✓ 子类型化支持灵活转换");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lifetime_basics() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        
        let s1 = "long";
        let s2 = "short";
        assert_eq!(longest(s1, s2), "short");
    }
    
    #[test]
    fn test_struct_lifetime() {
        struct Wrapper<'a> {
            data: &'a str,
        }
        
        let s = String::from("test");
        let w = Wrapper { data: &s };
        assert_eq!(w.data, "test");
    }
    
    #[test]
    fn test_static_lifetime() {
        let s: &'static str = "hello";
        assert_eq!(s, "hello");
    }
}
