# Rust 核心类型速查表

## Box<T>

### 何时使用
```rust
// ✓ 递归类型
enum List { Cons(i32, Box<List>), Nil }

// ✓ 大型数据（避免栈溢出）
let large = Box::new([0u8; 1_000_000]);

// ✓ Trait 对象
let obj: Box<dyn Trait> = Box::new(impl);

// ✗ 小数据（不必要的堆分配）
let x = Box::new(5);  // 浪费
```

### 常用方法
```rust
Box::new(value)       // 创建
*boxed                // 解引用
Box::into_raw(b)      // 转为裸指针
Box::from_raw(ptr)    // 从裸指针创建
```

### 内存布局
- 栈：8 字节（指针）
- 堆：T 的大小
- 移动：只复制指针

---

## Option<T>

### 创建
```rust
Some(value)
None
```

### 常用方法

#### 检查
```rust
.is_some()            // 是否有值
.is_none()            // 是否无值
```

#### 提取
```rust
.unwrap()             // 有值返回，无值 panic
.expect("msg")        // 自定义 panic 消息
.unwrap_or(default)   // 提供默认值
.unwrap_or_default()  // 使用 Default
```

#### 转换
```rust
.map(f)               // 转换内部值
.and_then(f)          // 链式可能失败操作
.filter(predicate)    // 过滤
.ok_or(err)           // 转 Result
```

#### 组合
```rust
.and(other)           // 都有值返回第二个
.or(other)            // 有值返回第一个有值的
.xor(other)           // 异或
```

### 模式匹配
```rust
match opt {
    Some(x) => { ... },
    None => { ... },
}

if let Some(x) = opt { ... }
```

### ? 操作符
```rust
fn f() -> Option<T> {
    let x = opt?;  // None 时提前返回
    Some(x)
}
```

---

## Result<T, E>

### 创建
```rust
Ok(value)
Err(error)
```

### 常用方法

#### 检查
```rust
.is_ok()              // 是否成功
.is_err()             // 是否失败
```

#### 提取
```rust
.unwrap()             // 成功返回值，失败 panic
.expect("msg")        // 自定义 panic 消息
.unwrap_or(default)   // 提供默认值
.unwrap_err()         // 提取错误
```

#### 转换
```rust
.map(f)               // 转换 Ok 值
.map_err(f)           // 转换 Err 值
.and_then(f)          // 链式可能失败操作
.or_else(f)           // 错误恢复
.ok()                 // 转 Option
```

### ? 操作符
```rust
fn f() -> Result<T, E> {
    let x = result?;  // Err 时提前返回
    Ok(x)
}
```

### 错误传播
```rust
// 多个 ? 链式调用
fn op() -> Result<T, E> {
    let a = op1()?;
    let b = op2(a)?;
    let c = op3(b)?;
    Ok(c)
}
```

---

## fmt::Display

### 实现
```rust
use std::fmt;

impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "格式化输出: {}", self.value)
    }
}
```

### 格式化语法
```rust
{}                    // Display
{:?}                  // Debug
{:#?}                 // 美化 Debug

{:5}                  // 宽度 5
{:<5}                 // 左对齐
{:>5}                 // 右对齐
{:^5}                 // 居中
{:05}                 // 填充零

{:.2}                 // 2 位小数
{:e}                  // 科学计数

{:b}                  // 二进制
{:o}                  // 八进制
{:x}                  // 十六进制
{:X}                  // 十六进制大写
```

### 宏家族
```rust
format!(...)          // 创建 String
print!(...)           // 标准输出
println!(...)         // 标准输出 + 换行
eprint!(...)          // 标准错误
eprintln!(...)        // 标准错误 + 换行
```

---

## 其他核心 Trait

### From & Into
```rust
impl From<T> for U {
    fn from(value: T) -> Self { ... }
}

let u = U::from(t);
let u: U = t.into();  // Into 自动实现
```

### Default
```rust
#[derive(Default)]
struct Config {
    value: i32,  // 默认 0
}

let c = Config::default();
```

### Clone
```rust
#[derive(Clone)]
struct Data { ... }

let d2 = d1.clone();  // 深拷贝
```

### Copy
```rust
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

let p2 = p1;  // 复制，p1 仍有效
```

### Drop
```rust
impl Drop for MyType {
    fn drop(&mut self) {
        // 清理代码
    }
}
```

---

## 决策树

### 选择 Box 还是普通类型？
```
需要递归类型？
  ├─ 是 → Box
  └─ 否 → 数据很大（>1KB）？
      ├─ 是 → Box
      └─ 否 → 需要 Trait 对象？
          ├─ 是 → Box
          └─ 否 → 普通类型
```

### 选择 Option 还是 Result？
```
失败时需要知道原因？
  ├─ 是 → Result
  └─ 否 → Option
```

### 选择 Display 还是 Debug？
```
输出给谁看？
  ├─ 用户 → Display
  └─ 开发者 → Debug
```

---

## 性能对比

| 操作 | 开销 |
|------|------|
| Box::new | 堆分配（~100 cycles） |
| Box 移动 | 8 字节复制 |
| Option | +1 字节（tag） |
| Option<&T> | 0 字节（空指针优化） |
| Result | +1 字节（tag） |
| Display | 虚函数调用 |
| Clone | O(n)（深拷贝） |
| Copy | O(1)（按位复制） |

---

## 常见模式

### Option 链式调用
```rust
input
    .filter(|x| x > 0)
    .map(|x| x * 2)
    .and_then(|x| Some(x + 1))
```

### Result 错误传播
```rust
fn f() -> Result<T, E> {
    let a = op1()?;
    let b = op2()?;
    Ok(process(a, b))
}
```

### 自定义 Display
```rust
impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyType({})", self.value)
    }
}
```

---

**快速命令：**
```bash
# 运行示例
cargo run --example core_types

# 查看详细文档
cat docs/CORE_TYPES.md
```
