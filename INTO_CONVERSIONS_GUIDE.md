# Rust Into 系列转换完全指南

完整的 `into_` 系列方法详解，包含理论、实践和最佳实践。

## 目录

1. [核心概念](#核心概念)
2. [into() - 通用转换](#into---通用转换)
3. [into_iter() - 迭代器转换](#into_iter---迭代器转换)
4. [into_bytes() - 字节转换](#into_bytes---字节转换)
5. [into_boxed_slice() - Box 转换](#into_boxed_slice---box-转换)
6. [into_string() - 字符串转换](#into_string---字符串转换)
7. [into_inner() - 解包转换](#into_inner---解包转换)
8. [From vs Into](#from-vs-into)
9. [实战案例](#实战案例)
10. [最佳实践](#最佳实践)

---

## 核心概念

### Into 方法特点

| 特性 | 说明 |
|------|------|
| **所有权** | 消费原值，转移所有权 |
| **开销** | 移动，通常零成本 |
| **原值** | 不可再使用 |
| **返回** | 新类型的值 |

### Into Trait

```rust
trait Into<T> {
    fn into(self) -> T;
}
```

**关键特点**:
- 消费 `self`（不是 `&self`）
- 自动类型推导
- 通常通过实现 `From` 自动获得

---

## into() - 通用转换

### 基础用法

```rust
// &str → String
let s: String = "hello".into();

// i32 → i64
let num: i64 = 42i32.into();

// Vec → Box<[T]>
let vec = vec![1, 2, 3];
let boxed: Box<[i32]> = vec.into();
// vec 不可再用！
```

### 自动类型推导

```rust
// 函数参数推导
fn takes_string(s: String) {
    println!("{}", s);
}

takes_string("hello".into());  // 自动推导为 String

// 变量类型推导
let vec = vec![1, 2, 3];
let boxed: Box<[i32]> = vec.into();
```

### 自定义 Into

**推荐方式：实现 From**

```rust
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

// 使用 into()
let celsius = Celsius(25.0);
let fahrenheit: Fahrenheit = celsius.into();
// celsius 不可再用
```

**复杂转换**:

```rust
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
            id: 0,  // 默认 ID
        }
    }
}

// 使用
let person = Person {
    name: "Alice".to_string(),
    age: 30,
};

let employee: Employee = person.into();
// person 已移动
```

---

## into_iter() - 迭代器转换

### 三种迭代方式

```rust
┌──────────────┬───────────┬──────────┬──────────┐
│  方法         │  类型      │  所有权   │  原值    │
├──────────────┼───────────┼──────────┼──────────┤
│  iter()      │  &T       │  借用     │  可用    │
│  iter_mut()  │  &mut T   │  可变借用 │  可用    │
│  into_iter() │  T        │  转移     │  不可用  │
└──────────────┴───────────┴──────────┴──────────┘
```

### Vec::into_iter()

```rust
let vec = vec![1, 2, 3, 4, 5];

// 拥有所有权的迭代器
for value in vec.into_iter() {
    println!("{}", value);  // value 是 i32，不是 &i32
}
// vec 不可再用
```

### 对比示例

```rust
let vec = vec![String::from("a"), String::from("b"), String::from("c")];

// 1. iter() - 借用
for item in vec.iter() {
    println!("{}", item);  // &String
}
println!("{:?}", vec);  // 仍可用

// 2. iter_mut() - 可变借用
let mut vec = vec![String::from("a"), String::from("b")];
for item in vec.iter_mut() {
    item.push_str("!");
}
println!("{:?}", vec);  // 仍可用

// 3. into_iter() - 所有权转移
let vec = vec![String::from("a"), String::from("b")];
for item in vec.into_iter() {
    println!("{}", item);  // String (owned)
}
// println!("{:?}", vec);  // 错误！已移动
```

### HashMap::into_iter()

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("name", "Alice");
map.insert("age", "25");

// 获取键值对的所有权
for (key, value) in map.into_iter() {
    println!("{} = {}", key, value);
}
// map 不可再用
```

### 数组 into_iter()

```rust
let arr = [10, 20, 30];

for value in arr.into_iter() {
    println!("{}", value);
}

// 数组的 Copy 类型，原值仍可用
println!("{:?}", arr);
```

---

## into_bytes() - 字节转换

### String::into_bytes()

```rust
let s = String::from("Hello, 世界!");
let bytes = s.into_bytes();

println!("{:?}", bytes);
// [72, 101, 108, 108, 111, 44, 32, 228, 184, 150, 231, 149, 140, 33]

// s 不可再用
// println!("{}", s);  // 错误！
```

### 从字节恢复

```rust
let bytes = vec![72, 101, 108, 108, 111];
let s = String::from_utf8(bytes).unwrap();
println!("{}", s);  // "Hello"
```

### 与 as_bytes() 对比

```rust
// as_bytes() - 借用
let s = "hello";
let bytes = s.as_bytes();  // &[u8]
println!("{}", s);  // 仍可用

// into_bytes() - 转移所有权
let s = String::from("hello");
let bytes = s.into_bytes();  // Vec<u8>
// println!("{}", s);  // 错误！
```

---

## into_boxed_slice() - Box 转换

### Vec::into_boxed_slice()

```rust
let mut vec = Vec::with_capacity(100);
vec.extend([1, 2, 3, 4, 5]);

println!("容量: {}", vec.capacity());  // 100

let boxed = vec.into_boxed_slice();
println!("长度: {}", boxed.len());      // 5
// vec 不可再用
```

### 性能优化

**为什么使用 `into_boxed_slice()`？**

1. **释放多余容量**: `Vec` 可能有未使用的容量
2. **固定大小**: Box<[T]> 大小固定，不能增长
3. **内存效率**: 精确匹配实际使用的内存

```rust
// 浪费内存
let mut vec = Vec::with_capacity(1000);
vec.push(1);
vec.push(2);
// 容量 1000，但只用了 2

// 优化
let boxed = vec.into_boxed_slice();
// 释放了 998 个元素的空间
```

---

## into_string() - 字符串转换

### OsString::into_string()

```rust
use std::ffi::OsString;

let os_string = OsString::from("hello");

match os_string.into_string() {
    Ok(s) => println!("成功: {}", s),
    Err(os) => println!("失败: {:?}", os),
}
// os_string 不可再用
```

### PathBuf::into_os_string()

```rust
use std::path::PathBuf;
use std::ffi::OsString;

let path = PathBuf::from("/usr/local/bin");
let os_string: OsString = path.into_os_string();
// path 不可再用
```

---

## into_inner() - 解包转换

### BufWriter::into_inner()

```rust
use std::io::{BufWriter, Write};

let buffer = Vec::new();
let mut writer = BufWriter::new(buffer);
write!(writer, "Hello, World!").unwrap();

// 提取内部 buffer
let buffer = writer.into_inner().unwrap();
println!("{:?}", String::from_utf8_lossy(&buffer));
// writer 不可再用
```

### Mutex::into_inner()

```rust
use std::sync::Mutex;

let mutex = Mutex::new(42);
let value = mutex.into_inner().unwrap();
println!("{}", value);  // 42
// mutex 不可再用
```

### Box 解包

```rust
let boxed = Box::new(String::from("boxed"));
let unboxed = *boxed;  // 移出 Box
println!("{}", unboxed);
```

---

## From vs Into

### 对比

```rust
┌──────────┬────────────┬────────────┐
│  特性     │  From      │  Into      │
├──────────┼────────────┼────────────┤
│  方向     │  明确      │  推导      │
│  实现     │  手动      │  自动      │
│  推荐     │  实现 From │  使用 Into │
└──────────┴────────────┴────────────┘
```

### 实现建议

**✅ 推荐：实现 From**

```rust
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

// 自动获得 Into
let point: Point = (10, 20).into();
```

**❌ 不推荐：同时实现 From 和 Into**

```rust
// 不要这样做
impl From<A> for B { ... }
impl Into<B> for A { ... }  // 冲突！
```

### 使用场景

```rust
// 使用 From - 类型明确
let point = Point::from((10, 20));

// 使用 Into - 类型推导
let point: Point = (30, 40).into();

// 函数参数 - Into 更灵活
fn process<T: Into<String>>(s: T) {
    let string = s.into();
    println!("{}", string);
}

process("hello");               // &str
process(String::from("world")); // String
```

---

## 实战案例

### 案例 1: 错误类型转换

```rust
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
    // io::Error 自动转换为 AppError
    let _content = std::fs::read_to_string("test.txt")?;
    
    // ParseIntError 自动转换为 AppError
    let num: i32 = "42".parse()?;
    
    Ok(num)
}
```

### 案例 2: 构建器模式

```rust
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
    
    // 接受任何可转换为 String 的类型
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

// 使用
let config = ConfigBuilder::new()
    .host("127.0.0.1")      // &str 自动转换
    .port(3000)
    .build();
```

### 案例 3: 数据处理管道

```rust
fn process_data(data: Vec<&str>) -> Vec<i32> {
    data.into_iter()                    // 转换为迭代器
        .filter_map(|s| s.parse().ok()) // 解析并过滤
        .map(|n: i32| n * 2)            // 处理
        .collect()                       // 收集结果
}

let data = vec!["1", "2", "invalid", "3"];
let result = process_data(data);
println!("{:?}", result);  // [2, 4, 6]
```

---

## 最佳实践

### ✅ 推荐做法

1. **实现 From，使用 Into**
   ```rust
   // 好
   impl From<A> for B { ... }
   let b: B = a.into();
   ```

2. **利用类型推导**
   ```rust
   // 好 - 清晰
   let s: String = "hello".into();
   
   // 好 - 函数推导
   fn takes_string(s: String) { }
   takes_string("hello".into());
   ```

3. **注意所有权转移**
   ```rust
   // 好 - 明确知道 vec 已移动
   let iter = vec.into_iter();
   // 不再使用 vec
   ```

4. **选择合适的迭代方式**
   ```rust
   // 只读取
   for item in vec.iter() { }
   
   // 需要修改
   for item in vec.iter_mut() { }
   
   // 需要所有权
   for item in vec.into_iter() { }
   ```

### ❌ 避免做法

1. **into 后使用原值**
   ```rust
   // 错误
   let bytes = s.into_bytes();
   println!("{}", s);  // 编译错误！
   ```

2. **不必要的转换**
   ```rust
   // 差
   let s = "hello".to_string().into_bytes();
   
   // 好
   let s = b"hello".to_vec();
   ```

3. **忽略所有权语义**
   ```rust
   // 差 - 可能意外移动
   let vec = vec![1, 2, 3];
   process(vec.into_iter());
   // vec 已移动，但可能不明显
   ```

---

## 总结

### Into 系列方法

| 方法 | 作用 | 所有权 | 返回 |
|------|------|--------|------|
| `into()` | 通用转换 | 转移 | 新类型 |
| `into_iter()` | 迭代器 | 转移 | 迭代器 |
| `into_bytes()` | 字节数组 | 转移 | Vec<u8> |
| `into_string()` | 字符串 | 转移 | String/Result |
| `into_inner()` | 解包 | 转移 | 内部值 |
| `into_boxed_slice()` | Box | 转移 | Box<[T]> |

### 快速参考

```rust
// 通用转换
let s: String = "hello".into();
let num: i64 = 42i32.into();

// 迭代器
for item in vec.into_iter() { }

// 字节
let bytes = string.into_bytes();

// 解包
let value = mutex.into_inner().unwrap();

// Box
let boxed = vec.into_boxed_slice();

// 自定义
impl From<A> for B { ... }
let b: B = a.into();
```

### 关键要点

1. **所有权转移** - into 后原值不可用
2. **实现 From** - 自动获得 Into
3. **类型推导** - into() 根据上下文推导
4. **迭代器选择** - iter() vs iter_mut() vs into_iter()
5. **零成本抽象** - 通常编译为高效代码

---

## 参考资源

- [Into trait 文档](https://doc.rust-lang.org/std/convert/trait.Into.html)
- [From trait 文档](https://doc.rust-lang.org/std/convert/trait.From.html)
- [IntoIterator trait 文档](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
- [所有权系统](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

---

**运行示例**:
```bash
cargo run --bin into_conversions_detailed
```
