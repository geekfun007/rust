// Rust Into 系列转换详解
//
// 本教程涵盖所有 into_ 开头的转换方法
// 包括 into(), into_iter(), into_response() 等

use std::collections::HashMap;

fn main() {
    println!("=== Rust Into 系列转换详解 ===\n");
    
    // 1. Into 基础
    demo_into_basics();
    
    // 2. into() - 所有权转换
    demo_into();
    
    // 3. into_iter() - 迭代器转换
    demo_into_iter();
    
    // 4. into_bytes() - 字节转换
    demo_into_bytes();
    
    // 5. into_boxed_slice() - Box 转换
    demo_into_boxed();
    
    // 6. into_string() - 字符串转换
    demo_into_string();
    
    // 7. into_inner() - 解包转换
    demo_into_inner();
    
    // 8. 自定义 Into
    demo_custom_into();
    
    // 9. From vs Into
    demo_from_vs_into();
    
    // 10. 实战案例
    demo_real_world_examples();
    
    println!("\n✅ 所有示例运行成功！");
}

// ============================================
// 1. Into 基础
// ============================================
fn demo_into_basics() {
    println!("--- 1. Into 基础 ---\n");
    
    println!("什么是 Into？");
    println!("  - 消费原值，转换为新类型");
    println!("  - 所有权转移");
    println!("  - 自动类型推导\n");
    
    println!("Into trait:");
    println!("  trait Into<T> {{");
    println!("      fn into(self) -> T;");
    println!("  }}");
    println!();
    
    println!("常用 into_ 方法:");
    println!("  📌 into()            - 通用转换");
    println!("  📌 into_iter()       - 转换为迭代器");
    println!("  📌 into_bytes()      - 转换为字节");
    println!("  📌 into_string()     - 转换为字符串");
    println!("  📌 into_inner()      - 解包内部值");
    println!("  📌 into_boxed_slice()- 转换为 Box<[T]>");
    println!("  📌 into_response()   - HTTP 响应转换");
    println!();
}

// ============================================
// 2. into() - 所有权转换
// ============================================
fn demo_into() {
    println!("--- 2. into() - 所有权转换 ---\n");
    
    println!("基础用法:");
    
    // String → &str（通过 as_str，不是 into）
    // &str → String
    let s: String = "hello".into();
    println!("  &str → String: {:?}", s);
    
    // i32 → i64
    let num: i64 = 42i32.into();
    println!("  i32 → i64: {}", num);
    
    // Vec<T> → Box<[T]>
    let vec = vec![1, 2, 3];
    let boxed: Box<[i32]> = vec.into();
    println!("  Vec → Box<[T]>: {:?}", boxed);
    println!();
    
    // 自动类型推导
    println!("自动类型推导:");
    
    fn takes_string(s: String) {
        println!("  接收 String: {}", s);
    }
    
    takes_string("hello".into()); // 自动推导为 String
    println!();
    
    // Option 和 Result
    println!("Option 和 Result:");
    
    let value: i32 = 42;
    let option: Option<i32> = Some(value);
    let result: Result<i32, String> = Ok(value);
    
    println!("  Option: {:?}", option);
    println!("  Result: {:?}", result);
    println!();
}

// ============================================
// 3. into_iter() - 迭代器转换
// ============================================
fn demo_into_iter() {
    println!("--- 3. into_iter() - 迭代器转换 ---\n");
    
    println!("什么是 into_iter？");
    println!("  - 消费集合，转换为拥有所有权的迭代器");
    println!("  - 移动集合中的元素");
    println!("  - 集合不可再使用\n");
    
    // Vec into_iter
    println!("Vec::into_iter():");
    
    let vec = vec![1, 2, 3, 4, 5];
    println!("  原始 Vec: {:?}", vec);
    
    for value in vec.into_iter() {
        println!("    值: {} (拥有所有权)", value);
    }
    // println!("{:?}", vec); // 错误！vec 已被移动
    println!();
    
    // iter() vs iter_mut() vs into_iter()
    println!("三种迭代方式对比:");
    
    let vec = vec![String::from("a"), String::from("b"), String::from("c")];
    
    // 1. iter() - 借用
    println!("  iter() - 借用:");
    for item in vec.iter() {
        println!("    &String: {}", item);
    }
    
    // 2. iter_mut() - 可变借用
    let mut vec = vec![String::from("a"), String::from("b")];
    println!("  iter_mut() - 可变借用:");
    for item in vec.iter_mut() {
        item.push_str("!");
        println!("    &mut String: {}", item);
    }
    
    // 3. into_iter() - 所有权转移
    println!("  into_iter() - 所有权:");
    for item in vec.into_iter() {
        println!("    String (owned): {}", item);
    }
    // println!("{:?}", vec); // 错误！已移动
    println!();
    
    // HashMap into_iter
    println!("HashMap::into_iter():");
    
    let mut map = HashMap::new();
    map.insert("name", "Alice");
    map.insert("age", "25");
    
    for (key, value) in map.into_iter() {
        println!("  {} = {}", key, value);
    }
    println!();
    
    // 数组 into_iter
    println!("Array::into_iter():");
    
    let arr = [10, 20, 30];
    for value in arr.into_iter() {
        println!("  {}", value);
    }
    println!();
}

// ============================================
// 4. into_bytes() - 字节转换
// ============================================
fn demo_into_bytes() {
    println!("--- 4. into_bytes() - 字节转换 ---\n");
    
    println!("String::into_bytes():");
    
    let s = String::from("Hello, 世界!");
    println!("  原始字符串: {}", s);
    
    let bytes = s.into_bytes();
    println!("  字节表示: {:?}", bytes);
    println!("  字节数量: {}", bytes.len());
    // println!("{}", s); // 错误！s 已被移动
    println!();
    
    // 从字节恢复
    println!("从字节恢复:");
    
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
    let s = String::from_utf8(bytes).unwrap();
    println!("  恢复字符串: {}", s);
    println!();
}

// ============================================
// 5. into_boxed_slice() - Box 转换
// ============================================
fn demo_into_boxed() {
    println!("--- 5. into_boxed_slice() - Box 转换 ---\n");
    
    println!("Vec::into_boxed_slice():");
    
    let vec = vec![1, 2, 3, 4, 5];
    println!("  Vec 容量: {}", vec.capacity());
    
    let boxed = vec.into_boxed_slice();
    println!("  Boxed slice: {:?}", boxed);
    println!("  说明: 释放多余容量，固定大小");
    println!();
    
    // 性能优化
    println!("性能优化示例:");
    
    let mut vec = Vec::with_capacity(100);
    vec.extend([1, 2, 3, 4, 5]);
    println!("  Vec 容量: {} (浪费空间)", vec.capacity());
    
    let boxed = vec.into_boxed_slice();
    println!("  Boxed 长度: {} (精确匹配)", boxed.len());
    println!();
}

// ============================================
// 6. into_string() - 字符串转换
// ============================================
fn demo_into_string() {
    println!("--- 6. into_string() - 字符串转换 ---\n");
    
    println!("OsString::into_string():");
    
    use std::ffi::OsString;
    
    let os_string = OsString::from("hello");
    match os_string.into_string() {
        Ok(s) => println!("  转换成功: {}", s),
        Err(os) => println!("  转换失败: {:?}", os),
    }
    println!();
    
    // PathBuf into
    println!("PathBuf 转换:");
    
    use std::path::PathBuf;
    
    let path = PathBuf::from("/usr/local/bin");
    let os_string: OsString = path.into_os_string();
    println!("  PathBuf → OsString: {:?}", os_string);
    println!();
}

// ============================================
// 7. into_inner() - 解包转换
// ============================================
fn demo_into_inner() {
    println!("--- 7. into_inner() - 解包转换 ---\n");
    
    println!("BufWriter::into_inner():");
    
    use std::io::{BufWriter, Write};
    
    let buffer = Vec::new();
    let mut writer = BufWriter::new(buffer);
    write!(writer, "Hello, World!").unwrap();
    
    // 提取内部 buffer
    let buffer = writer.into_inner().unwrap();
    println!("  内部 buffer: {:?}", buffer);
    println!("  字符串: {}", String::from_utf8_lossy(&buffer));
    println!();
    
    // Mutex into_inner
    println!("Mutex::into_inner():");
    
    use std::sync::Mutex;
    
    let mutex = Mutex::new(42);
    let value = mutex.into_inner().unwrap();
    println!("  解包的值: {}", value);
    println!();
    
    // Box into_inner (解引用)
    println!("Box 解包:");
    
    let boxed = Box::new(String::from("boxed"));
    let unboxed = *boxed; // 或使用 Box::into_inner (nightly)
    println!("  解包字符串: {}", unboxed);
    println!();
}

// ============================================
// 8. 自定义 Into
// ============================================
fn demo_custom_into() {
    println!("--- 8. 自定义 Into ---\n");
    
    println!("实现 From（自动获得 Into）:");
    
    #[derive(Debug)]
    struct Celsius(f64);
    
    #[derive(Debug)]
    struct Fahrenheit(f64);
    
    // 实现 From，自动获得 Into
    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Self {
            Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
        }
    }
    
    let celsius = Celsius(25.0);
    println!("  摄氏度: {:?}", celsius);
    
    // 使用 into() 转换
    let fahrenheit: Fahrenheit = celsius.into();
    println!("  华氏度: {:?}", fahrenheit);
    println!();
    
    // 复杂转换
    println!("复杂类型转换:");
    
    struct Person {
        name: String,
        age: u32,
    }
    
    struct Employee {
        name: String,
        age: u32,
        id: u32,
    }
    
    impl From<Person> for Employee {
        fn from(person: Person) -> Self {
            Employee {
                name: person.name,
                age: person.age,
                id: 0, // 默认 ID
            }
        }
    }
    
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let employee: Employee = person.into();
    println!("  员工: {} (ID: {})", employee.name, employee.id);
    println!();
}

// ============================================
// 9. From vs Into
// ============================================
fn demo_from_vs_into() {
    println!("--- 9. From vs Into ---\n");
    
    println!("From vs Into 对比:");
    println!("┌──────────┬────────────┬────────────┐");
    println!("│  特性     │  From      │  Into      │");
    println!("├──────────┼────────────┼────────────┤");
    println!("│  方向     │  明确      │  推导      │");
    println!("│  实现     │  手动      │  自动      │");
    println!("│  推荐     │  实现 From │  使用 Into │");
    println!("└──────────┴────────────┴────────────┘");
    println!();
    
    // 示例
    println!("示例:");
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    // 实现 From
    impl From<(i32, i32)> for Point {
        fn from(tuple: (i32, i32)) -> Self {
            Point {
                x: tuple.0,
                y: tuple.1,
            }
        }
    }
    
    // 使用 From
    let point = Point::from((10, 20));
    println!("  From: {:?}", point);
    
    // 使用 Into
    let point: Point = (30, 40).into();
    println!("  Into: {:?}", point);
    println!();
    
    println!("最佳实践:");
    println!("  ✓ 实现 From（自动获得 Into）");
    println!("  ✓ 使用 into() 更灵活（类型推导）");
    println!("  ✗ 不要同时实现 From 和 Into");
    println!();
}

// ============================================
// 10. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 10. 实战案例 ---\n");
    
    // 案例 1: 错误类型转换
    println!("案例 1: 错误类型转换\n");
    error_conversion_example();
    
    // 案例 2: 配置构建器
    println!("\n案例 2: 配置构建器\n");
    builder_pattern_example();
    
    // 案例 3: 数据处理管道
    println!("\n案例 3: 数据处理管道\n");
    pipeline_example();
}

// 案例 1: 错误类型转换
fn error_conversion_example() {
    use std::io;
    
    #[derive(Debug)]
    enum AppError {
        Io(io::Error),
        Parse(std::num::ParseIntError),
        Custom(String),
    }
    
    // 自动转换
    impl From<io::Error> for AppError {
        fn from(err: io::Error) -> Self {
            AppError::Io(err)
        }
    }
    
    impl From<std::num::ParseIntError> for AppError {
        fn from(err: std::num::ParseIntError) -> Self {
            AppError::Parse(err)
        }
    }
    
    fn process() -> Result<i32, AppError> {
        // io::Error 自动转换
        let _content = std::fs::read_to_string("test.txt")?;
        
        // ParseIntError 自动转换
        let num: i32 = "not-a-number".parse()?;
        
        Ok(num)
    }
    
    match process() {
        Err(e) => println!("  错误: {:?}", e),
        Ok(n) => println!("  成功: {}", n),
    }
}

// 案例 2: 配置构建器
fn builder_pattern_example() {
    struct Config {
        host: String,
        port: u16,
        timeout: u64,
    }
    
    struct ConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        timeout: Option<u64>,
    }
    
    impl ConfigBuilder {
        fn new() -> Self {
            ConfigBuilder {
                host: None,
                port: None,
                timeout: None,
            }
        }
        
        fn host<S: Into<String>>(mut self, host: S) -> Self {
            self.host = Some(host.into());
            self
        }
        
        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        
        fn build(self) -> Config {
            Config {
                host: self.host.unwrap_or_else(|| "localhost".to_string()),
                port: self.port.unwrap_or(8080),
                timeout: self.timeout.unwrap_or(30),
            }
        }
    }
    
    // 使用构建器
    let config = ConfigBuilder::new()
        .host("127.0.0.1")  // &str 自动转换为 String
        .port(3000)
        .build();
    
    println!("  配置: {}:{}", config.host, config.port);
}

// 案例 3: 数据处理管道
fn pipeline_example() {
    let data = vec!["1", "2", "3", "4", "5"];
    
    let result: Vec<i32> = data
        .into_iter()                    // into_iter
        .filter_map(|s| s.parse().ok()) // 转换并过滤
        .map(|n: i32| n * 2)            // 处理
        .collect();                      // 收集
    
    println!("  处理结果: {:?}", result);
}

/*
=== 总结 ===

1. Into 核心概念:

   特点:
   - 消费原值
   - 所有权转移
   - 自动类型推导
   - 零成本抽象

2. 常用 into_ 方法:

   通用:
   - into()            - 类型转换
   - into_iter()       - 迭代器
   
   字符串:
   - into_bytes()      - 字节数组
   - into_string()     - 字符串
   
   容器:
   - into_boxed_slice()- Box<[T]>
   - into_inner()      - 解包
   
   HTTP:
   - into_response()   - 响应转换

3. into_iter 对比:

   方法           类型        所有权
   iter()        &T          借用
   iter_mut()    &mut T      可变借用
   into_iter()   T           转移

4. From vs Into:

   From:
   - 实现 From trait
   - 明确类型转换
   - 自动获得 Into
   
   Into:
   - 类型推导
   - 使用更灵活
   - 不需要手动实现

5. 最佳实践:

   DO:
   ✓ 实现 From，获得 Into
   ✓ 使用 into() 简化代码
   ✓ 合理使用 into_iter()
   ✓ 利用类型推导
   
   DON'T:
   ✗ into 后使用原值
   ✗ 同时实现 From 和 Into
   ✗ 忽略所有权转移

运行示例:
  cargo run --bin into_conversions_detailed
*/
