# Rust 核心概念速查表

## 所有权 (Ownership)

### 三大规则
1. 每个值都有一个所有者
2. 同时只能有一个所有者
3. 所有者离开作用域，值被丢弃

### 内存操作

```rust
// Copy (栈上复制)
let x = 5;
let y = x;  // x 仍然有效

// Move (转移所有权)
let s1 = String::from("hello");
let s2 = s1;  // s1 失效

// Clone (深拷贝)
let s1 = String::from("hello");
let s2 = s1.clone();  // s1 和 s2 都有效
```

### 类型分类

**Copy 类型** (栈上)
- 整数、浮点、布尔、字符
- 元组（元素都是 Copy）
- 固定数组（元素是 Copy）

**非 Copy 类型** (涉及堆)
- String, Vec, Box
- 包含非 Copy 字段的结构体

---

## 借用 (Borrowing)

### 借用规则

```
同一时刻，要么有：
1. 任意数量的不可变引用 (&T)
2. 一个可变引用 (&mut T)

引用必须总是有效（不能悬垂）
```

### 引用类型

```rust
// 不可变引用
let s = String::from("hello");
let r1 = &s;
let r2 = &s;  // OK：多个不可变引用

// 可变引用
let mut s = String::from("hello");
let r = &mut s;  // OK：唯一的可变引用
r.push_str(" world");
```

### 内存大小

| 类型 | 大小（64位） | 说明 |
|------|-------------|------|
| `&T` | 8 字节 | 瘦指针 |
| `&mut T` | 8 字节 | 瘦指针 |
| `&str` | 16 字节 | 胖指针 (ptr + len) |
| `&[T]` | 16 字节 | 胖指针 (ptr + len) |

---

## 切片 (Slice)

### 字符串切片

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"

// 语法糖
let slice = &s[..2];   // 等于 &s[0..2]
let slice = &s[3..];   // 等于 &s[3..len]
let slice = &s[..];    // 整个字符串
```

### 数组切片

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3];  // [2, 3]
```

### 切片结构

```
&str / &[T] 内存布局：
┌─────────┬─────────┐
│ ptr (8) │ len (8) │
└─────────┴─────────┘
总共 16 字节
```

---

## 生命周期 (Lifetime)

### 基本语法

```rust
// 函数签名
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 结构体
struct Excerpt<'a> {
    text: &'a str,
}

// 方法
impl<'a> Excerpt<'a> {
    fn text(&self) -> &str {  // 消除规则自动推导
        self.text
    }
}
```

### 消除规则

**规则 1**：每个引用参数有独立生命周期
```rust
fn foo(x: &i32, y: &i32)
// 推导为
fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
```

**规则 2**：单个输入生命周期赋给所有输出
```rust
fn foo(x: &i32) -> &i32
// 推导为
fn foo<'a>(x: &'a i32) -> &'a i32
```

**规则 3**：&self 的生命周期赋给所有输出
```rust
fn name(&self) -> &str
// 推导为
fn name<'a>(&'a self) -> &'a str
```

### 特殊生命周期

```rust
// 'static：整个程序期间有效
let s: &'static str = "hello";

// 生命周期约束
where T: 'a  // T 比 'a 活得久
where 'a: 'b // 'a 比 'b 活得久
```

---

## 类型大小参考

### 基本类型

| 类型 | 大小 | 对齐 |
|------|------|------|
| `i8/u8` | 1B | 1B |
| `i16/u16` | 2B | 2B |
| `i32/u32` | 4B | 4B |
| `i64/u64` | 8B | 8B |
| `i128/u128` | 16B | 16B |
| `f32` | 4B | 4B |
| `f64` | 8B | 8B |
| `bool` | 1B | 1B |
| `char` | 4B | 4B |

### 复合类型

| 类型 | 大小 | 组成 |
|------|------|------|
| `String` | 24B | ptr(8) + len(8) + cap(8) |
| `Vec<T>` | 24B | ptr(8) + len(8) + cap(8) |
| `&str` | 16B | ptr(8) + len(8) |
| `&[T]` | 16B | ptr(8) + len(8) |
| `Option<&T>` | 8B | 优化后 |
| `Box<T>` | 8B | 指针 |
| `&T` | 8B | 指针 |

---

## 性能最佳实践

### ✅ 推荐

```rust
// 1. 使用引用避免复制
fn process(data: &Vec<i32>) { }

// 2. 使用切片增加灵活性
fn sum(data: &[i32]) -> i32 { }

// 3. 预分配容量
let mut vec = Vec::with_capacity(100);

// 4. 使用迭代器链
data.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .sum()
```

### ❌ 避免

```rust
// 1. 不必要的克隆
fn process(data: Vec<i32>) { }  // 移动所有权

// 2. 频繁重新分配
let mut vec = Vec::new();
for _ in 0..1000 {
    vec.push(i);  // 可能多次重新分配
}

// 3. 过度使用 String
let s = String::from("hello");  // 堆分配
let s = "hello";  // 无需分配，使用 &str
```

---

## 常见错误速查

### 错误：值已移动
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // ❌ 错误

// 解决方案
let s2 = s1.clone();  // 或使用引用 &s1
```

### 错误：可变借用冲突
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // ❌ 错误
println!("{}", r1);

// 解决方案（NLL）
let r1 = &s;
println!("{}", r1);  // r1 最后使用
let r2 = &mut s;     // ✓ OK
```

### 错误：生命周期不足
```rust
fn dangle() -> &String {  // ❌ 错误
    let s = String::from("hello");
    &s  // s 在此处被丢弃
}

// 解决方案
fn no_dangle() -> String {  // 返回所有权
    String::from("hello")
}
```

---

## 检查清单

### 选择所有权还是借用？

```
需要修改数据？
  ├─ 是 → 使用 &mut T
  └─ 否 → 需要多个访问者？
      ├─ 是 → 使用 &T
      └─ 否 → 使用 T（所有权）
```

### 选择 String 还是 &str？

```
需要修改字符串？
  ├─ 是 → String
  └─ 否 → 需要拥有数据？
      ├─ 是 → String
      └─ 否 → &str
```

### 选择 Vec 还是数组？

```
大小在编译时已知？
  ├─ 是 → [T; N]
  └─ 否 → Vec<T>
```

---

## 编译器标志

```bash
# 查看详细错误
cargo build --verbose

# 优化构建
cargo build --release

# 检查代码
cargo check

# 格式化
cargo fmt

# Lint
cargo clippy

# 展开宏
cargo expand
```

---

## 调试技巧

```rust
// 打印类型
let x = 5;
println!("type: {}", std::any::type_name_of_val(&x));

// 打印大小
println!("size: {}", std::mem::size_of::<String>());

// 打印地址
let s = String::from("hello");
println!("ptr: {:p}", s.as_ptr());

// 调试打印
#[derive(Debug)]
struct Point { x: i32, y: i32 }
println!("{:?}", Point { x: 1, y: 2 });
```

---

**更多信息：**
- 详细文档：`docs/CORE_PRINCIPLES.md`
- 运行示例：`cargo run --example <name>`
- 官方文档：https://doc.rust-lang.org/
