# Rust String & &str 完全指南

深入理解 Rust 中的字符串类型：String 和 &str 的区别、使用场景和最佳实践。

## 目录

1. [核心概念](#核心概念)
2. [String 详解](#string-详解)
3. [&str 详解](#str-详解)
4. [相互转换](#相互转换)
5. [使用场景](#使用场景)
6. [性能考虑](#性能考虑)
7. [函数参数设计](#函数参数设计)
8. [字符串操作](#字符串操作)
9. [常见陷阱](#常见陷阱)
10. [实战案例](#实战案例)
11. [最佳实践](#最佳实践)

---

## 核心概念

### String vs &str 对比

```
┌──────────────┬────────────────┬────────────────┐
│  特性         │  String        │  &str          │
├──────────────┼────────────────┼────────────────┤
│  所有权       │  拥有          │  借用          │
│  可变性       │  可变          │  不可变        │
│  内存         │  堆分配        │  静态/栈/堆    │
│  大小         │  动态          │  已知          │
│  修改         │  可以          │  不可以        │
│  生命周期     │  owned         │  borrowed      │
└──────────────┴────────────────┴────────────────┘
```

### 内存布局

**String**:
```
栈: [指针 | 长度 | 容量]  (24 字节, 64位系统)
     ↓
堆: [实际字符串数据...]
```

**&str**:
```
栈: [指针 | 长度]  (16 字节, 64位系统)
     ↓
数据: 指向某处的字符串切片
      (可能在静态区、栈、堆)
```

---

## String 详解

### 什么是 String？

- **可增长的、可变的、拥有所有权的 UTF-8 编码字符串**
- 存储在堆上
- 类似于 `Vec<u8>`，但保证有效 UTF-8

### 创建 String

```rust
// 方法 1: 空字符串
let mut s = String::new();

// 方法 2: 从字面量
let s = String::from("Hello");

// 方法 3: to_string()
let s = "World".to_string();

// 方法 4: to_owned()
let s = "Rust".to_owned();

// 方法 5: format!
let s = format!("Hello, {}!", "World");

// 方法 6: 预分配容量
let s = String::with_capacity(100);
```

### String 操作

#### 追加

```rust
let mut s = String::from("Hello");

// 追加字符串
s.push_str(", World");

// 追加字符
s.push('!');

println!("{}", s);  // "Hello, World!"
```

#### 插入

```rust
let mut s = String::from("Hello");

// 插入字符
s.insert(5, ' ');

// 插入字符串
s.insert_str(6, "Rust ");

println!("{}", s);  // "Hello Rust "
```

#### 替换

```rust
let s = String::from("Hello, World!");

// replace - 返回新字符串
let s2 = s.replace("World", "Rust");
println!("{}", s2);  // "Hello, Rust!"

// replace_range - 原地修改
let mut s3 = String::from("Hello, World!");
s3.replace_range(7..12, "Rust");
println!("{}", s3);  // "Hello, Rust!"
```

#### 删除

```rust
let mut s = String::from("Hello, World!");

// 清空
s.clear();

// 截断
let mut s = String::from("Hello, World!");
s.truncate(5);
println!("{}", s);  // "Hello"

// 删除字符
let mut s = String::from("Hello");
s.pop();  // 返回 Some('o')

// 删除范围
let mut s = String::from("Hello, World!");
s.drain(5..7);  // 删除 ", "
```

### 容量管理

```rust
let mut s = String::new();
println!("容量: {}", s.capacity());  // 0

// 预留容量
s.reserve(100);
println!("容量: {}", s.capacity());  // >= 100

// 精确预留
s.reserve_exact(200);

// 缩减到实际大小
s.shrink_to_fit();

// 检查容量
if s.capacity() > s.len() {
    println!("有剩余容量");
}
```

---

## &str 详解

### 什么是 &str？

- **字符串切片（slice）**
- 不可变引用
- 可以指向：
  - 静态字符串（`'static` 生命周期）
  - `String` 的一部分
  - 其他 `&str`

### &str 的来源

```rust
// 1. 字符串字面量（静态）
let s: &str = "Hello, World!";

// 2. String 的引用
let string = String::from("Hello");
let s: &str = &string;

// 3. String 的切片
let string = String::from("Hello, World!");
let s: &str = &string[0..5];  // "Hello"

// 4. 切片的切片
let s2: &str = &s[0..4];  // "Hell"
```

### &str 操作（只读）

```rust
let s = "Hello, Rust!";

// 基本信息
s.len()         // 12
s.is_empty()    // false
s.chars()       // 字符迭代器
s.bytes()       // 字节迭代器

// 搜索
s.contains("Rust")           // true
s.starts_with("Hello")       // true
s.ends_with("!")             // true
s.find("Rust")               // Some(7)

// 分割
s.split(',')                 // 迭代器
s.split_whitespace()         // 按空格分割
s.lines()                    // 按行分割

// 转换
s.to_uppercase()             // "HELLO, RUST!"
s.to_lowercase()             // "hello, rust!"
s.trim()                     // 去除首尾空格
s.replace("Rust", "World")   // 替换
```

---

## 相互转换

### &str → String

```rust
let s: &str = "hello";

// 方法 1: to_string() ✓ 推荐
let string1 = s.to_string();

// 方法 2: to_owned() ✓ 语义清晰
let string2 = s.to_owned();

// 方法 3: String::from() ✓ 显式
let string3 = String::from(s);

// 方法 4: into()
let string4: String = s.into();
```

### String → &str

```rust
let string = String::from("world");

// 方法 1: & 引用 ✓ 最简单
let s1: &str = &string;

// 方法 2: as_str() ✓ 显式
let s2: &str = string.as_str();

// 方法 3: 切片
let s3: &str = &string[..];
```

### 转换成本

```rust
// &str → String
// 成本：堆分配 + 复制数据
let s = "hello";
let string = s.to_string();  // 分配新内存

// String → &str
// 成本：零成本（只是借用）
let string = String::from("hello");
let s = &string;  // 不分配内存
```

---

## 使用场景

### 使用 String 的场景

✅ **需要拥有字符串所有权**

```rust
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // 返回拥有所有权的 String
}
```

✅ **需要修改字符串内容**

```rust
let mut text = String::from("Hello");
text.push_str(", World!");
```

✅ **动态构建字符串**

```rust
let mut result = String::new();
for i in 0..5 {
    result.push_str(&i.to_string());
}
```

✅ **存储在结构体中**

```rust
struct User {
    name: String,    // 拥有数据
    email: String,   // 拥有数据
}
```

✅ **从函数返回字符串**

```rust
fn get_name() -> String {
    String::from("Alice")
}
```

### 使用 &str 的场景

✅ **只读访问字符串**

```rust
fn print_message(msg: &str) {
    println!("{}", msg);
}
```

✅ **函数参数（推荐）**

```rust
// 好 - 灵活，可接受 &str 或 String
fn process(text: &str) { }

// 差 - 不灵活，只能接受 String
fn process_bad(text: String) { }
```

✅ **字符串字面量**

```rust
let greeting = "Hello, World!";  // &str
```

✅ **不需要所有权**

```rust
fn extract_first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
```

---

## 性能考虑

### 内存分配

```rust
// &str - 零成本
let s1 = "hello";        // 静态字符串
let s2 = s1;             // Copy，无分配
println!("{} {}", s1, s2);

// String - 堆分配
let s1 = String::from("hello");
let s2 = s1.clone();     // 分配新内存
println!("{}", s2);
```

### 克隆成本

| 类型 | 克隆成本 | 说明 |
|------|---------|------|
| `&str` | 零成本 | 只复制指针和长度（16字节） |
| `String` | 高成本 | 分配新内存 + 复制数据 |

### 性能优化

**1. 预分配容量**

```rust
// 差 - 多次重新分配
let mut s = String::new();
for i in 0..1000 {
    s.push_str("x");  // 可能触发多次重新分配
}

// 好 - 一次分配
let mut s = String::with_capacity(1000);
for i in 0..1000 {
    s.push_str("x");  // 不会重新分配
}
```

**2. 避免不必要的转换**

```rust
// 差
fn process(s: String) { }
let text = "hello";
process(text.to_string());  // 不必要的分配

// 好
fn process(s: &str) { }
let text = "hello";
process(text);  // 零成本
```

**3. 使用静态字符串**

```rust
// 运行时字符串
let s = String::from("Hello");

// 编译时字符串（更快）
const GREETING: &str = "Hello";
```

---

## 函数参数设计

### 参数类型选择

```
┌─────────────┬────────────┬────────────────┐
│  场景        │  推荐类型   │  原因          │
├─────────────┼────────────┼────────────────┤
│  只读        │  &str      │  最灵活        │
│  需要所有权  │  String    │  拥有数据      │
│  可能修改    │  &mut String│ 可变引用      │
└─────────────┴────────────┴────────────────┘
```

### 示例

**只读参数 - 使用 &str**

```rust
// ✅ 好 - 灵活
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 可以传入任何字符串类型
greet("Alice");                    // &str
greet(&String::from("Bob"));       // String
greet(&name[0..3]);                // 切片
```

**需要所有权 - 使用 String**

```rust
fn take_ownership(s: String) {
    println!("{}", s);
    // s 在这里被销毁
}

let owned = String::from("test");
take_ownership(owned);
// owned 不可再用
```

**修改参数 - 使用 &mut String**

```rust
fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

let mut text = String::from("Hello");
append_suffix(&mut text, ", World!");
println!("{}", text);  // "Hello, World!"
```

**返回值 - 使用 String**

```rust
// ✅ 好 - 返回拥有所有权的值
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

let greeting = create_greeting("Alice");
println!("{}", greeting);
```

---

## 字符串操作

### 拼接

```rust
// 方法 1: + 运算符
let s1 = String::from("Hello");
let s2 = String::from("World");
let s3 = s1 + " " + &s2;  // s1 被移动

// 方法 2: format! （推荐）
let s1 = String::from("Hello");
let s2 = String::from("World");
let s3 = format!("{} {}", s1, s2);  // 不移动

// 方法 3: push_str
let mut s = String::from("Hello");
s.push_str(" World");

// 方法 4: join
let words = vec!["Hello", "World"];
let sentence = words.join(" ");
```

### 切片

```rust
let s = "Hello, World!";

let hello = &s[0..5];      // "Hello"
let world = &s[7..12];     // "World"
let all = &s[..];          // "Hello, World!"

// 注意：必须在 UTF-8 字符边界上切片
let chinese = "你好世界";
let slice = &chinese[0..3];  // "你" (3 字节)
// let bad = &chinese[0..1];  // panic!
```

### 迭代

```rust
let s = "Hello";

// 字符迭代
for ch in s.chars() {
    println!("{}", ch);  // 'H', 'e', 'l', 'l', 'o'
}

// 字节迭代
for byte in s.bytes() {
    println!("{}", byte);  // 72, 101, 108, 108, 111
}

// 索引枚举
for (i, ch) in s.chars().enumerate() {
    println!("{}:{}", i, ch);
}
```

### 搜索和匹配

```rust
let text = "Hello, Rust Programming!";

// 查找
text.find("Rust")              // Some(7)
text.rfind('o')                // Some(14)

// 检查
text.contains("Rust")          // true
text.starts_with("Hello")      // true
text.ends_with("!")            // true

// 分割
let words: Vec<&str> = text.split(' ').collect();
let parts: Vec<&str> = text.split(',').collect();

// 匹配
match text {
    s if s.contains("Rust") => println!("Found Rust!"),
    _ => println!("Not found"),
}
```

---

## 常见陷阱

### ❌ 陷阱 1: 不能直接索引

```rust
let s = "Hello";

// ❌ 错误
// let ch = s[0];  // 编译错误！

// ✅ 正确
let ch = s.chars().nth(0).unwrap();  // 'H'
let slice = &s[0..1];                // "H"
```

**原因**: Rust 字符串是 UTF-8 编码，字符可能占用多个字节。

### ❌ 陷阱 2: UTF-8 边界

```rust
let s = "你好";

// ❌ 错误 - 不在字符边界
// let slice = &s[0..1];  // panic!

// ✅ 正确 - 一个中文字符 3 字节
let slice = &s[0..3];  // "你"
```

### ❌ 陷阱 3: 所有权转移

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 移动到 s2

// ❌ 错误
// println!("{}", s1);  // 编译错误！s1 已移动

// ✅ 正确
println!("{}", s2);
```

### ❌ 陷阱 4: 不必要的克隆

```rust
// ❌ 差 - 不必要的分配
fn process_bad(s: String) {
    println!("{}", s);
}

let text = String::from("test");
process_bad(text.clone());  // 克隆
process_bad(text);          // 移动

// ✅ 好 - 借用
fn process_good(s: &str) {
    println!("{}", s);
}

let text = String::from("test");
process_good(&text);  // 借用
process_good(&text);  // 可以多次使用
```

---

## 实战案例

### 案例 1: 配置管理器

```rust
struct Config {
    host: String,
    port: u16,
    api_key: String,
}

impl Config {
    // 参数使用 &str（灵活）
    fn new(host: &str, port: u16, api_key: &str) -> Self {
        Config {
            host: host.to_string(),
            port,
            api_key: api_key.to_string(),
        }
    }
    
    // 返回 &str（高效）
    fn get_host(&self) -> &str {
        &self.host
    }
    
    // 返回 String（拥有所有权）
    fn get_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
    
    // 可变方法
    fn set_host(&mut self, host: &str) {
        self.host = host.to_string();
    }
}

// 使用
let mut config = Config::new("localhost", 8080, "secret");
println!("URL: {}", config.get_url());
config.set_host("127.0.0.1");
```

### 案例 2: 日志系统

```rust
struct Logger {
    prefix: String,
    messages: Vec<String>,
}

impl Logger {
    fn new(prefix: &str) -> Self {
        Logger {
            prefix: prefix.to_string(),
            messages: Vec::new(),
        }
    }
    
    fn log(&mut self, level: &str, message: &str) {
        let entry = format!("[{}] {}: {}", self.prefix, level, message);
        self.messages.push(entry);
    }
    
    fn get_logs(&self) -> Vec<&str> {
        self.messages.iter().map(|s| s.as_str()).collect()
    }
    
    fn dump(&self) -> String {
        self.messages.join("\n")
    }
}

// 使用
let mut logger = Logger::new("APP");
logger.log("INFO", "Started");
logger.log("ERROR", "Failed");
println!("{}", logger.dump());
```

### 案例 3: URL 构建器

```rust
struct UrlBuilder {
    scheme: String,
    host: String,
    port: Option<u16>,
    path: String,
    query: Vec<(String, String)>,
}

impl UrlBuilder {
    fn new(host: &str) -> Self {
        UrlBuilder {
            scheme: "http".to_string(),
            host: host.to_string(),
            port: None,
            path: String::new(),
            query: Vec::new(),
        }
    }
    
    fn scheme(mut self, scheme: &str) -> Self {
        self.scheme = scheme.to_string();
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
    
    fn query(mut self, key: &str, value: &str) -> Self {
        self.query.push((key.to_string(), value.to_string()));
        self
    }
    
    fn build(&self) -> String {
        let mut url = format!("{}://{}", self.scheme, self.host);
        
        if let Some(port) = self.port {
            url.push_str(&format!(":{}", port));
        }
        
        if !self.path.is_empty() {
            url.push_str(&self.path);
        }
        
        if !self.query.is_empty() {
            url.push('?');
            let query: Vec<String> = self.query
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&query.join("&"));
        }
        
        url
    }
}

// 使用
let url = UrlBuilder::new("api.example.com")
    .scheme("https")
    .path("/v1/users")
    .query("page", "1")
    .query("limit", "10")
    .build();
```

---

## 最佳实践

### ✅ 推荐做法

**1. 函数参数优先使用 &str**

```rust
// ✅ 好
fn process(text: &str) { }

// ❌ 差
fn process(text: String) { }
```

**2. 返回值使用 String**

```rust
// ✅ 好
fn create() -> String {
    String::from("data")
}
```

**3. 结构体字段使用 String**

```rust
// ✅ 好
struct User {
    name: String,  // 拥有数据
}
```

**4. 预分配容量**

```rust
// ✅ 好
let mut s = String::with_capacity(100);
for i in 0..100 {
    s.push('x');
}
```

**5. 使用 format! 拼接**

```rust
// ✅ 好 - 不移动所有权
let result = format!("{} {}", s1, s2);

// ❌ 差 - 移动 s1
let result = s1 + " " + &s2;
```

### ❌ 避免做法

**1. 不必要的 to_string()**

```rust
// ❌ 差
fn process(s: &str) -> String {
    s.to_string().to_uppercase()  // 两次分配
}

// ✅ 好
fn process(s: &str) -> String {
    s.to_uppercase()  // 一次分配
}
```

**2. 不必要的克隆**

```rust
// ❌ 差
fn print(s: String) { println!("{}", s); }
print(text.clone());

// ✅ 好
fn print(s: &str) { println!("{}", s); }
print(&text);
```

**3. 频繁的小字符串拼接**

```rust
// ❌ 差 - 每次拼接都重新分配
let mut s = String::new();
s = s + "a";
s = s + "b";
s = s + "c";

// ✅ 好 - 使用 push_str
let mut s = String::new();
s.push_str("a");
s.push_str("b");
s.push_str("c");
```

---

## 快速参考

### 创建

```rust
String::new()                    // 空字符串
String::from("hello")            // 从字面量
"hello".to_string()              // 转换
String::with_capacity(100)       // 预分配
format!("Hello, {}!", name)      // 格式化
```

### 转换

```rust
// &str → String
s.to_string()
s.to_owned()
String::from(s)

// String → &str
&string
string.as_str()
&string[..]
```

### 操作

```rust
s.push_str("text")               // 追加字符串
s.push('c')                      // 追加字符
s.insert(i, 'c')                 // 插入字符
s.replace("old", "new")          // 替换
s.trim()                         // 去空格
s.split(',')                     // 分割
s.to_uppercase()                 // 大写
s.to_lowercase()                 // 小写
```

### 检查

```rust
s.len()                          // 长度
s.is_empty()                     // 是否为空
s.contains("text")               // 包含
s.starts_with("prefix")          // 前缀
s.ends_with("suffix")            // 后缀
```

---

## 总结

### 核心要点

1. **String** - 拥有所有权，可变，堆分配
2. **&str** - 借用，不可变，零成本
3. **参数用 &str，返回值用 String**
4. **结构体字段用 String**
5. **预分配容量提升性能**
6. **注意 UTF-8 边界**
7. **避免不必要的克隆**

### 选择指南

```
需要修改？        是 → String
                否 ↓
需要所有权？      是 → String
                否 ↓
函数参数？        是 → &str
                否 ↓
返回值？          是 → String
                否 ↓
默认             → &str
```

---

## 参考资源

- [Rust Book - Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [String 文档](https://doc.rust-lang.org/std/string/struct.String.html)
- [str 文档](https://doc.rust-lang.org/std/primitive.str.html)
- [UTF-8 Everywhere](http://utf8everywhere.org/)

---

**运行示例**:
```bash
cargo run --bin string_vs_str_detailed
```
