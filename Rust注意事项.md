# Rust 编程注意事项与最佳实践

## 目录
- [1. 所有权系统陷阱](#1-所有权系统陷阱)
- [2. 借用检查器常见问题](#2-借用检查器常见问题)
- [3. 生命周期注意事项](#3-生命周期注意事项)
- [4. 错误处理最佳实践](#4-错误处理最佳实践)
- [5. 性能优化建议](#5-性能优化建议)
- [6. 并发编程陷阱](#6-并发编程陷阱)
- [7. 类型系统注意事项](#7-类型系统注意事项)
- [8. 异步编程最佳实践](#8-异步编程最佳实践)
- [9. 依赖管理](#9-依赖管理)
- [10. 测试与调试](#10-测试与调试)

---

## 1. 所有权系统陷阱

### 1.1 常见错误：值被移动后使用

```rust
// ❌ 错误示例
fn main() {
    let s = String::from("hello");
    take_ownership(s);
    println!("{}", s);  // 编译错误：s 已被移动
}

fn take_ownership(s: String) {
    println!("{}", s);
}

// ✅ 正确示例 1：使用引用
fn main() {
    let s = String::from("hello");
    borrow(&s);
    println!("{}", s);  // 正确：s 仍然有效
}

fn borrow(s: &String) {
    println!("{}", s);
}

// ✅ 正确示例 2：返回所有权
fn main() {
    let s = String::from("hello");
    let s = take_and_return(s);
    println!("{}", s);  // 正确
}

fn take_and_return(s: String) -> String {
    println!("{}", s);
    s
}

// ✅ 正确示例 3：使用 Clone
fn main() {
    let s = String::from("hello");
    take_ownership(s.clone());
    println!("{}", s);  // 正确：使用了克隆
}
```

### 1.2 部分移动陷阱

```rust
// ❌ 错误示例
struct Point {
    x: String,
    y: String,
}

fn main() {
    let p = Point {
        x: String::from("1"),
        y: String::from("2"),
    };
    
    let x = p.x;  // x 被移动
    println!("{}", p.y);  // 正确：y 仍可用
    println!("{:?}", p);  // 错误：p 部分被移动
}

// ✅ 正确示例：使用引用或 Clone
#[derive(Debug, Clone)]
struct Point {
    x: String,
    y: String,
}

fn main() {
    let p = Point {
        x: String::from("1"),
        y: String::from("2"),
    };
    
    let x = p.x.clone();
    println!("{:?}", p);  // 正确
}
```

### 1.3 循环中的所有权

```rust
// ❌ 错误示例
fn main() {
    let strings = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ];
    
    for s in strings {
        println!("{}", s);
    }
    
    println!("{:?}", strings);  // 错误：strings 被移动
}

// ✅ 正确示例：使用引用迭代
fn main() {
    let strings = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ];
    
    for s in &strings {  // 借用
        println!("{}", s);
    }
    
    println!("{:?}", strings);  // 正确
}
```

---

## 2. 借用检查器常见问题

### 2.1 多个可变借用

```rust
// ❌ 错误示例
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;  // 错误：不能有两个可变借用
    println!("{}, {}", r1, r2);
}

// ✅ 正确示例：作用域分离
fn main() {
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
        println!("{}", r1);
    }  // r1 离开作用域
    
    let r2 = &mut s;  // 正确
    println!("{}", r2);
}
```

### 2.2 可变借用与不可变借用共存

```rust
// ❌ 错误示例
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // 错误：已有不可变借用
    println!("{}, {}, {}", r1, r2, r3);
}

// ✅ 正确示例：NLL（非词法作用域生命周期）
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1 和 r2 不再使用
    
    let r3 = &mut s;  // 正确
    println!("{}", r3);
}
```

### 2.3 悬垂引用

```rust
// ❌ 错误示例
fn dangle() -> &String {  // 编译错误
    let s = String::from("hello");
    &s  // s 在函数结束时被释放
}

// ✅ 正确示例：返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 移动所有权
}
```

---

## 3. 生命周期注意事项

### 3.1 显式生命周期标注

```rust
// ❌ 错误示例
fn longest(x: &str, y: &str) -> &str {  // 编译错误：缺少生命周期
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ✅ 正确示例
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 使用示例
fn main() {
    let string1 = String::from("long string");
    let result;
    
    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str());
        // 错误：result 的生命周期不能超过 string2
    }
    
    // println!("{}", result);  // 编译错误
}
```

### 3.2 结构体生命周期

```rust
// ✅ 正确示例
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    // 生命周期省略规则适用
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
}
```

### 3.3 'static 生命周期的误用

```rust
// ❌ 不推荐：过度使用 'static
fn get_str() -> &'static str {
    // 只有字符串字面量才应该用 'static
    "hello"
}

// ❌ 错误：试图返回局部变量的 'static 引用
fn bad_static() -> &'static str {
    let s = String::from("hello");
    // &s  // 编译错误
    Box::leak(s.into_boxed_str())  // 可以但会泄漏内存！
}

// ✅ 正确：按需使用生命周期
fn get_name<'a>(input: &'a str) -> &'a str {
    input
}
```

---

## 4. 错误处理最佳实践

### 4.1 避免过度使用 unwrap 和 expect

```rust
// ❌ 不推荐：生产代码中使用 unwrap
fn read_config() -> String {
    std::fs::read_to_string("config.txt").unwrap()  // 可能 panic
}

// ✅ 推荐：使用 Result 类型
fn read_config() -> Result<String, std::io::Error> {
    std::fs::read_to_string("config.txt")
}

// ✅ 推荐：使用 ? 操作符
fn read_and_parse() -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("number.txt")?;
    let number: i32 = content.trim().parse()?;
    Ok(number)
}
```

### 4.2 自定义错误类型

```rust
// ✅ 推荐：使用 thiserror 或自定义错误
use std::fmt;

#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO 错误: {}", e),
            AppError::ParseError(e) => write!(f, "解析错误: {}", e),
            AppError::NotFound(msg) => write!(f, "未找到: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// From trait 实现
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

fn do_work() -> Result<i32, AppError> {
    let content = std::fs::read_to_string("data.txt")?;
    let number: i32 = content.trim().parse()?;
    Ok(number)
}
```

### 4.3 使用 anyhow 简化错误处理

```rust
// ✅ 推荐：应用程序中使用 anyhow
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .context(format!("无法读取文件: {}", path))?;
    
    // ...
    Ok(String::from("success"))
}
```

---

## 5. 性能优化建议

### 5.1 避免不必要的克隆

```rust
// ❌ 低效
fn process_strings(strings: Vec<String>) -> Vec<String> {
    strings.iter().map(|s| s.clone()).collect()  // 不必要的克隆
}

// ✅ 高效：使用引用
fn process_strings_ref(strings: &[String]) -> Vec<&str> {
    strings.iter().map(|s| s.as_str()).collect()
}

// ✅ 高效：转移所有权
fn process_strings_move(mut strings: Vec<String>) -> Vec<String> {
    strings.retain(|s| !s.is_empty());
    strings
}
```

### 5.2 预分配容量

```rust
// ❌ 低效：多次重新分配
fn build_vector() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);  // 可能多次重新分配
    }
    v
}

// ✅ 高效：预分配
fn build_vector_optimized() -> Vec<i32> {
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i);
    }
    v
}

// ✅ 更好：使用迭代器
fn build_vector_iter() -> Vec<i32> {
    (0..1000).collect()
}
```

### 5.3 使用迭代器而不是索引

```rust
// ❌ 低效
fn sum_vector(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..v.len() {
        sum += v[i];  // 每次都做边界检查
    }
    sum
}

// ✅ 高效：使用迭代器
fn sum_vector_iter(v: &[i32]) -> i32 {
    v.iter().sum()
}
```

### 5.4 选择合适的字符串类型

```rust
// String vs &str vs Cow

use std::borrow::Cow;

// ✅ 只读字符串：使用 &str
fn print_message(msg: &str) {
    println!("{}", msg);
}

// ✅ 需要修改：使用 String
fn modify_string(mut s: String) -> String {
    s.push_str(" modified");
    s
}

// ✅ 可能修改可能不修改：使用 Cow
fn maybe_modify<'a>(s: &'a str, should_modify: bool) -> Cow<'a, str> {
    if should_modify {
        Cow::Owned(format!("{} modified", s))
    } else {
        Cow::Borrowed(s)
    }
}
```

### 5.5 内联优化

```rust
// ✅ 小函数使用 #[inline]
#[inline]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// ✅ 强制内联
#[inline(always)]
fn critical_function() {
    // ...
}

// ✅ 禁止内联
#[inline(never)]
fn debug_function() {
    // ...
}
```

---

## 6. 并发编程陷阱

### 6.1 数据竞争预防

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// ❌ 错误：尝试在线程间共享可变引用
fn bad_concurrency() {
    let mut counter = 0;
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            // counter += 1;  // 编译错误
        });
        handles.push(handle);
    }
}

// ✅ 正确：使用 Arc<Mutex<T>>
fn good_concurrency() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("结果: {}", *counter.lock().unwrap());
}
```

### 6.2 避免死锁

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// ❌ 可能死锁
fn potential_deadlock() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));
    
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);
    
    let handle1 = thread::spawn(move || {
        let _a = lock1_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _b = lock2_clone.lock().unwrap();  // 可能死锁
    });
    
    let handle2 = thread::spawn(move || {
        let _b = lock2.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _a = lock1.lock().unwrap();  // 可能死锁
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}

// ✅ 正确：统一锁顺序
fn no_deadlock() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));
    
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);
    
    let handle1 = thread::spawn(move || {
        let _a = lock1_clone.lock().unwrap();
        let _b = lock2_clone.lock().unwrap();  // 统一顺序
    });
    
    let handle2 = thread::spawn(move || {
        let _a = lock1.lock().unwrap();        // 统一顺序
        let _b = lock2.lock().unwrap();
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### 6.3 使用读写锁

```rust
use std::sync::{Arc, RwLock};
use std::thread;

// ✅ 读多写少场景使用 RwLock
fn use_rwlock() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // 多个读线程
    for _ in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let d = data.read().unwrap();
            println!("{:?}", *d);
        });
        handles.push(handle);
    }
    
    // 一个写线程
    let data_write = Arc::clone(&data);
    let write_handle = thread::spawn(move || {
        let mut d = data_write.write().unwrap();
        d.push(4);
    });
    handles.push(write_handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

## 7. 类型系统注意事项

### 7.1 Sized trait

```rust
// ❌ 错误：未实现 Sized 的类型不能作为返回值
fn returns_str() -> str {  // 编译错误
    "hello"
}

// ✅ 正确：使用引用或 Box
fn returns_str_ref() -> &'static str {
    "hello"
}

fn returns_str_box() -> Box<str> {
    Box::from("hello")
}

// ✅ 正确：使用 ?Sized 标记
fn takes_unsized<T: ?Sized>(value: &T) {
    // ...
}
```

### 7.2 Copy vs Clone

```rust
// Copy 类型：可以通过简单的位复制
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// 不能实现 Copy（包含堆数据）
struct NotCopy {
    data: String,  // String 不是 Copy
}

// ✅ 正确使用
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // Copy，p1 仍然有效
    println!("{}, {}", p1.x, p2.x);
    
    let s1 = NotCopy { data: String::from("hello") };
    let s2 = s1.clone();  // 需要显式 Clone
    // let s3 = s1;  // Move，s1 不再有效
}
```

### 7.3 Deref 强制转换

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ✅ 利用 Deref 强制转换
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox(String::from("Rust"));
    hello(&m);  // &MyBox<String> -> &String -> &str
}
```

---

## 8. 异步编程最佳实践

### 8.1 避免阻塞异步运行时

```rust
use tokio;

// ❌ 错误：在异步函数中使用同步阻塞
async fn bad_async() {
    std::thread::sleep(std::time::Duration::from_secs(1));  // 阻塞整个运行时
}

// ✅ 正确：使用异步睡眠
async fn good_async() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

// ✅ 正确：CPU 密集型任务使用 spawn_blocking
async fn cpu_intensive() {
    tokio::task::spawn_blocking(|| {
        // 执行 CPU 密集型计算
        let mut sum = 0;
        for i in 0..1000000 {
            sum += i;
        }
        sum
    }).await.unwrap();
}
```

### 8.2 处理 Future 取消

```rust
use tokio::select;
use tokio::time::{sleep, Duration};

// ✅ 正确处理取消
async fn cancellable_task() {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    
    loop {
        select! {
            _ = interval.tick() => {
                println!("执行任务");
            }
            _ = tokio::signal::ctrl_c() => {
                println!("收到取消信号");
                break;
            }
        }
    }
}
```

### 8.3 避免 Send/Sync 问题

```rust
use std::rc::Rc;
use tokio;

// ❌ 错误：Rc 不是 Send
async fn bad_send() {
    let rc = Rc::new(5);
    tokio::spawn(async move {
        // println!("{}", rc);  // 编译错误：Rc 不是 Send
    });
}

// ✅ 正确：使用 Arc
use std::sync::Arc;

async fn good_send() {
    let arc = Arc::new(5);
    tokio::spawn(async move {
        println!("{}", arc);  // 正确
    });
}

// ✅ 正确：确保借用在 await 之前结束
async fn avoid_hold_across_await() {
    let mut data = vec![1, 2, 3];
    
    {
        let len = data.len();  // 借用
        println!("{}", len);
    }  // 借用结束
    
    some_async_fn().await;  // 正确
    
    // ❌ 错误写法
    // let len = &data.len();
    // some_async_fn().await;  // 错误：借用跨越 await
}

async fn some_async_fn() {}
```

---

## 9. 依赖管理

### 9.1 Cargo.toml 最佳实践

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"  # 使用最新稳定版
rust-version = "1.70"  # 最小 Rust 版本

[dependencies]
# 明确版本（生产环境推荐）
serde = "1.0.193"

# 使用特性标志
tokio = { version = "1.35", features = ["full"] }

# 可选依赖
tracing = { version = "0.1", optional = true }

[dev-dependencies]
# 测试依赖
criterion = "0.5"

[features]
default = ["std"]
std = []
logging = ["tracing"]

# 发布配置
[profile.release]
opt-level = 3
lto = true
codegen-units = 1

# 开发配置
[profile.dev]
opt-level = 0
debug = true
```

### 9.2 避免依赖地狱

```rust
// ✅ 使用 workspace 管理多个 crate
// Cargo.toml (workspace root)
[workspace]
members = [
    "crate-a",
    "crate-b",
    "crate-c",
]

[workspace.dependencies]
serde = "1.0"
tokio = { version = "1.35", features = ["full"] }

// crate-a/Cargo.toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

---

## 10. 测试与调试

### 10.1 编写有效的测试

```rust
// ✅ 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn test_subtraction() {
        assert_eq!(5 - 3, 2);
    }
    
    #[test]
    #[should_panic(expected = "除数不能为零")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }
    
    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 不等于 4"))
        }
    }
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除数不能为零");
    }
    a / b
}
```

### 10.2 集成测试

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    let result = my_crate::public_function();
    assert_eq!(result, expected_value);
}
```

### 10.3 调试技巧

```rust
// ✅ 使用 dbg! 宏
fn debug_example() {
    let x = 5;
    let y = dbg!(x * 2) + 1;  // 打印并返回值
    dbg!(y);
}

// ✅ 条件编译
#[cfg(debug_assertions)]
fn debug_only() {
    println!("这只在调试模式下运行");
}

// ✅ 自定义 Debug 实现
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}
```

---

## 常见编译错误及解决方案

### 错误 1：cannot borrow as mutable
```rust
// 原因：已有不可变借用
// 解决：分离借用作用域或使用 RefCell<T>
```

### 错误 2：lifetime may not live long enough
```rust
// 原因：生命周期不匹配
// 解决：添加正确的生命周期注解
```

### 错误 3：trait bound not satisfied
```rust
// 原因：类型未实现所需 trait
// 解决：为类型实现 trait 或使用 where 子句
```

### 错误 4：type annotations needed
```rust
// 原因：编译器无法推断类型
// 解决：添加显式类型注解
let x: i32 = value.parse().unwrap();
```

---

## 总结

### 核心原则
1. **优先使用借用而不是克隆**
2. **明确所有权转移**
3. **遵循单一责任原则**
4. **编写测试**
5. **处理所有错误情况**
6. **文档化公共 API**
7. **使用 clippy 检查代码质量**
8. **性能优化应基于性能分析**

### 开发流程建议
```bash
# 开发流程
cargo check       # 快速检查
cargo clippy      # 代码检查
cargo fmt         # 格式化
cargo test        # 运行测试
cargo build --release  # 发布构建
```

### 学习资源
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
