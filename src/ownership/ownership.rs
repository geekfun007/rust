// 所有权 (Ownership)

/// # 所有权规则
/// 
/// 1. Rust 中的每个值都有一个所有者
/// 2. 值在任一时刻只能有一个所有者
/// 3. 当所有者离开作用域，值将被丢弃
pub fn ownership_rules_demo() {
    println!("\n=== 所有权规则 ===");
    
    println!("规则 1: 每个值都有一个所有者");
    let s = String::from("hello");
    println!("  s 拥有 String \"{}\"", s);
    
    println!("\n规则 2: 值在任一时刻只能有一个所有者");
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权移动到 s2
    // println!("{}", s1);  // 错误！s1 不再有效
    println!("  s2 现在拥有 String \"{}\"", s2);
    
    println!("\n规则 3: 当所有者离开作用域，值将被丢弃");
    {
        let s = String::from("hello");
        println!("  作用域内: {}", s);
    } // s 在这里被丢弃
    println!("  s 已经被丢弃，不再可用");
}

/// # 变量与数据的交互：移动
pub fn move_demo() {
    println!("\n=== 移动 (Move) ===");
    
    // 栈上的数据：复制
    let x = 5;
    let y = x;  // 复制
    println!("x = {}, y = {}", x, y);  // 两者都可用
    
    // 堆上的数据：移动
    let s1 = String::from("hello");
    let s2 = s1;  // 移动
    // println!("{}", s1);  // 错误！s1 已被移动
    println!("s2 = {}", s2);
    
    println!("\n为什么会移动？");
    println!("  String 包含：指针、长度、容量");
    println!("  如果简单复制，两个变量会指向同一块内存");
    println!("  当两者离开作用域时，会尝试释放同一块内存（双重释放）");
    println!("  所以 Rust 选择移动：使第一个变量无效");
}

/// # 变量与数据的交互：克隆
pub fn clone_demo() {
    println!("\n=== 克隆 (Clone) ===");
    
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝
    
    println!("s1 = {}", s1);  // s1 仍然有效
    println!("s2 = {}", s2);  // s2 也有效
    
    println!("\nclone 会:");
    println!("  - 复制堆上的数据");
    println!("  - 创建新的内存分配");
    println!("  - 可能比较昂贵");
    
    // 对于实现了 Copy trait 的类型，不需要 clone
    let x = 5;
    let y = x;  // 自动复制
    println!("\n整数等简单类型自动复制: x = {}, y = {}", x, y);
}

/// # Copy trait
pub fn copy_trait_demo() {
    println!("\n=== Copy Trait ===");
    
    println!("实现了 Copy trait 的类型会自动复制而不是移动：");
    println!("  - 所有整数类型：i32, u32, 等");
    println!("  - 布尔类型：bool");
    println!("  - 浮点类型：f32, f64");
    println!("  - 字符类型：char");
    println!("  - 元组（如果所有元素都实现 Copy）");
    
    // 示例
    let x = 5;
    let y = x;
    println!("\nx = {}, y = {}", x, y);
    
    let tuple = (1, 2.0, true);
    let tuple2 = tuple;
    println!("tuple = {:?}, tuple2 = {:?}", tuple, tuple2);
    
    println!("\n不能实现 Copy 的类型：");
    println!("  - String");
    println!("  - Vec<T>");
    println!("  - 任何包含非 Copy 类型的类型");
}

/// # 所有权与函数
pub fn ownership_and_functions_demo() {
    println!("\n=== 所有权与函数 ===");
    
    fn takes_ownership(s: String) {
        println!("  函数获取所有权: {}", s);
    } // s 在这里被丢弃
    
    fn makes_copy(x: i32) {
        println!("  函数复制值: {}", x);
    }
    
    let s = String::from("hello");
    println!("调用前: {}", s);
    takes_ownership(s);  // s 的所有权移动到函数中
    // println!("{}", s);  // 错误！s 已被移动
    
    let x = 5;
    println!("\n调用前: {}", x);
    makes_copy(x);  // x 被复制到函数中
    println!("调用后: {}", x);  // x 仍然有效
}

/// # 返回值与所有权
pub fn return_values_demo() {
    println!("\n=== 返回值与所有权 ===");
    
    fn gives_ownership() -> String {
        let s = String::from("hello");
        s  // 返回 s，所有权移出函数
    }
    
    fn takes_and_gives_back(s: String) -> String {
        s  // 返回 s，所有权移出函数
    }
    
    let s1 = gives_ownership();
    println!("获取所有权: {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2);  // 错误！s2 已被移动
    println!("返回所有权: {}", s3);
    
    // 如果想在函数中使用值，又想保留所有权，可以返回元组
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)  // 返回所有权和长度
    }
    
    let s = String::from("hello");
    let (s, len) = calculate_length(s);
    println!("字符串 '{}' 的长度是 {}", s, len);
}

/// # 所有权的实际应用
pub fn practical_ownership_demo() {
    println!("\n=== 所有权的实际应用 ===");
    
    // 资源管理：自动清理
    println!("1. 自动资源管理:");
    {
        let data = vec![1, 2, 3, 4, 5];
        println!("  数据: {:?}", data);
    } // data 在这里自动被清理
    println!("  Vec 的内存已被释放");
    
    // 防止数据竞争
    println!("\n2. 防止数据竞争:");
    println!("  - 在编译时保证内存安全");
    println!("  - 不会有悬垂指针");
    println!("  - 不会有双重释放");
    
    // 明确的数据流
    println!("\n3. 明确的数据流:");
    let data = vec![1, 2, 3];
    let data = process_data(data);  // 明确的所有权转移
    println!("  处理后的数据: {:?}", data);
}

fn process_data(mut data: Vec<i32>) -> Vec<i32> {
    data.push(4);
    data
}

/// # 所有权模式
pub fn ownership_patterns_demo() {
    println!("\n=== 所有权模式 ===");
    
    // 模式 1: 消费者模式
    println!("1. 消费者模式 - 获取所有权并消费");
    fn consume(s: String) {
        println!("  消费: {}", s);
    }
    let s = String::from("hello");
    consume(s);
    // s 不再可用
    
    // 模式 2: 借用模式（见 borrowing.rs）
    println!("\n2. 借用模式 - 使用引用而不获取所有权");
    fn borrow(s: &String) {
        println!("  借用: {}", s);
    }
    let s = String::from("hello");
    borrow(&s);
    println!("  仍然可用: {}", s);
    
    // 模式 3: 构建器模式
    println!("\n3. 构建器模式 - 链式调用转移所有权");
    struct Builder {
        value: String,
    }
    
    impl Builder {
        fn new(value: String) -> Self {
            Builder { value }
        }
        
        fn add(mut self, s: &str) -> Self {
            self.value.push_str(s);
            self
        }
        
        fn build(self) -> String {
            self.value
        }
    }
    
    let result = Builder::new(String::from("Hello"))
        .add(" ")
        .add("World")
        .build();
    
    println!("  构建结果: {}", result);
}

/// # 所有权与容器
pub fn ownership_with_containers_demo() {
    println!("\n=== 所有权与容器 ===");
    
    // Vec 拥有其元素
    let mut v = Vec::new();
    v.push(String::from("hello"));
    v.push(String::from("world"));
    
    println!("Vec 拥有元素:");
    for s in &v {
        println!("  {}", s);
    }
    
    // 移动出 Vec
    let first = v.remove(0);  // 移动第一个元素
    println!("\n移动出的元素: {}", first);
    println!("剩余元素: {:?}", v);
    
    // HashMap 拥有键和值
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let key = String::from("key");
    let value = String::from("value");
    map.insert(key, value);
    // key 和 value 已被移动
    // println!("{}", key);  // 错误！
    
    println!("\nHashMap: {:?}", map);
}

/// # 所有权的开销
pub fn ownership_cost_demo() {
    println!("\n=== 所有权的开销 ===");
    
    println!("移动的开销：");
    println!("  - 通常只是几个字节的栈复制");
    println!("  - 不涉及堆内存的复制");
    println!("  - 非常高效");
    
    println!("\n克隆的开销：");
    println!("  - 需要分配新内存");
    println!("  - 复制堆上的数据");
    println!("  - 可能比较昂贵");
    
    println!("\n建议：");
    println!("  - 优先使用借用（引用）");
    println!("  - 需要所有权时使用移动");
    println!("  - 必要时才使用克隆");
}

/// 运行所有所有权示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 所有权详解             ║");
    println!("╚════════════════════════════════════╝");
    
    ownership_rules_demo();
    move_demo();
    clone_demo();
    copy_trait_demo();
    ownership_and_functions_demo();
    return_values_demo();
    practical_ownership_demo();
    ownership_patterns_demo();
    ownership_with_containers_demo();
    ownership_cost_demo();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_move() {
        let s1 = String::from("hello");
        let s2 = s1;
        assert_eq!(s2, "hello");
        // s1 不再可用
    }
    
    #[test]
    fn test_clone() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
    }
    
    #[test]
    fn test_copy() {
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }
}
