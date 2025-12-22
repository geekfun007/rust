// Rust 类型转换详解：.ok_()、.to_()、.as_() 系列方法
// 
// 本文件演示了 Rust 中常见的类型转换模式和方法

use std::collections::HashMap;

fn main() {
    println!("=== Rust 类型转换实战 ===\n");
    
    // 1. .ok() - Result -> Option 转换
    demo_ok_conversion();
    
    // 2. .ok_or() 和 .ok_or_else() - Option -> Result 转换
    demo_ok_or_conversion();
    
    // 3. .to_*() 系列 - 创建新的拥有所有权的值
    demo_to_conversions();
    
    // 4. .as_*() 系列 - 引用转换（不创建新值）
    demo_as_conversions();
    
    // 5. .into() - 消费原值并转换
    demo_into_conversions();
    
    // 6. 实战案例
    demo_real_world_examples();
}

// ============================================
// 1. .ok() - 将 Result<T, E> 转换为 Option<T>
// ============================================
fn demo_ok_conversion() {
    println!("--- 1. .ok() 方法 ---");
    
    // 场景：当你不关心错误的具体信息，只关心是否成功
    let result: Result<i32, &str> = Ok(42);
    let option = result.ok(); // Some(42)
    println!("Ok(42).ok() = {:?}", option);
    
    let result: Result<i32, &str> = Err("出错了");
    let option = result.ok(); // None
    println!("Err('出错了').ok() = {:?}", option);
    
    // 实际应用：解析字符串
    let numbers: Vec<i32> = vec!["1", "2", "abc", "4", "xyz"]
        .iter()
        .filter_map(|s| s.parse::<i32>().ok()) // 只保留成功解析的
        .collect();
    println!("成功解析的数字: {:?}\n", numbers);
}

// ============================================
// 2. .ok_or() 和 .ok_or_else() - Option -> Result
// ============================================
fn demo_ok_or_conversion() {
    println!("--- 2. .ok_or() 和 .ok_or_else() ---");
    
    // .ok_or() - 将 None 转换为指定的错误
    let some_value: Option<i32> = Some(100);
    let result = some_value.ok_or("没有值");
    println!("Some(100).ok_or('没有值') = {:?}", result);
    
    let none_value: Option<i32> = None;
    let result = none_value.ok_or("没有值");
    println!("None.ok_or('没有值') = {:?}", result);
    
    // .ok_or_else() - 惰性求值，只在 None 时才计算错误
    let config: HashMap<String, String> = HashMap::new();
    let port = config
        .get("port")
        .ok_or_else(|| format!("配置文件缺少 'port' 字段"));
    println!("获取配置 port: {:?}", port);
    
    // 实际应用：从 HashMap 获取必需的值
    let mut settings = HashMap::new();
    settings.insert("host", "localhost");
    settings.insert("port", "8080");
    
    // 使用闭包来处理可能的错误
    let _ = (|| -> Result<(), &str> {
        let host = settings.get("host").ok_or("缺少 host 配置")?;
        let port = settings.get("port").ok_or("缺少 port 配置")?;
        println!("服务器配置: {}:{}", host, port);
        Ok(())
    })();
    
    println!();
}

// ============================================
// 3. .to_*() 系列 - 创建新的拥有所有权的副本
// ============================================
fn demo_to_conversions() {
    println!("--- 3. .to_*() 系列（创建新值） ---");
    
    // .to_string() - 转换为 String（拥有所有权）
    let s: &str = "hello";
    let owned: String = s.to_string();
    println!("&str -> String: {:?}", owned);
    
    let num = 42;
    let num_string = num.to_string();
    println!("i32 -> String: {:?}", num_string);
    
    // .to_owned() - 从借用创建拥有所有权的副本
    let slice: &str = "world";
    let owned: String = slice.to_owned();
    println!("&str -> String (to_owned): {:?}", owned);
    
    let vec_slice: &[i32] = &[1, 2, 3];
    let vec_owned: Vec<i32> = vec_slice.to_owned();
    println!("&[i32] -> Vec<i32>: {:?}", vec_owned);
    
    // .to_vec() - 从切片创建 Vec
    let arr = [1, 2, 3, 4, 5];
    let vec = arr.to_vec();
    println!("数组 -> Vec: {:?}", vec);
    
    // to_lowercase() / to_uppercase()
    let text = "Hello World";
    println!("小写: {}", text.to_lowercase());
    println!("大写: {}\n", text.to_uppercase());
}

// ============================================
// 4. .as_*() 系列 - 引用转换（零成本）
// ============================================
fn demo_as_conversions() {
    println!("--- 4. .as_*() 系列（引用转换） ---");
    
    // .as_str() - String -> &str
    let string = String::from("hello");
    let str_ref: &str = string.as_str();
    println!("String.as_str(): {:?}", str_ref);
    
    // .as_bytes() - 字符串 -> 字节切片
    let text = "Hello";
    let bytes: &[u8] = text.as_bytes();
    println!("'Hello'.as_bytes(): {:?}", bytes);
    
    // .as_ref() - 通用引用转换
    let string = String::from("world");
    let str_ref: &str = string.as_ref();
    println!("String.as_ref(): {:?}", str_ref);
    
    // Option<T> 的 .as_ref() - 避免移动所有权
    let opt_string: Option<String> = Some(String::from("test"));
    let opt_ref: Option<&String> = opt_string.as_ref(); // 不移动 String
    println!("Option<String>.as_ref(): {:?}", opt_ref);
    println!("原始值仍可用: {:?}", opt_string); // 所有权没有被移动
    
    // Result<T, E> 的 .as_ref()
    let result: Result<String, String> = Ok(String::from("success"));
    let result_ref: Result<&String, &String> = result.as_ref();
    println!("Result.as_ref(): {:?}", result_ref);
    
    // .as_mut() - 可变引用转换
    let mut vec = vec![1, 2, 3];
    let slice: &mut [i32] = vec.as_mut_slice();
    slice[0] = 100;
    println!("修改后的 Vec: {:?}\n", vec);
}

// ============================================
// 5. .into() - 消费原值并转换（移动所有权）
// ============================================
fn demo_into_conversions() {
    println!("--- 5. .into() 转换 ---");
    
    // String <-> &str
    let s: &str = "hello";
    let string: String = s.into(); // &str -> String
    println!("&str.into() -> String: {:?}", string);
    // 注意：string 的所有权已经被移动，不能再使用
    
    // Vec <-> 数组
    let arr = [1, 2, 3];
    let vec: Vec<i32> = arr.into(); // [i32; 3] -> Vec<i32>
    println!("数组.into() -> Vec: {:?}", vec);
    
    // 自动类型推导
    fn accept_string(s: String) {
        println!("接收到: {}", s);
    }
    accept_string("hello".into()); // 编译器自动推导为 String
    
    // Result/Option 转换
    let opt: Option<i32> = Some(42);
    let result: Result<i32, ()> = opt.ok_or(()); // 手动转换
    println!("Option -> Result: {:?}\n", result);
}

// ============================================
// 6. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 6. 实战案例 ---");
    
    // 案例 1：配置文件解析
    println!("案例 1: 配置解析");
    parse_config_example();
    
    // 案例 2：数据处理管道
    println!("\n案例 2: 数据处理管道");
    data_pipeline_example();
    
    // 案例 3：API 响应处理
    println!("\n案例 3: API 响应处理");
    api_response_example();
}

// 案例 1：配置文件解析
fn parse_config_example() {
    let config_data = vec![
        ("host", "localhost"),
        ("port", "8080"),
        ("timeout", "30"),
    ];
    
    let config: HashMap<&str, &str> = config_data.into_iter().collect();
    
    // 组合使用 .get() + .ok_or() + .parse() + .ok()
    let host = config.get("host").ok_or("缺少 host").unwrap();
    let port: u16 = config
        .get("port")
        .ok_or("缺少 port")
        .and_then(|s| s.parse().map_err(|_| "port 格式错误"))
        .unwrap();
    let timeout: u64 = config
        .get("timeout")
        .and_then(|s| s.parse().ok())
        .unwrap_or(60); // 默认值
    
    println!("  配置: {}:{}, 超时: {}s", host, port, timeout);
}

// 案例 2：数据处理管道
fn data_pipeline_example() {
    let data = vec!["1", "2", "abc", "4", "5.5", "6"];
    
    // 链式调用：解析 -> 过滤 -> 转换
    let result: Vec<i32> = data
        .iter()
        .filter_map(|s| s.parse::<f64>().ok()) // 解析为浮点数
        .filter(|&n| n >= 1.0 && n <= 10.0) // 过滤范围
        .map(|n| n as i32) // 转换为整数
        .collect();
    
    println!("  处理结果: {:?}", result);
}

// 案例 3：API 响应处理
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn api_response_example() {
    // 模拟 API 响应
    let response: Result<Option<User>, String> = Ok(Some(User {
        name: "张三".to_string(),
        age: 25,
    }));
    
    // 链式处理：Result -> Option -> 提取值
    match &response {
        Ok(Some(user)) => {
            println!("  用户: {} ({}岁)", user.name, user.age);
        }
        Ok(None) => println!("  用户不存在"),
        Err(e) => println!("  错误: {}", e),
    }
    
    // 使用 .ok() 和 .ok_or()
    let user = response
        .ok() // Result -> Option
        .flatten() // Option<Option<User>> -> Option<User>
        .ok_or("获取用户失败");
    
    println!("  提取结果: {:?}", user);
}
