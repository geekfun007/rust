# Rust 核心类型与 Trait 详解

## 目录
1. [Box<T> - 智能指针](#1-boxt---智能指针)
2. [Option<T> - 可选值](#2-optiont---可选值)
3. [Result<T, E> - 错误处理](#3-resultt-e---错误处理)
4. [fmt::Display - 格式化输出](#4-fmtdisplay---格式化输出)
5. [其他核心 Trait](#5-其他核心-trait)

---

## 1. Box<T> - 智能指针

### 1.1 核心概念

**Box<T> 是 Rust 中最简单的智能指针，用于在堆上分配数据。**

### 1.2 内存布局

```
Box<i32> 内存结构：

栈上（Box 本身）          堆上（实际数据）
┌──────────┐             ┌──────────┐
│  ptr     │────────────>│    42    │
└──────────┘             └──────────┘
8 字节                   4 字节

关键点：
1. Box 本身只占用一个指针大小（8字节，64位系统）
2. 实际数据存储在堆上
3. 离开作用域时自动释放堆内存
4. 所有权语义：移动时转移所有权
```

### 1.3 使用场景

#### 场景 1：递归类型

```rust
// ❌ 错误：无限大小
// struct List {
//     value: i32,
//     next: List,  // 编译错误
// }

// ✅ 正确：使用 Box 打破递归
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 内存布局：
// list = Cons(1, Box::new(Cons(2, Box::new(Nil))))
// 
// 栈                     堆
// Cons ┌───┐            ┌─────────────┐
//      │ 1 │            │ Cons        │
//      ├───┤            │ ├───┐       │
//      │ptr│───────────>│ │ 2 │       │
//      └───┘            │ ├───┤       │
//                       │ │ptr│─────> Nil
//                       │ └───┘       │
//                       └─────────────┘
```

#### 场景 2：大型数据

```rust
// 避免栈溢出
struct LargeData {
    data: [u8; 1_000_000],  // 1MB
}

// ❌ 栈上分配可能溢出
// let data = LargeData { data: [0; 1_000_000] };

// ✅ 堆上分配
let data = Box::new(LargeData { data: [0; 1_000_000] });

// 性能：
// - 栈分配：瞬时但可能溢出
// - Box 分配：稍慢但安全
```

#### 场景 3：动态分派 (Trait Object)

```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Draw for Circle { fn draw(&self) { println!("○"); } }
impl Draw for Square { fn draw(&self) { println!("□"); } }

// 编译时不知道具体类型
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle),
    Box::new(Square),
];

// 内存布局：
// Box<dyn Draw> 是胖指针（16字节）
// ┌─────────┬─────────┐
// │ data_ptr│ vtable  │
// └─────────┴─────────┘
//     8B        8B
```

### 1.4 Box 的方法

```rust
// 创建
let b = Box::new(5);

// 解引用
let value = *b;  // 5

// 泄漏内存（转为裸指针）
let ptr = Box::into_raw(b);
// 需要手动释放
unsafe { drop(Box::from_raw(ptr)); }

// 从裸指针创建
let b = unsafe { Box::from_raw(ptr) };

// 模式匹配
let b = Box::new(5);
match b {
    box 5 => println!("匹配!"),  // 需要 #![feature(box_patterns)]
    _ => {}
}
```

### 1.5 Box vs 其他智能指针

```
┌──────────┬──────────┬──────────┬──────────┐
│ 类型     │ 所有权   │ 线程安全 │ 开销     │
├──────────┼──────────┼──────────┼──────────┤
│ Box<T>   │ 独占     │ 否       │ 最低     │
│ Rc<T>    │ 共享     │ 否       │ 引用计数 │
│ Arc<T>   │ 共享     │ 是       │ 原子计数 │
│ &T       │ 借用     │ 是       │ 零       │
└──────────┴──────────┴──────────┴──────────┘
```

---

## 2. Option<T> - 可选值

### 2.1 核心概念

**Option<T> 是 Rust 中表示"可能存在的值"的类型，用于替代空指针。**

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

### 2.2 内存布局

```
Option<i32> 内存布局：

Some(42)                 None
┌────┬────────┐         ┌────┬────────┐
│ 1  │   42   │         │ 0  │ 未定义 │
└────┴────────┘         └────┴────────┘
tag   value              tag   (不使用)
1B    4B (对齐到8B)     1B

Option<&T> 优化（空指针优化）：

Some(&42)                None
┌──────────┐            ┌──────────┐
│  ptr     │            │  null    │
└──────────┘            └──────────┘
8 字节                  8 字节

关键：Option<&T> 只占 8 字节，利用了指针不能为 null 的特性
```

### 2.3 创建 Option

```rust
// 方式 1：显式创建
let some_number = Some(5);
let no_number: Option<i32> = None;

// 方式 2：从可空值转换
let s = "123";
let n: Option<i32> = s.parse().ok();

// 方式 3：条件创建
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
```

### 2.4 Option 的方法

#### 检查方法

```rust
let x: Option<i32> = Some(42);

// 是否有值
assert!(x.is_some());
assert!(!x.is_none());

// 比较值
assert_eq!(x, Some(42));
```

#### 提取值

```rust
let x = Some(42);

// unwrap - 有值返回，无值 panic
let value = x.unwrap();  // 42

// expect - 自定义 panic 信息
let value = x.expect("应该有值");

// unwrap_or - 提供默认值
let value = x.unwrap_or(0);  // 42
let none: Option<i32> = None;
let value = none.unwrap_or(0);  // 0

// unwrap_or_else - 懒计算默认值
let value = none.unwrap_or_else(|| expensive_default());

// unwrap_or_default - 使用 Default trait
let value = none.unwrap_or_default();  // 0
```

#### 转换方法

```rust
let x = Some(42);

// map - 转换内部值
let doubled = x.map(|v| v * 2);  // Some(84)

// map_or - 带默认值的 map
let result = x.map_or(0, |v| v * 2);  // 84

// map_or_else - 懒计算默认值
let result = x.map_or_else(|| 0, |v| v * 2);

// and_then (flatMap) - 链式操作
let result = x.and_then(|v| {
    if v > 0 {
        Some(v * 2)
    } else {
        None
    }
});  // Some(84)

// filter - 过滤
let result = x.filter(|&v| v > 40);  // Some(42)
let result = x.filter(|&v| v > 50);  // None
```

#### 组合方法

```rust
let x = Some(2);
let y: Option<i32> = None;

// and - 都有值返回第二个
assert_eq!(x.and(Some(3)), Some(3));
assert_eq!(x.and(None), None);
assert_eq!(y.and(Some(3)), None);

// or - 有值返回第一个有值的
assert_eq!(x.or(Some(3)), Some(2));
assert_eq!(y.or(Some(3)), Some(3));
assert_eq!(y.or(None), None);

// xor - 异或
assert_eq!(x.xor(None), Some(2));
assert_eq!(x.xor(Some(3)), None);
```

#### ? 操作符

```rust
fn try_parse(s: &str) -> Option<i32> {
    let n: i32 = s.parse().ok()?;  // 失败时提前返回 None
    Some(n * 2)
}

// 等价于：
fn try_parse_verbose(s: &str) -> Option<i32> {
    match s.parse().ok() {
        Some(n) => Some(n * 2),
        None => None,
    }
}
```

### 2.5 模式匹配

```rust
let x = Some(42);

// match
match x {
    Some(value) => println!("值: {}", value),
    None => println!("无值"),
}

// if let
if let Some(value) = x {
    println!("值: {}", value);
}

// while let
let mut stack = vec![Some(1), Some(2), Some(3)];
while let Some(Some(value)) = stack.pop() {
    println!("{}", value);
}
```

### 2.6 实战模式

#### 模式 1：链式调用

```rust
fn process(input: Option<String>) -> Option<usize> {
    input
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .map(|s| s.len())
}
```

#### 模式 2：多个 Option 组合

```rust
fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}

// 或使用 and_then
fn add_options_2(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.and_then(|x| b.map(|y| x + y))
}
```

#### 模式 3：提前返回

```rust
fn get_user_age(user_id: u32) -> Option<u32> {
    let user = find_user(user_id)?;
    let profile = user.profile?;
    Some(profile.age)
}
```

---

## 3. Result<T, E> - 错误处理

### 3.1 核心概念

**Result<T, E> 是 Rust 中用于可恢复错误处理的类型。**

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 3.2 内存布局

```
Result<i32, String> 内存布局：

Ok(42)                      Err("error")
┌────┬────────┐            ┌────┬────────────────┐
│ 0  │   42   │            │ 1  │ String {       │
│    │        │            │    │   ptr,         │
│    │        │            │    │   len,         │
│    │        │            │    │   cap          │
│    │        │            │    │ }              │
└────┴────────┘            └────┴────────────────┘
tag   Ok value             tag   Err value
1B    4B (padding)         1B    24B

大小 = 1 + max(size_of::<T>(), size_of::<E>()) + 对齐
```

### 3.3 创建 Result

```rust
// 成功
let success: Result<i32, String> = Ok(42);

// 失败
let failure: Result<i32, String> = Err("出错了".to_string());

// 从函数返回
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

// 从 Option 转换
let opt = Some(42);
let result = opt.ok_or("没有值");  // Ok(42)
let result = opt.ok_or_else(|| "没有值".to_string());
```

### 3.4 Result 的方法

#### 检查方法

```rust
let x: Result<i32, &str> = Ok(42);

// 是否成功/失败
assert!(x.is_ok());
assert!(!x.is_err());

// 比较值
assert_eq!(x, Ok(42));
```

#### 提取值

```rust
let x = Ok(42);

// unwrap - 成功返回值，失败 panic
let value = x.unwrap();  // 42

// expect - 自定义 panic 信息
let value = x.expect("应该成功");

// unwrap_or - 提供默认值
let value = x.unwrap_or(0);

// unwrap_err - 提取错误（Ok 会 panic）
let err = Err::<i32, _>("错误").unwrap_err();
```

#### 转换方法

```rust
let x: Result<i32, &str> = Ok(42);

// map - 转换 Ok 值
let doubled = x.map(|v| v * 2);  // Ok(84)

// map_err - 转换 Err 值
let result = x.map_err(|e| format!("错误: {}", e));

// and_then - 链式操作（可能失败）
let result = x.and_then(|v| {
    if v > 0 {
        Ok(v * 2)
    } else {
        Err("值必须为正")
    }
});

// or_else - 错误恢复
let result = Err("错误").or_else(|_| Ok(0));  // Ok(0)
```

#### ? 操作符

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;  // 失败时提前返回错误
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 等价于：
fn read_file_verbose(path: &str) -> io::Result<String> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}
```

### 3.5 模式匹配

```rust
let result: Result<i32, &str> = Ok(42);

// match
match result {
    Ok(value) => println!("成功: {}", value),
    Err(error) => println!("失败: {}", error),
}

// if let
if let Ok(value) = result {
    println!("值: {}", value);
}

// if let Err
if let Err(error) = result {
    eprintln!("错误: {}", error);
}
```

### 3.6 错误传播

#### 模式 1：? 操作符链

```rust
fn complex_operation() -> Result<i32, Box<dyn std::error::Error>> {
    let a = operation1()?;
    let b = operation2(a)?;
    let c = operation3(b)?;
    Ok(c)
}
```

#### 模式 2：多个错误类型

```rust
use std::num::ParseIntError;
use std::io;

// 方式 1：使用 Box<dyn Error>
fn parse_and_read() -> Result<i32, Box<dyn std::error::Error>> {
    let n: i32 = "42".parse()?;
    let mut file = std::fs::File::open("data.txt")?;
    Ok(n)
}

// 方式 2：自定义错误类型
#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    Io(io::Error),
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

fn parse_and_read_2() -> Result<i32, MyError> {
    let n: i32 = "42".parse()?;  // 自动转换
    let mut file = std::fs::File::open("data.txt")?;
    Ok(n)
}
```

### 3.7 Result vs Option

```
┌──────────┬──────────────┬──────────────┐
│ 特性     │ Option<T>    │ Result<T, E> │
├──────────┼──────────────┼──────────────┤
│ 用途     │ 可能无值     │ 可能失败     │
│ 成功     │ Some(T)      │ Ok(T)        │
│ 失败     │ None         │ Err(E)       │
│ 错误信息 │ 无           │ 有（E）      │
│ ? 操作符 │ 支持         │ 支持         │
└──────────┴──────────────┴──────────────┘

转换：
- Option -> Result: ok_or(), ok_or_else()
- Result -> Option: ok(), err()
```

---

## 4. fmt::Display - 格式化输出

### 4.1 核心概念

**fmt::Display 是用于用户友好格式化输出的 trait。**

```rust
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

### 4.2 Display vs Debug

```
┌──────────┬──────────────────┬──────────────────┐
│ Trait    │ 用途             │ 语法             │
├──────────┼──────────────────┼──────────────────┤
│ Display  │ 用户友好输出     │ {}               │
│ Debug    │ 程序员调试输出   │ {:?} 或 {:#?}    │
└──────────┴──────────────────┴──────────────────┘
```

### 4.3 实现 Display

#### 基本实现

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 使用
let p = Point { x: 10, y: 20 };
println!("点: {}", p);  // 点: (10, 20)
```

#### 复杂实现

```rust
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 可以多次调用 write!
        write!(f, "姓名: {}", self.name)?;
        write!(f, ", 年龄: {}", self.age)
    }
}
```

#### 条件格式化

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "✓ 激活"),
            Status::Inactive => write!(f, "✗ 停用"),
            Status::Pending => write!(f, "⏳ 待定"),
        }
    }
}
```

### 4.4 格式化参数

```rust
struct Number(i32);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 读取格式化参数
        let width = f.width();
        let precision = f.precision();
        
        match (width, precision) {
            (Some(w), Some(p)) => {
                write!(f, "{:w$.p$}", self.0, w = w, p = p)
            }
            (Some(w), None) => {
                write!(f, "{:w$}", self.0, w = w)
            }
            (None, Some(p)) => {
                write!(f, "{:.p$}", self.0, p = p)
            }
            (None, None) => {
                write!(f, "{}", self.0)
            }
        }
    }
}

// 使用
let n = Number(42);
println!("{}", n);        // 42
println!("{:5}", n);      // "   42"
println!("{:.2}", n);     // "42.00"
```

### 4.5 其他格式化 Trait

#### Debug

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

let p = Point { x: 1, y: 2 };
println!("{:?}", p);   // Point { x: 1, y: 2 }
println!("{:#?}", p);  // 美化输出
```

#### Binary, Octal, Hex

```rust
struct Flags(u32);

impl fmt::Binary for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::Octal for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:o}", self.0)
    }
}

impl fmt::LowerHex for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

let flags = Flags(42);
println!("二进制: {:b}", flags);  // 101010
println!("八进制: {:o}", flags);  // 52
println!("十六进制: {:x}", flags); // 2a
```

### 4.6 format! 宏家族

```rust
// format! - 创建 String
let s = format!("x = {}, y = {}", 10, 20);

// print! - 标准输出
print!("Hello ");

// println! - 标准输出 + 换行
println!("World!");

// eprint! - 标准错误输出
eprint!("错误: ");

// eprintln! - 标准错误输出 + 换行
eprintln!("出现问题!");

// write! - 写入实现 Write 的类型
use std::fmt::Write;
let mut s = String::new();
write!(&mut s, "x = {}", 42).unwrap();

// writeln! - write! + 换行
writeln!(&mut s, "y = {}", 24).unwrap();
```

---

## 5. 其他核心 Trait

### 5.1 From & Into

**类型转换 trait**

```rust
// From
impl From<i32> for MyType {
    fn from(value: i32) -> Self {
        MyType(value)
    }
}

let x: MyType = MyType::from(42);
let x: MyType = 42.into();  // Into 自动实现

// 常见用法：错误转换
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}
```

### 5.2 Default

**默认值 trait**

```rust
#[derive(Default)]
struct Config {
    timeout: u32,  // 0
    retries: u32,  // 0
}

let config = Config::default();
```

### 5.3 Clone & Copy

```rust
// Clone - 显式克隆
#[derive(Clone)]
struct Data {
    value: String,
}

let d1 = Data { value: "hello".to_string() };
let d2 = d1.clone();

// Copy - 隐式复制（栈上按位复制）
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // 复制，p1 仍然有效
```

### 5.4 Drop

**析构函数 trait**

```rust
struct FileGuard {
    file: std::fs::File,
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("关闭文件");
        // 自动清理
    }
}
```

### 5.5 Iterator

**迭代器 trait**

```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

## 6. 实战最佳实践

### 6.1 何时使用 Box

```rust
// ✓ 递归类型
enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),
}

// ✓ 大型数据
let large = Box::new([0u8; 1_000_000]);

// ✓ Trait 对象
let drawable: Box<dyn Draw> = Box::new(Circle);

// ✗ 小数据（不如直接用栈）
let x = Box::new(42);  // 浪费
```

### 6.2 Option vs Result

```rust
// ✓ 用 Option
fn find_item(id: u32) -> Option<Item> { ... }  // 找不到是正常的

// ✓ 用 Result
fn parse_config(path: &str) -> Result<Config, Error> { ... }  // 失败需要知道原因
```

### 6.3 错误处理策略

```rust
// ✓ 库代码：返回详细错误
fn library_fn() -> Result<T, DetailedError> { ... }

// ✓ 应用代码：使用 anyhow
fn app_fn() -> anyhow::Result<T> { ... }

// ✓ 关键路径：使用 expect
let config = load_config().expect("配置文件必须存在");

// ✗ 滥用 unwrap
let x = some_result.unwrap();  // 可能 panic
```

---

## 7. 性能考虑

```
┌──────────────┬──────────┬──────────────┐
│ 操作         │ 开销     │ 说明         │
├──────────────┼──────────┼──────────────┤
│ Box::new     │ 堆分配   │ ~100 cycles  │
│ Box 移动     │ 8 字节   │ 只复制指针   │
│ Option/Some  │ 1 字节   │ 标签开销     │
│ Option<&T>   │ 0 字节   │ 空指针优化   │
│ Result       │ 1 字节   │ 标签开销     │
│ Display::fmt │ 虚函数   │ 动态分派     │
└──────────────┴──────────┴──────────────┘
```

---

**更多信息：**
- 运行示例：`cargo run --example core_types`
- 查看测试：`cargo test --example core_types`
- 官方文档：https://doc.rust-lang.org/std/
