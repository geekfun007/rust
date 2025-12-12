# Rust 编程详解与实战

全面的 Rust 编程指南，包含基础概念、核心特性和实战技巧。

## 目录

1. [基础类型与集合](#1-基础类型与集合)
2. [所有权、借用与生命周期](#2-所有权借用与生命周期)
3. [Trait 与泛型](#3-trait-与泛型)
4. [错误处理](#4-错误处理)
5. [并发编程](#5-并发编程)
6. [IO 与网络](#6-io-与网络)
7. [进程管理](#7-进程管理)
8. [注意事项与高级技巧](#8-注意事项与高级技巧)

---

## 1. 基础类型与集合

### 1.1 整数类型 (int)

Rust 提供了多种整数类型，分为有符号和无符号：

```rust
// 有符号整数: i8, i16, i32, i64, i128, isize
let signed: i32 = -42;
let big_signed: i64 = -9223372036854775808;

// 无符号整数: u8, u16, u32, u64, u128, usize
let unsigned: u32 = 42;
let byte: u8 = 255;

// isize 和 usize 根据架构决定（32位或64位）
let index: usize = 0;
```

**常用方法：**

```rust
let num: i32 = 42;

// 数学运算
println!("{}", num.abs());           // 绝对值: 42
println!("{}", num.pow(2));          // 幂运算: 1764
println!("{}", num.checked_add(10)); // 检查溢出的加法: Some(52)

// 类型转换
let as_float = num as f64;           // 转换为浮点数
let as_string = num.to_string();     // 转换为字符串

// 位运算
println!("{}", num.count_ones());    // 统计1的个数
println!("{}", num.leading_zeros()); // 前导零个数
println!("{}", num.rotate_left(2));  // 循环左移
```

### 1.2 浮点数类型 (float)

```rust
// f32 和 f64（默认）
let x: f64 = 3.14159265359;
let y: f32 = 2.71828;

// 常用方法
println!("{}", x.floor());           // 向下取整: 3.0
println!("{}", x.ceil());            // 向上取整: 4.0
println!("{}", x.round());           // 四舍五入: 3.0
println!("{}", x.abs());             // 绝对值
println!("{}", x.sqrt());            // 平方根
println!("{}", x.powi(2));           // 整数次幂
println!("{}", x.powf(2.5));         // 浮点次幂
println!("{}", x.sin());             // 三角函数

// 检查特殊值
println!("{}", x.is_nan());          // 是否为 NaN
println!("{}", x.is_infinite());     // 是否为无穷
println!("{}", x.is_finite());       // 是否为有限数
```

### 1.3 布尔类型 (bool)

```rust
let is_true: bool = true;
let is_false: bool = false;

// 逻辑运算
let and_result = is_true && is_false;  // false
let or_result = is_true || is_false;   // true
let not_result = !is_true;             // false

// 条件判断
if is_true {
    println!("It's true!");
}

// 布尔转换
let as_int = is_true as i32;           // 1
let as_string = is_true.to_string();   // "true"
```

### 1.4 字符串类型 (str vs String)

**`&str` - 字符串切片（不可变，固定大小）**

```rust
// 字符串字面量
let s: &str = "Hello, world!";

// 常用方法
println!("{}", s.len());                // 字节长度: 13
println!("{}", s.is_empty());           // 是否为空: false
println!("{}", s.starts_with("Hello")); // true
println!("{}", s.ends_with("!"));       // true
println!("{}", s.contains("world"));    // true

// 字符串切片
let slice = &s[0..5];                   // "Hello"
let slice = &s[7..];                    // "world!"

// 分割与迭代
for word in s.split_whitespace() {
    println!("{}", word);
}

// 模式匹配
let parts: Vec<&str> = s.split(',').collect();
```

**`String` - 可变字符串（堆分配）**

```rust
// 创建 String
let mut s = String::from("Hello");
let s2 = "World".to_string();
let s3 = String::new();

// 修改字符串
s.push_str(", World");                  // 追加字符串
s.push('!');                            // 追加字符
s.insert(5, ',');                       // 插入字符
s.insert_str(7, "Rust ");               // 插入字符串

// 删除操作
s.pop();                                // 删除最后一个字符
s.remove(0);                            // 删除指定位置字符
s.truncate(5);                          // 截断到指定长度
s.clear();                              // 清空

// 字符串拼接
let s1 = String::from("Hello");
let s2 = String::from("World");
let s3 = s1 + " " + &s2;               // s1 被移动
let s4 = format!("{} {}", "Hello", "World"); // 更灵活

// 替换
let new_s = s.replace("old", "new");
let new_s = s.replacen("old", "new", 1); // 替换前n个

// 大小写转换
println!("{}", s.to_uppercase());
println!("{}", s.to_lowercase());

// 去除空白
println!("{}", s.trim());
println!("{}", s.trim_start());
println!("{}", s.trim_end());
```

### 1.5 元组 (tuple)

```rust
// 创建元组
let tuple: (i32, f64, &str) = (42, 3.14, "hello");

// 访问元素
let first = tuple.0;
let second = tuple.1;
let third = tuple.2;

// 解构
let (x, y, z) = tuple;
println!("{}, {}, {}", x, y, z);

// 单元素元组（注意逗号）
let single = (42,);

// 嵌套元组
let nested = ((1, 2), (3, 4));

// 元组作为返回值
fn get_coords() -> (i32, i32) {
    (10, 20)
}

let (x, y) = get_coords();
```

### 1.6 结构体 (struct)

**命名结构体：**

```rust
// 定义结构体
struct Person {
    name: String,
    age: u32,
    email: String,
}

// 创建实例
let person = Person {
    name: String::from("Alice"),
    age: 30,
    email: String::from("alice@example.com"),
};

// 访问字段
println!("{}", person.name);

// 可变实例
let mut person = Person { /* ... */ };
person.age = 31;

// 字段初始化简写
let name = String::from("Bob");
let age = 25;
let person = Person {
    name,  // 等同于 name: name
    age,
    email: String::from("bob@example.com"),
};

// 结构体更新语法
let person2 = Person {
    name: String::from("Charlie"),
    ..person  // 其余字段从 person 复制
};
```

**元组结构体：**

```rust
struct Point(i32, i32);
struct Color(u8, u8, u8);

let origin = Point(0, 0);
let black = Color(0, 0, 0);

println!("{}, {}", origin.0, origin.1);
```

**单元结构体：**

```rust
struct Unit;
let unit = Unit;
```

### 1.7 数组 (array) vs 向量 (Vec)

**数组 (array) - 固定大小：**

```rust
// 创建数组
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];  // [0, 0, 0, ..., 0]

// 访问元素
let first = arr[0];
let last = arr[arr.len() - 1];

// 遍历
for elem in &arr {
    println!("{}", elem);
}

for (index, elem) in arr.iter().enumerate() {
    println!("{}: {}", index, elem);
}

// 数组切片
let slice: &[i32] = &arr[1..4];  // [2, 3, 4]

// 常用方法
println!("{}", arr.len());
println!("{:?}", arr);
arr.reverse();  // 需要可变引用
```

**向量 (Vec) - 动态大小：**

```rust
// 创建 Vec
let mut vec: Vec<i32> = Vec::new();
let mut vec = vec![1, 2, 3, 4, 5];
let vec = vec![0; 10];  // 10个0

// 添加元素
vec.push(6);
vec.push(7);
vec.insert(0, 0);  // 在索引0处插入

// 删除元素
let last = vec.pop();           // 删除并返回最后一个
let removed = vec.remove(2);    // 删除并返回指定索引
vec.clear();                    // 清空

// 访问元素
let first = vec[0];
let second = vec.get(1);  // 返回 Option<&T>

// 遍历
for elem in &vec {
    println!("{}", elem);
}

for elem in &mut vec {
    *elem *= 2;  // 修改元素
}

// 常用方法
println!("{}", vec.len());
println!("{}", vec.capacity());
println!("{}", vec.is_empty());
vec.sort();
vec.reverse();
vec.dedup();  // 去重（需要先排序）

// 切片操作
let slice = &vec[1..4];

// 连接
vec.extend([7, 8, 9]);
vec.append(&mut other_vec);

// 转换
let array: [i32; 3] = vec[0..3].try_into().unwrap();
```

### 1.8 枚举 (enum)

```rust
// 简单枚举
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;

// 带数据的枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("hello"));

// Option 枚举（标准库）
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;

// Result 枚举（标准库）
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 枚举方法
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
}
```

### 1.9 实现 (impl)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

// 关联函数和方法
impl Rectangle {
    // 关联函数（类似静态方法）
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // 方法（需要 self）
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 可变方法
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// 使用
let rect = Rectangle::new(30, 50);
let square = Rectangle::square(20);
println!("Area: {}", rect.area());

let mut rect = Rectangle::new(10, 10);
rect.scale(2);
```

### 1.10 控制流

**if-else 语句：**

```rust
let number = 6;

if number % 4 == 0 {
    println!("divisible by 4");
} else if number % 3 == 0 {
    println!("divisible by 3");
} else if number % 2 == 0 {
    println!("divisible by 2");
} else {
    println!("not divisible by 4, 3, or 2");
}

// if 作为表达式
let condition = true;
let number = if condition { 5 } else { 6 };
```

**match 表达式：**

```rust
let number = 3;

match number {
    1 => println!("One"),
    2 | 3 => println!("Two or Three"),
    4..=10 => println!("Four to Ten"),
    _ => println!("Something else"),
}

// match 解构
let point = (0, 5);
match point {
    (0, 0) => println!("Origin"),
    (0, y) => println!("On y-axis at {}", y),
    (x, 0) => println!("On x-axis at {}", x),
    (x, y) => println!("At ({}, {})", x, y),
}

// match 守卫
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

**for 循环：**

```rust
// 遍历范围
for i in 0..10 {
    println!("{}", i);
}

for i in 0..=10 {  // 包含10
    println!("{}", i);
}

// 遍历集合
let vec = vec![1, 2, 3, 4, 5];
for item in &vec {
    println!("{}", item);
}

for item in &mut vec {
    *item *= 2;
}

// enumerate
for (index, value) in vec.iter().enumerate() {
    println!("{}: {}", index, value);
}
```

**while 循环：**

```rust
let mut number = 3;

while number != 0 {
    println!("{}", number);
    number -= 1;
}

// while let
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

**loop 循环：**

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    
    if counter == 10 {
        break counter * 2;
    }
};

// 循环标签
'outer: loop {
    loop {
        break 'outer;
    }
}
```

### 1.11 函数 (fn)

```rust
// 基本函数
fn add(x: i32, y: i32) -> i32 {
    x + y  // 隐式返回（无分号）
}

// 多个返回值（使用元组）
fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

// 引用参数
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变引用
fn change(s: &mut String) {
    s.push_str(", world");
}

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

// 闭包
let add_one = |x: i32| x + 1;
let result = add_one(5);

let multiply = |x: i32, y: i32| x * y;

// 闭包捕获环境
let x = 4;
let equal_to_x = |z| z == x;
println!("{}", equal_to_x(4));
```

### 1.12 异步函数 (async/await)

```rust
use tokio;

// 异步函数
async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

// 调用异步函数
#[tokio::main]
async fn main() {
    let result = fetch_data("https://api.example.com").await;
    
    match result {
        Ok(data) => println!("Data: {}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// 并发执行
async fn concurrent_tasks() {
    let task1 = fetch_data("https://api1.example.com");
    let task2 = fetch_data("https://api2.example.com");
    
    let (result1, result2) = tokio::join!(task1, task2);
}

// 超时控制
use tokio::time::{timeout, Duration};

async fn with_timeout() {
    let result = timeout(
        Duration::from_secs(5),
        fetch_data("https://api.example.com")
    ).await;
    
    match result {
        Ok(Ok(data)) => println!("Got data: {}", data),
        Ok(Err(e)) => println!("Request error: {}", e),
        Err(_) => println!("Timeout!"),
    }
}
```

### 1.13 日期时间 (chrono)

```rust
use chrono::{DateTime, Utc, Local, NaiveDate, Duration};

// 当前时间
let now = Utc::now();
let local_now = Local::now();

println!("UTC: {}", now);
println!("Local: {}", local_now);

// 解析日期时间
let dt = DateTime::parse_from_rfc3339("2024-12-12T10:30:00+00:00").unwrap();
let dt = DateTime::parse_from_str("2024-12-12 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();

// 创建日期
let date = NaiveDate::from_ymd_opt(2024, 12, 12).unwrap();

// 格式化
println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
println!("{}", now.format("%Y年%m月%d日"));

// 时间运算
let tomorrow = now + Duration::days(1);
let one_hour_ago = now - Duration::hours(1);

// 时间戳
let timestamp = now.timestamp();
let timestamp_millis = now.timestamp_millis();

// 从时间戳创建
let dt = DateTime::from_timestamp(timestamp, 0).unwrap();
```

### 1.14 正则表达式 (regex)

```rust
use regex::Regex;

// 创建正则表达式
let re = Regex::new(r"\d{3}-\d{3}-\d{4}").unwrap();

// 匹配检查
let text = "My phone is 123-456-7890";
if re.is_match(text) {
    println!("Found a phone number!");
}

// 查找匹配
if let Some(mat) = re.find(text) {
    println!("Found: {}", mat.as_str());
}

// 查找所有匹配
let re = Regex::new(r"\d+").unwrap();
let text = "There are 123 apples and 456 oranges";
for mat in re.find_iter(text) {
    println!("Number: {}", mat.as_str());
}

// 捕获组
let re = Regex::new(r"(\d{3})-(\d{3})-(\d{4})").unwrap();
let text = "123-456-7890";

if let Some(caps) = re.captures(text) {
    println!("Area code: {}", &caps[1]);
    println!("Exchange: {}", &caps[2]);
    println!("Number: {}", &caps[3]);
}

// 替换
let re = Regex::new(r"\d+").unwrap();
let text = "Price: 100 dollars";
let result = re.replace(text, "XXX");
println!("{}", result);  // "Price: XXX dollars"

let result = re.replace_all(text, "XXX");
```

---

## 2. 所有权、借用与生命周期

### 2.1 所有权 (Ownership)

Rust 的核心特性，确保内存安全而无需垃圾回收。

**所有权规则：**
1. 每个值都有一个所有者
2. 一次只能有一个所有者
3. 当所有者离开作用域时，值被丢弃

```rust
// 移动语义
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权移动到 s2
// println!("{}", s1);  // 错误！s1 不再有效

// 克隆
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝
println!("{}, {}", s1, s2);  // 正常

// Copy trait（栈上数据）
let x = 5;
let y = x;  // x 被复制，而不是移动
println!("{}, {}", x, y);  // 正常

// 函数与所有权
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s 在这里被丢弃

fn makes_copy(x: i32) {
    println!("{}", x);
}

let s = String::from("hello");
takes_ownership(s);  // s 的所有权移入函数
// println!("{}", s);  // 错误！

let x = 5;
makes_copy(x);  // x 被复制
println!("{}", x);  // 正常

// 返回值与所有权
fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

let s1 = gives_ownership();
let s2 = String::from("hello");
let s3 = takes_and_gives_back(s2);
```

### 2.2 借用 (Borrowing)

引用允许使用值而不获取所有权。

**不可变借用：**

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 离开作用域，但不丢弃数据

let s1 = String::from("hello");
let len = calculate_length(&s1);  // 借用 s1
println!("Length of '{}' is {}.", s1, len);

// 多个不可变借用
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &s;
println!("{}, {}, {}", r1, r2, r3);  // 正常
```

**可变借用：**

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
println!("{}", s);

// 限制：同一时间只能有一个可变借用
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s;  // 错误！
println!("{}", r1);

// 不可变借用和可变借用不能同时存在
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
// let r3 = &mut s;  // 错误！
println!("{}, {}", r1, r2);

// 引用的作用域
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);
// r1 和 r2 的作用域在这里结束

let r3 = &mut s;  // 正常
println!("{}", r3);
```

**悬垂引用（编译器防止）：**

```rust
// 编译错误
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}  // s 被丢弃，返回的引用无效

// 正确做法
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 移动所有权
}
```

### 2.3 生命周期 (Lifetime)

生命周期确保引用在需要时始终有效。

**生命周期注解语法：**

```rust
// 函数签名中的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 使用
let string1 = String::from("long string");
let string2 = String::from("short");
let result = longest(string1.as_str(), string2.as_str());
println!("Longest: {}", result);

// 生命周期作用域
let string1 = String::from("long string");
let result;
{
    let string2 = String::from("short");
    result = longest(string1.as_str(), string2.as_str());
    // string2 在这里结束
}
// println!("{}", result);  // 错误！result 引用了 string2
```

**结构体中的生命周期：**

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

// 使用
let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().unwrap();
let excerpt = ImportantExcerpt {
    part: first_sentence,
};
```

**生命周期省略规则：**

```rust
// 规则1：每个引用参数都有自己的生命周期
fn foo(x: &str) -> &str { x }
// 等同于
fn foo<'a>(x: &'a str) -> &'a str { x }

// 规则2：如果只有一个输入生命周期，赋给所有输出
fn first_word(s: &str) -> &str {
    &s[..1]
}

// 规则3：如果有多个输入生命周期，且其中一个是 &self 或 &mut self，
// self 的生命周期赋给所有输出
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self) -> &str {
        self.part
    }
}
```

**静态生命周期：**

```rust
// 'static 表示整个程序运行期间
let s: &'static str = "I have a static lifetime.";

// 字符串字面量都是 'static
const GREETING: &str = "Hello, world!";
```

---

## 3. Trait 与泛型

### 3.1 Trait 定义与实现

Trait 类似于其他语言中的接口。

```rust
// 定义 trait
trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// 为类型实现 trait
struct Article {
    title: String,
    content: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// 使用
let article = Article {
    title: String::from("Rust Programming"),
    content: String::from("Rust is awesome!"),
    author: String::from("Alice"),
};

println!("{}", article.summarize());
println!("{}", article.default_summary());
```

### 3.2 Trait 作为参数

```rust
// trait bound 语法
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 完整语法
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 多个 trait bound
fn notify<T: Summary + Display>(item: &T) {
    // ...
}

// where 子句（更清晰）
fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

### 3.3 返回实现 Trait 的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user"),
        content: String::from("content"),
    }
}

// 注意：只能返回单一类型
// 以下代码无法编译
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Article { /* ... */ }  // 错误！
    } else {
        Tweet { /* ... */ }
    }
}
```

### 3.4 常用标准库 Trait

**Debug 和 Display：**

```rust
use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
println!("{:?}", p);   // Debug
println!("{}", p);     // Display
```

**Clone 和 Copy：**

```rust
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();  // 显式克隆
let p3 = p1;          // Copy（自动复制）
```

**PartialEq 和 Eq：**

```rust
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 1, y: 2 };
assert_eq!(p1, p2);
```

**PartialOrd 和 Ord：**

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 2, y: 3 };
assert!(p1 < p2);
```

### 3.5 泛型

**结构体中的泛型：**

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// 特定类型的实现
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多个泛型参数
struct Point<T, U> {
    x: T,
    y: U,
}

let point = Point { x: 5, y: 4.0 };
```

**枚举中的泛型：**

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**方法中的泛型：**

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };
let p3 = p1.mixup(p2);
```

### 3.6 关联类型

```rust
trait Iterator {
    type Item;  // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
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

### 3.7 运算符重载

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2;
assert_eq!(p3, Point { x: 4, y: 6 });
```

---

## 4. 错误处理

### 4.1 panic! 宏（不可恢复错误）

```rust
// 直接 panic
panic!("crash and burn");

// 数组越界会触发 panic
let v = vec![1, 2, 3];
v[99];  // panic!

// RUST_BACKTRACE=1 查看完整回溯
```

### 4.2 Result 枚举（可恢复错误）

```rust
use std::fs::File;
use std::io::ErrorKind;

// 基本使用
let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
};

// 匹配不同错误类型
let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
    },
};

// 使用闭包简化
let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
});
```

### 4.3 unwrap 和 expect

```rust
// unwrap：Ok 则返回值，Err 则 panic
let f = File::open("hello.txt").unwrap();

// expect：类似 unwrap，但可以自定义错误消息
let f = File::open("hello.txt")
    .expect("Failed to open hello.txt");
```

### 4.4 传播错误 (? 运算符)

```rust
use std::fs::File;
use std::io::{self, Read};

// 手动传播
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式调用
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 更简洁
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

### 4.5 自定义错误类型

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {}

fn do_something() -> Result<(), MyError> {
    Err(MyError {
        message: String::from("Something went wrong"),
    })
}

// 使用 thiserror 库简化
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("Custom error: {0}")]
    Custom(String),
}
```

### 4.6 anyhow 库（应用层错误处理）

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    Ok(())
}
```

---

## 5. 并发编程

### 5.1 线程

```rust
use std::thread;
use std::time::Duration;

// 创建线程
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("spawned thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("main thread: {}", i);
    thread::sleep(Duration::from_millis(1));
}

// 等待线程结束
handle.join().unwrap();

// 移动值到线程
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("vector: {:?}", v);
});
handle.join().unwrap();
```

### 5.2 消息传递 (Channel)

```rust
use std::sync::mpsc;
use std::thread;

// 创建通道
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);

// 发送多个值
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];
    
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// 多个生产者
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();

thread::spawn(move || {
    tx.send(String::from("hi from first")).unwrap();
});

thread::spawn(move || {
    tx1.send(String::from("hi from second")).unwrap();
});

for received in rx {
    println!("Got: {}", received);
}
```

### 5.3 共享状态 (Mutex)

```rust
use std::sync::{Mutex, Arc};
use std::thread;

// 基本使用
let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();
    *num = 6;
}  // 锁自动释放

println!("m = {:?}", m);

// 多线程共享
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

println!("Result: {}", *counter.lock().unwrap());
```

### 5.4 原子类型

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

let counter = Arc::new(AtomicUsize::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            counter.fetch_add(1, Ordering::SeqCst);
        }
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", counter.load(Ordering::SeqCst));
```

### 5.5 异步运行时 (Tokio)

```rust
use tokio;

#[tokio::main]
async fn main() {
    // 并发执行
    let task1 = tokio::spawn(async {
        // 异步工作
        42
    });
    
    let task2 = tokio::spawn(async {
        // 异步工作
        "hello"
    });
    
    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();
    
    println!("{}, {}", result1, result2);
}

// 异步 Mutex
use tokio::sync::Mutex;

let data = Arc::new(Mutex::new(0));

let data_clone = data.clone();
tokio::spawn(async move {
    let mut lock = data_clone.lock().await;
    *lock += 1;
});

// 异步 Channel
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(32);

tokio::spawn(async move {
    tx.send("hello").await.unwrap();
});

while let Some(msg) = rx.recv().await {
    println!("got: {}", msg);
}
```

---

## 6. IO 与网络

### 6.1 文件 IO

```rust
use std::fs::{self, File};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};

// 读取整个文件
let contents = fs::read_to_string("file.txt")?;
let bytes = fs::read("file.bin")?;

// 写入文件
fs::write("file.txt", "Hello, world!")?;
fs::write("file.bin", &[0, 1, 2, 3])?;

// 使用 File
let mut file = File::open("file.txt")?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;

let mut file = File::create("output.txt")?;
file.write_all(b"Hello, world!")?;

// 追加模式
use std::fs::OpenOptions;

let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("log.txt")?;
file.write_all(b"Log entry\n")?;

// 缓冲读写
let file = File::open("file.txt")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    println!("{}", line?);
}

let file = File::create("output.txt")?;
let mut writer = BufWriter::new(file);
writer.write_all(b"Hello\n")?;
writer.flush()?;

// 文件元数据
let metadata = fs::metadata("file.txt")?;
println!("Size: {}", metadata.len());
println!("Is file: {}", metadata.is_file());
println!("Is dir: {}", metadata.is_dir());

// 目录操作
fs::create_dir("new_dir")?;
fs::create_dir_all("path/to/new/dir")?;
fs::remove_dir("empty_dir")?;
fs::remove_dir_all("dir_with_contents")?;

// 遍历目录
for entry in fs::read_dir(".")? {
    let entry = entry?;
    println!("{:?}", entry.path());
}
```

### 6.2 标准输入输出

```rust
use std::io::{self, Write};

// 标准输出
println!("Hello, world!");
print!("No newline");
eprintln!("Error message");

// 直接写入
io::stdout().write_all(b"Hello\n")?;
io::stderr().write_all(b"Error\n")?;

// 标准输入
let mut input = String::new();
io::stdin().read_line(&mut input)?;
println!("You entered: {}", input.trim());

// 读取多行
for line in io::stdin().lock().lines() {
    let line = line?;
    println!("Got: {}", line);
}
```

### 6.3 HTTP 客户端 (reqwest)

```rust
use reqwest;

// 同步
let body = reqwest::blocking::get("https://www.rust-lang.org")?
    .text()?;
println!("{}", body);

// 异步
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET 请求
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("{}", body);
    
    // POST 请求
    let client = reqwest::Client::new();
    let res = client
        .post("https://httpbin.org/post")
        .json(&serde_json::json!({
            "name": "Alice",
            "age": 30
        }))
        .send()
        .await?;
    
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: {}", body);
    
    // 设置请求头
    let res = client
        .get("https://api.example.com")
        .header("Authorization", "Bearer token")
        .header("User-Agent", "My App")
        .send()
        .await?;
    
    // JSON 响应
    #[derive(serde::Deserialize)]
    struct ApiResponse {
        id: u32,
        name: String,
    }
    
    let data: ApiResponse = reqwest::get("https://api.example.com/data")
        .await?
        .json()
        .await?;
    
    Ok(())
}
```

### 6.4 HTTP 服务器 (axum)

```rust
use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, Query},
    response::Html,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/search", get(search));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    let user = User {
        id,
        name: "Alice".to_string(),
    };
    Json(user)
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 1,
        name: payload.name,
    };
    Json(user)
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    page: Option<u32>,
}

async fn search(Query(params): Query<SearchParams>) -> String {
    format!("Searching for: {} (page: {})", 
        params.q, 
        params.page.unwrap_or(1))
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}
```

### 6.5 WebSocket

```rust
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 客户端
    let (ws_stream, _) = connect_async("ws://localhost:8080").await?;
    let (mut write, mut read) = ws_stream.split();
    
    // 发送消息
    write.send(Message::Text("Hello".to_string())).await?;
    
    // 接收消息
    while let Some(msg) = read.next().await {
        let msg = msg?;
        println!("Received: {:?}", msg);
    }
    
    Ok(())
}
```

### 6.6 TCP Socket

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP 服务器
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);
        
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => {
                if socket.write_all(&buf[0..n]).await.is_err() {
                    return;
                }
            }
            Err(_) => return,
        }
    }
}

// TCP 客户端
async fn tcp_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    
    stream.write_all(b"Hello, server!").await?;
    
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[0..n]));
    
    Ok(())
}
```

---

## 7. 进程管理

### 7.1 执行外部命令

```rust
use std::process::Command;

// 基本执行
let output = Command::new("ls")
    .arg("-l")
    .arg("-a")
    .output()?;

println!("Status: {}", output.status);
println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

// 检查状态
if output.status.success() {
    println!("Success!");
} else {
    println!("Failed with: {}", output.status);
}

// 流式输出
let status = Command::new("cargo")
    .arg("build")
    .status()?;

println!("Exit code: {:?}", status.code());

// 设置工作目录和环境变量
let output = Command::new("ls")
    .current_dir("/tmp")
    .env("RUST_LOG", "debug")
    .env_remove("PATH")
    .output()?;

// 管道
use std::process::Stdio;

let process = Command::new("echo")
    .arg("Hello")
    .stdout(Stdio::piped())
    .spawn()?;

let output = process.wait_with_output()?;
```

### 7.2 子进程通信

```rust
use std::process::{Command, Stdio};
use std::io::{Write, Read};

// 写入 stdin
let mut child = Command::new("cat")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

{
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"Hello from parent\n")?;
}  // stdin 关闭

let output = child.wait_with_output()?;
println!("Output: {}", String::from_utf8_lossy(&output.stdout));

// 读取 stdout
let mut child = Command::new("ls")
    .stdout(Stdio::piped())
    .spawn()?;

let mut stdout = child.stdout.take().unwrap();
let mut buffer = String::new();
stdout.read_to_string(&mut buffer)?;

child.wait()?;
println!("Output: {}", buffer);
```

### 7.3 异步进程

```rust
use tokio::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .await?;
    
    println!("Status: {}", output.status);
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    
    Ok(())
}
```

### 7.4 信号处理

```rust
use tokio::signal;

#[tokio::main]
async fn main() {
    let ctrl_c = signal::ctrl_c();
    
    tokio::select! {
        _ = ctrl_c => {
            println!("Received Ctrl+C!");
        }
        _ = do_work() => {
            println!("Work completed!");
        }
    }
}

async fn do_work() {
    // 长时间运行的任务
}
```

---

## 8. 注意事项与高级技巧

### 8.1 性能优化

**避免不必要的克隆：**

```rust
// 不好
fn process(s: String) -> String {
    s.clone()
}

// 好
fn process(s: &str) -> String {
    s.to_string()
}
```

**使用迭代器：**

```rust
// 不好
let mut sum = 0;
for i in 0..1000 {
    sum += i;
}

// 好（零成本抽象）
let sum: i32 = (0..1000).sum();
```

**预分配容量：**

```rust
// 不好
let mut vec = Vec::new();
for i in 0..1000 {
    vec.push(i);
}

// 好
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);
}
```

**使用 Cow 避免不必要的克隆：**

```rust
use std::borrow::Cow;

fn process(input: &str) -> Cow<str> {
    if input.contains("bad") {
        Cow::Owned(input.replace("bad", "good"))
    } else {
        Cow::Borrowed(input)
    }
}
```

### 8.2 内存管理

**智能指针：**

```rust
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

// Rc：单线程引用计数
let a = Rc::new(5);
let b = Rc::clone(&a);
println!("Count: {}", Rc::strong_count(&a));

// Arc：线程安全引用计数
let a = Arc::new(5);
let b = Arc::clone(&a);

// RefCell：内部可变性
let value = RefCell::new(5);
*value.borrow_mut() += 1;
```

**Box 堆分配：**

```rust
// 递归类型
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 大型结构
let large_data = Box::new([0; 1000000]);
```

### 8.3 宏编程

**声明宏：**

```rust
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = my_vec![1, 2, 3];
```

**过程宏（derive）：**

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}
```

### 8.4 测试

```rust
// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("This should panic");
    }
    
    #[test]
    fn returns_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Math is broken"))
        }
    }
}

// 集成测试（tests/ 目录）
// 文档测试
/// 将两个数字相加
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 8.5 模块系统

```rust
// lib.rs 或 main.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// 文件系统模块
// src/lib.rs
mod front_of_house;

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}

// 或 src/front_of_house/mod.rs
// src/front_of_house/hosting.rs
```

### 8.6 常用设计模式

**构建器模式：**

```rust
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

struct UserBuilder {
    name: String,
    age: u32,
    email: Option<String>,
}

impl UserBuilder {
    fn new(name: String, age: u32) -> Self {
        UserBuilder {
            name,
            age,
            email: None,
        }
    }
    
    fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    fn build(self) -> User {
        User {
            name: self.name,
            age: self.age,
            email: self.email,
        }
    }
}

let user = UserBuilder::new("Alice".to_string(), 30)
    .email("alice@example.com".to_string())
    .build();
```

**新类型模式：**

```rust
struct Meters(u32);
struct Kilometers(u32);

impl Meters {
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 / 1000)
    }
}
```

**类型状态模式：**

```rust
struct Locked;
struct Unlocked;

struct Door<State> {
    state: std::marker::PhantomData<State>,
}

impl Door<Locked> {
    fn unlock(self) -> Door<Unlocked> {
        Door {
            state: std::marker::PhantomData,
        }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        Door {
            state: std::marker::PhantomData,
        }
    }
    
    fn open(&self) {
        println!("Door opened");
    }
}
```

### 8.7 最佳实践

1. **使用 clippy 检查代码质量：**
   ```bash
   cargo clippy
   ```

2. **使用 rustfmt 格式化代码：**
   ```bash
   cargo fmt
   ```

3. **编写文档：**
   ```rust
   /// 这是一个公共函数
   ///
   /// # Arguments
   ///
   /// * `x` - 第一个参数
   /// * `y` - 第二个参数
   ///
   /// # Returns
   ///
   /// 返回 x 和 y 的和
   ///
   /// # Examples
   ///
   /// ```
   /// let result = add(2, 3);
   /// assert_eq!(result, 5);
   /// ```
   pub fn add(x: i32, y: i32) -> i32 {
       x + y
   }
   ```

4. **错误处理：** 库代码返回 `Result`，应用代码可以使用 `unwrap` 或 `expect`

5. **避免过度使用 `unsafe`：** 只在必要时使用，并添加详细注释

6. **使用类型系统：** 让编译器帮你捕获错误

7. **避免过早优化：** 先保证正确性，再优化性能

8. **利用 Cargo 工作空间：** 管理多个相关 crate

### 8.8 常用 Crates

- **序列化：** serde, serde_json, toml, bincode
- **日志：** log, env_logger, tracing
- **CLI：** clap, structopt
- **数据库：** sqlx, diesel, tokio-postgres
- **Web：** axum, actix-web, rocket
- **异步：** tokio, async-std
- **HTTP：** reqwest, hyper
- **时间：** chrono, time
- **随机：** rand
- **正则：** regex
- **错误处理：** anyhow, thiserror
- **测试：** proptest, criterion

---

## 总结

Rust 是一门注重安全、并发和性能的系统编程语言。通过所有权系统、借用检查和生命周期，Rust 在编译时保证内存安全和线程安全。虽然学习曲线较陡，但掌握后可以编写高效、可靠的系统级应用。

关键要点：
- **所有权系统**是 Rust 的核心，理解它是掌握 Rust 的关键
- **trait 和泛型**提供了强大的抽象能力
- **错误处理**使用 `Result` 和 `Option` 而非异常
- **并发编程**通过类型系统保证线程安全
- **零成本抽象**让高级特性不牺牲性能

持续学习资源：
- 官方文档：https://doc.rust-lang.org/book/
- Rust by Example：https://doc.rust-lang.org/rust-by-example/
- Rustlings：https://github.com/rust-lang/rustlings
- Crates.io：https://crates.io/

Happy Coding! 🦀
