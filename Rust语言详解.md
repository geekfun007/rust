# Rust 语言详解

## 目录
- [1. 命令行操作](#1-命令行操作)
- [2. 模块化](#2-模块化)
- [3. 语法基础](#3-语法基础)
- [4. 数据类型](#4-数据类型)
- [5. 逻辑控制](#5-逻辑控制)
- [6. 函数](#6-函数)
- [7. IO 操作](#7-io-操作)
- [8. HTTP 编程](#8-http-编程)
- [9. 异步编程](#9-异步编程)

---

## 1. 命令行操作

### 1.1 安装 Rust

```bash
# 安装 Rust（Unix-like 系统）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 更新 Rust
rustup update

# 查看版本
rustc --version
cargo --version
```

### 1.2 Cargo 基础命令

```bash
# 创建新项目
cargo new my_project         # 创建二进制项目
cargo new my_lib --lib       # 创建库项目

# 构建项目
cargo build                  # 调试版本
cargo build --release        # 发布版本

# 运行项目
cargo run                    # 运行项目
cargo run --release          # 运行发布版本

# 测试
cargo test                   # 运行测试
cargo test test_name         # 运行特定测试

# 文档
cargo doc                    # 生成文档
cargo doc --open             # 生成并打开文档

# 其他
cargo check                  # 快速检查代码（不生成可执行文件）
cargo clean                  # 清理构建产物
cargo fmt                    # 格式化代码
cargo clippy                 # 代码检查（需要安装）
```

### 1.3 项目结构

```
my_project/
├── Cargo.toml              # 项目配置文件
├── Cargo.lock              # 依赖锁定文件
├── src/
│   ├── main.rs             # 二进制项目入口
│   ├── lib.rs              # 库项目入口
│   └── bin/                # 额外的二进制文件
├── tests/                  # 集成测试
├── benches/                # 性能测试
└── examples/               # 示例代码
```

---

## 2. 模块化

### 2.1 模块基础

```rust
// 定义模块
mod network {
    pub fn connect() {
        println!("连接网络");
    }
    
    fn internal_function() {
        println!("内部函数");
    }
    
    // 嵌套模块
    pub mod server {
        pub fn start() {
            println!("启动服务器");
        }
    }
}

// 使用模块
fn main() {
    network::connect();
    network::server::start();
}
```

### 2.2 文件模块

```rust
// src/lib.rs 或 src/main.rs
mod utils;          // 引入 src/utils.rs
mod database;       // 引入 src/database.rs 或 src/database/mod.rs

// src/utils.rs
pub fn helper_function() {
    println!("辅助函数");
}

// src/database/mod.rs
pub mod connection;
pub mod query;
```

### 2.3 使用 use 关键字

```rust
use std::collections::HashMap;
use std::io::{self, Write};  // 导入多个
use std::fmt::Result;
use std::io::Result as IoResult;  // 别名

// 重新导出
pub use self::network::connect;

// glob 导入（不推荐过度使用）
use std::collections::*;
```

### 2.4 模块可见性

```rust
pub mod outer {
    pub fn public_function() {}
    
    fn private_function() {}
    
    pub mod inner {
        pub fn inner_public() {}
        
        pub(crate) fn crate_visible() {}      // 整个 crate 可见
        pub(super) fn parent_visible() {}      // 父模块可见
        pub(in crate::outer) fn outer_visible() {}  // 指定路径可见
    }
}
```

---

## 3. 语法基础

### 3.1 变量与可变性

```rust
fn main() {
    // 不可变变量（默认）
    let x = 5;
    // x = 6;  // 错误：不能修改不可变变量
    
    // 可变变量
    let mut y = 5;
    y = 6;  // 正确
    
    // 常量
    const MAX_POINTS: u32 = 100_000;
    
    // 遮蔽（Shadowing）
    let z = 5;
    let z = z + 1;  // 创建新变量
    let z = "字符串";  // 可以改变类型
}
```

### 3.2 数据类型转换

```rust
fn main() {
    // 显式类型转换
    let x: i32 = 42;
    let y: i64 = x as i64;
    let z: f64 = x as f64;
    
    // 字符串转换
    let s = "42";
    let num: i32 = s.parse().expect("不是数字");
    let num: i32 = s.parse().unwrap();
    
    // 使用 turbofish 语法
    let num = s.parse::<i32>().unwrap();
}
```

### 3.3 注释

```rust
// 单行注释

/*
 * 多行注释
 */

/// 文档注释（用于函数、结构体等）
/// 
/// # Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! 模块级文档注释
```

---

## 4. 数据类型

### 4.1 标量类型

```rust
fn main() {
    // 整数类型：i8, i16, i32, i64, i128, isize
    //          u8, u16, u32, u64, u128, usize
    let a: i32 = 42;
    let b: u64 = 100;
    let c = 0xff;        // 十六进制
    let d = 0o77;        // 八进制
    let e = 0b1111_0000; // 二进制
    let f = b'A';        // 字节（u8）
    
    // 浮点类型：f32, f64
    let x = 2.0;      // f64（默认）
    let y: f32 = 3.0;
    
    // 布尔类型
    let t = true;
    let f: bool = false;
    
    // 字符类型（4字节 Unicode）
    let c = 'z';
    let emoji = '😊';
    let chinese = '中';
}
```

### 4.2 复合类型

```rust
fn main() {
    // 元组
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    let (x, y, z) = tuple;  // 解构
    let first = tuple.0;     // 索引访问
    
    // 数组（固定长度）
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let first = array[0];
    let zeros = [0; 10];  // [0, 0, 0, ..., 0]
    
    // 切片（动态大小）
    let slice: &[i32] = &array[1..3];  // [2, 3]
}
```

### 4.3 字符串类型

```rust
fn main() {
    // String（可变，堆分配）
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push('!');
    
    // &str（不可变，字符串切片）
    let slice: &str = &s[0..5];
    let literal: &str = "hello";
    
    // 字符串操作
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 被移动
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    
    // 遍历字符串
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    for b in "hello".bytes() {
        println!("{}", b);
    }
}
```

### 4.4 集合类型

```rust
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // Vector（动态数组）
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    
    match v.get(2) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("没有第三个元素"),
    }
    
    for i in &v {
        println!("{}", i);
    }
    
    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // HashSet
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    // VecDeque（双端队列）
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_front(0);
}
```

### 4.5 结构体

```rust
// 经典结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 单元结构体
struct AlwaysEqual;

impl User {
    // 关联函数（构造器）
    fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
    
    // 方法
    fn is_active(&self) -> bool {
        self.active
    }
    
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user1 = User::new(
        String::from("user123"),
        String::from("user@example.com"),
    );
    
    user1.deactivate();
    
    // 结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

### 4.6 枚举

```rust
// 基础枚举
enum IpAddrKind {
    V4,
    V6,
}

// 带数据的枚举
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 复杂枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("颜色: ({}, {}, {})", r, g, b),
        }
    }
}

// Option<T> 枚举（标准库）
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Result<T, E> 枚举（标准库）
fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(numerator / denominator)
    }
}
```

---

## 5. 逻辑控制

### 5.1 条件语句

```rust
fn main() {
    let number = 6;
    
    // if 表达式
    if number < 5 {
        println!("条件为真");
    } else if number < 10 {
        println!("第二个条件为真");
    } else {
        println!("条件为假");
    }
    
    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    // match 表达式（模式匹配）
    let number = 3;
    match number {
        1 => println!("一"),
        2 | 3 => println!("二或三"),
        4..=10 => println!("四到十"),
        _ => println!("其他"),
    }
    
    // match 守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于五: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    
    // if let（简化的 match）
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("三");
    }
    
    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

### 5.2 循环

```rust
fn main() {
    // loop（无限循环）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // 返回值
        }
    };
    
    // 循环标签
    'outer: loop {
        loop {
            break 'outer;  // 跳出外层循环
        }
    }
    
    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("值是: {}", element);
    }
    
    // 范围
    for number in 1..4 {  // 1, 2, 3
        println!("{}", number);
    }
    
    for number in (1..=4).rev() {  // 4, 3, 2, 1
        println!("{}", number);
    }
    
    // 枚举索引
    let v = vec!["a", "b", "c"];
    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}
```

---

## 6. 函数

### 6.1 函数基础

```rust
// 基本函数
fn add(x: i32, y: i32) -> i32 {
    x + y  // 表达式返回值（无分号）
}

// 无返回值（返回 unit 类型 ()）
fn print_sum(x: i32, y: i32) {
    println!("和是: {}", x + y);
}

// 提前返回
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        return None;
    }
    Some(x / y)
}
```

### 6.2 所有权与借用

```rust
// 移动语义
fn take_ownership(s: String) {
    println!("{}", s);
}  // s 在这里被释放

// 不可变借用
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变借用
fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);
    // println!("{}", s1);  // 错误：s1 已被移动
    
    let s2 = String::from("hello");
    let len = calculate_length(&s2);
    println!("{} 的长度是 {}", s2, len);  // s2 仍然有效
    
    let mut s3 = String::from("hello");
    change(&mut s3);
    println!("{}", s3);
}
```

### 6.3 生命周期

```rust
// 显式生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

// 生命周期省略规则
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 'static 生命周期
let s: &'static str = "这个字符串拥有静态生命周期";
```

### 6.4 泛型

```rust
// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 泛型枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 6.5 Trait（特征）

```rust
// 定义 trait
pub trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn default_summary(&self) -> String {
        String::from("(阅读更多...)")
    }
}

// 实现 trait
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// trait 作为参数
fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}

// trait bound
fn notify_bound<T: Summary>(item: &T) {
    println!("突发新闻！{}", item.summarize());
}

// 多个 trait bound
fn notify_multiple<T: Summary + Display>(item: &T) {
    // ...
}

// where 子句
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
    0
}

// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Article {
        title: String::from("标题"),
        content: String::from("内容"),
    }
}

// trait 继承
trait OutlinePrint: Display {
    fn outline_print(&self) {
        println!("* {} *", self);
    }
}
```

### 6.6 闭包

```rust
fn main() {
    // 基本闭包
    let add = |x, y| x + y;
    println!("{}", add(1, 2));
    
    // 类型注解
    let add: fn(i32, i32) -> i32 = |x, y| x + y;
    
    // 捕获环境
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("{}", equal_to_x(4));
    
    // 移动闭包
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    
    // FnOnce（获取所有权）
    let consume = || {
        println!("{:?}", x);
    };
    
    // FnMut（可变借用）
    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(7);
    
    // Fn（不可变借用）
    let borrows = || println!("{:?}", list);
    
    // 作为参数
    fn apply<F>(f: F) where F: FnOnce() {
        f();
    }
}
```

### 6.7 迭代器

```rust
fn main() {
    let v = vec![1, 2, 3];
    
    // 基本迭代
    let v_iter = v.iter();
    for val in v_iter {
        println!("{}", val);
    }
    
    // map
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    
    // filter
    let v3: Vec<&i32> = v.iter().filter(|x| **x > 1).collect();
    
    // fold
    let sum: i32 = v.iter().fold(0, |acc, x| acc + x);
    
    // zip
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3: Vec<_> = v1.iter().zip(v2.iter()).collect();
    
    // chain
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3: Vec<_> = v1.iter().chain(v2.iter()).collect();
    
    // enumerate
    for (i, v) in vec![1, 2, 3].iter().enumerate() {
        println!("{}: {}", i, v);
    }
}

// 自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

## 7. IO 操作

### 7.1 标准输入输出

```rust
use std::io::{self, Write, BufRead};

fn main() {
    // 打印到标准输出
    println!("Hello, world!");
    print!("不换行 ");
    eprintln!("错误信息");  // 标准错误输出
    
    // 格式化输出
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
    println!("{0} 和 {1}，再次 {0}", x, y);
    println!("{first} {second}", first = x, second = y);
    
    // 读取标准输入
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");
    println!("你输入的是: {}", input);
    
    // 从缓冲区读取
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
```

### 7.2 文件操作

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // 读取文件（一次性读取）
    let contents = fs::read_to_string("file.txt")?;
    println!("{}", contents);
    
    // 读取为字节
    let bytes = fs::read("file.txt")?;
    
    // 写入文件
    fs::write("output.txt", "Hello, Rust!")?;
    
    // 使用 File
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // 创建文件并写入
    let mut file = File::create("output.txt")?;
    file.write_all(b"Hello, Rust!")?;
    
    // 追加模式
    let mut file = OpenOptions::new()
        .append(true)
        .open("output.txt")?;
    file.write_all(b"\nNew line")?;
    
    // 缓冲读取（高效）
    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    
    // 缓冲写入
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"Hello")?;
    writer.flush()?;
    
    // 文件元信息
    let metadata = fs::metadata("file.txt")?;
    println!("文件大小: {} bytes", metadata.len());
    println!("是否为目录: {}", metadata.is_dir());
    
    // 路径操作
    let path = Path::new("./some/path.txt");
    println!("文件名: {:?}", path.file_name());
    println!("扩展名: {:?}", path.extension());
    println!("父目录: {:?}", path.parent());
    
    // 目录操作
    fs::create_dir("new_dir")?;
    fs::create_dir_all("a/b/c")?;  // 递归创建
    fs::remove_dir("new_dir")?;
    fs::remove_dir_all("a")?;      // 递归删除
    
    // 遍历目录
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{:?}", entry.path());
    }
    
    // 复制和移动
    fs::copy("source.txt", "dest.txt")?;
    fs::rename("old.txt", "new.txt")?;
    
    Ok(())
}
```

### 7.3 错误处理

```rust
use std::fs::File;
use std::io::{self, Read};
use std::error::Error;

// Result 类型
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// 链式调用
fn read_username_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 自定义错误类型
use std::fmt;

#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO 错误: {}", e),
            MyError::ParseError(e) => write!(f, "解析错误: {}", e),
            MyError::Custom(s) => write!(f, "自定义错误: {}", s),
        }
    }
}

impl Error for MyError {}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

// 使用 Box<dyn Error>
fn do_something() -> Result<(), Box<dyn Error>> {
    let file = File::open("file.txt")?;
    let number: i32 = "42".parse()?;
    Ok(())
}
```

---

## 8. HTTP 编程

### 8.1 使用 reqwest（HTTP 客户端）

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET 请求
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?;
    
    println!("状态码: {}", response.status());
    let body = response.text().await?;
    println!("响应体: {}", body);
    
    // JSON 反序列化
    let post: Post = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json()
        .await?;
    println!("{:?}", post);
    
    // POST 请求
    let new_post = Post {
        id: 0,
        title: "新标题".to_string(),
        body: "新内容".to_string(),
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;
    
    println!("状态码: {}", response.status());
    
    // 设置请求头
    let response = client
        .get("https://api.example.com/data")
        .header("Authorization", "Bearer TOKEN")
        .header("User-Agent", "MyApp/1.0")
        .send()
        .await?;
    
    // 查询参数
    let response = client
        .get("https://api.example.com/search")
        .query(&[("q", "rust"), ("limit", "10")])
        .send()
        .await?;
    
    // 超时设置
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    Ok(())
}
```

### 8.2 使用 axum（HTTP 服务器）

```rust
use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<Vec<User>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(Mutex::new(vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
        ])),
    };
    
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("服务器运行在 http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.lock().await;
    Json(users.clone())
}

async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().await;
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(new_user): Json<User>,
) -> (StatusCode, Json<User>) {
    let mut users = state.users.lock().await;
    users.push(new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}

async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> StatusCode {
    let mut users = state.users.lock().await;
    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
```

---

## 9. 异步编程

### 9.1 async/await 基础

```rust
use tokio;

async fn say_hello() {
    println!("Hello");
}

async fn fetch_data() -> String {
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "数据".to_string()
}

#[tokio::main]
async fn main() {
    say_hello().await;
    
    let data = fetch_data().await;
    println!("{}", data);
    
    // 并发执行
    let (result1, result2) = tokio::join!(
        fetch_data(),
        fetch_data()
    );
    
    println!("{}, {}", result1, result2);
}
```

### 9.2 Future Trait

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = u32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}
```

### 9.3 tokio 运行时

```rust
use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    // 生成任务
    let handle = task::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "完成"
    });
    
    let result = handle.await.unwrap();
    println!("{}", result);
    
    // 生成阻塞任务
    let blocking = task::spawn_blocking(|| {
        // 执行 CPU 密集型任务
        std::thread::sleep(Duration::from_secs(1));
        "完成阻塞任务"
    });
    
    println!("{}", blocking.await.unwrap());
    
    // 超时
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        async {
            sleep(Duration::from_secs(3)).await;
            "完成"
        }
    ).await;
    
    match result {
        Ok(msg) => println!("{}", msg),
        Err(_) => println!("超时"),
    }
}
```

### 9.4 异步通道

```rust
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    // mpsc（多生产者单消费者）
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        tx.send("消息1").await.unwrap();
        tx.send("消息2").await.unwrap();
    });
    
    while let Some(msg) = rx.recv().await {
        println!("收到: {}", msg);
    }
    
    // oneshot（一次性通道）
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        tx.send("单次消息").unwrap();
    });
    
    let msg = rx.await.unwrap();
    println!("{}", msg);
}
```

### 9.5 异步流（Stream）

```rust
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    // 创建流
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    while let Some(value) = stream.next().await {
        println!("{}", value);
    }
    
    // map 和 filter
    let stream = stream::iter(vec![1, 2, 3, 4, 5])
        .map(|x| x * 2)
        .filter(|x| *x > 5);
    
    tokio::pin!(stream);
    
    while let Some(value) = stream.next().await {
        println!("{}", value);
    }
}
```

### 9.6 异步互斥锁

```rust
use tokio::sync::{Mutex, RwLock};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Mutex
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut num = data.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("结果: {}", *data.lock().await);
    
    // RwLock（读写锁）
    let data = Arc::new(RwLock::new(0));
    
    let data_clone = Arc::clone(&data);
    tokio::spawn(async move {
        let mut write = data_clone.write().await;
        *write += 1;
    });
    
    let read = data.read().await;
    println!("读取: {}", *read);
}
```
