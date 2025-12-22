# Rust Derive 宏完全指南

## 目录
- [什么是 Derive](#什么是-derive)
- [常用 Derive 宏详解](#常用-derive-宏详解)
- [Derive 组合模式](#derive-组合模式)
- [手动实现 vs Derive](#手动实现-vs-derive)
- [决策树](#决策树)
- [最佳实践](#最佳实践)
- [常见问题](#常见问题)

---

## 什么是 Derive

**Derive 宏**是 Rust 提供的一种自动生成 trait 实现的机制。

### 核心概念

```rust
// 手动实现需要大量样板代码
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point { /* 10+ 行代码 */ }
impl Clone for Point { /* 5+ 行代码 */ }
impl PartialEq for Point { /* 5+ 行代码 */ }

// 使用 Derive，一行搞定
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

### 语法

```rust
#[derive(Trait1, Trait2, Trait3)]
struct MyType { ... }

#[derive(Trait1, Trait2)]
enum MyEnum { ... }
```

---

## 常用 Derive 宏详解

### 1. Debug

**用途**: 调试输出，支持 `{:?}` 和 `{:#?}` 格式化

```rust
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

let user = User { id: 1, name: "Alice".to_string() };

// {:?} - 单行输出
println!("{:?}", user);
// User { id: 1, name: "Alice" }

// {:#?} - 格式化输出
println!("{:#?}", user);
// User {
//     id: 1,
//     name: "Alice",
// }
```

**要点**:
- 几乎所有类型都应该 derive Debug
- 嵌套结构自动递归处理
- 可以自定义实现特殊格式

---

### 2. Clone

**用途**: 显式深拷贝

```rust
#[derive(Clone)]
struct Config {
    settings: Vec<String>,
}

let config1 = Config {
    settings: vec!["a".to_string()],
};

let config2 = config1.clone(); // 显式调用 .clone()
```

**特点**:
- 深拷贝所有字段
- 需要显式调用 `.clone()`
- 可能开销较大（堆分配）
- 允许包含 `String`、`Vec` 等堆类型

**对比**:
| 特性 | Clone | Copy |
|------|-------|------|
| 调用方式 | 显式 `.clone()` | 自动 |
| 开销 | 可能大 | 小（按位复制） |
| 堆分配 | 允许 | 不允许 |
| 使用后原值 | 有效 | 有效 |

---

### 3. Copy

**用途**: 自动按位复制（栈上小型值）

```rust
#[derive(Clone, Copy)]  // Copy 必须同时 derive Clone
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // 自动复制，无需 .clone()
println!("{:?} {:?}", p1, p2);  // p1 仍然有效
```

**限制**:
✅ 所有字段都必须实现 `Copy`  
✅ 不能包含堆分配（`String`、`Vec`、`Box` 等）  
✅ 必须同时 derive `Clone`  

**适用类型**:
- 基本类型：`i32`, `f64`, `bool`, `char`
- 数组：`[T; N]` 其中 `T: Copy`
- 元组：`(T, U)` 其中 `T, U: Copy`
- 简单结构体（只含上述类型）

---

### 4. PartialEq

**用途**: 支持 `==` 和 `!=` 运算符

```rust
#[derive(PartialEq)]
struct User {
    id: u32,
    name: String,
}

let u1 = User { id: 1, name: "Alice".to_string() };
let u2 = User { id: 1, name: "Alice".to_string() };

assert!(u1 == u2);  // true
```

**行为**:
- 逐字段比较所有字段
- 短路求值（第一个不等就返回）
- 可以自定义实现特殊逻辑

**自定义示例**:
```rust
// 大小写不敏感的字符串比较
struct CaseInsensitive(String);

impl PartialEq for CaseInsensitive {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}
```

---

### 5. Eq

**用途**: 标记类型满足完全相等关系

```rust
#[derive(PartialEq, Eq)]  // Eq 需要 PartialEq
struct Id(u32);
```

**要求**:
- 必须先实现 `PartialEq`
- 相等关系具有自反性：`a == a` 总是 `true`
- 用于某些泛型约束

**PartialEq vs Eq**:
| Trait | 自反性 | 例子 |
|-------|--------|------|
| PartialEq | 不保证 | `f64`（`NaN != NaN`） |
| Eq | 保证 | `i32`, `String` |

---

### 6. PartialOrd

**用途**: 支持 `<`, `<=`, `>`, `>=` 运算符

```rust
#[derive(PartialEq, PartialOrd)]
struct Score(u32);

assert!(Score(100) < Score(200));
```

**要求**:
- 必须先实现 `PartialEq`
- 按字段声明顺序比较（字典序）

---

### 7. Ord

**用途**: 完全排序，支持 `.sort()`

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Priority {
    level: u32,
    name: String,
}

let mut items = vec![
    Priority { level: 2, name: "B".to_string() },
    Priority { level: 1, name: "A".to_string() },
];

items.sort();  // 按 level 排序，再按 name
```

**要求**:
- 必须实现 `Eq` + `PartialOrd`
- 所有字段都必须实现 `Ord`

---

### 8. Hash

**用途**: 用作 `HashMap`/`HashSet` 的键

```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct ProductId {
    category: String,
    sku: u32,
}

let mut inventory = HashMap::new();
inventory.insert(
    ProductId { category: "Electronics".to_string(), sku: 123 },
    100,
);
```

**要求**:
- 必须同时实现 `Eq`
- 相等的值必须有相同的哈希

**不变性原则**:
```rust
// ⚠️ 错误：修改了用作键的值
let key = ProductId { ... };
map.insert(key, value);
key.sku = 999;  // 破坏了哈希映射！
```

---

### 9. Default

**用途**: 提供类型的默认值

```rust
#[derive(Default)]
struct Config {
    host: String,       // ""
    port: u16,          // 0
    debug: bool,        // false
}

let config = Config::default();

// 结构体更新语法
let custom = Config {
    port: 8080,
    debug: true,
    ..Default::default()
};
```

**默认值规则**:
| 类型 | 默认值 |
|------|--------|
| 数值 | `0` |
| `bool` | `false` |
| `String` | `""` |
| `Vec` | `[]` |
| `Option` | `None` |

**自定义默认值**:
```rust
struct Server {
    workers: usize,
}

impl Default for Server {
    fn default() -> Self {
        Server { workers: 4 }  // 自定义
    }
}
```

---

## Derive 组合模式

### 1. 基础组合（最常用）
```rust
#[derive(Debug, Clone, PartialEq)]
struct Basic {
    value: i32,
}
```
**适用**: 90% 的普通结构体

---

### 2. 可排序的类型
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sortable {
    priority: u32,
}
```
**适用**: 需要排序的类型（任务优先级、时间戳等）

---

### 3. HashMap 键
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    id: u32,
}
```
**适用**: 用作 HashMap 键

---

### 4. 简单值类型（Copy）
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```
**适用**: 小型栈上值（坐标、颜色、ID等）

---

### 5. 完整功能
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Complete {
    id: u32,
}
```
**适用**: 需要全部功能的简单类型

---

### 6. 配置类型
```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct AppConfig {
    host: String,
    port: u16,
}
```
**适用**: 配置、设置类

---

## 手动实现 vs Derive

### 何时使用 Derive

✅ 标准行为足够  
✅ 所有字段都参与  
✅ 快速原型  

### 何时手动实现

✅ 自定义行为  
✅ 忽略某些字段  
✅ 性能优化  
✅ 特殊逻辑  

### 示例：忽略字段

```rust
#[derive(Debug)]
struct Record {
    id: u32,
    data: String,
    timestamp: u64,  // 忽略此字段
}

// 手动实现，仅比较 id 和 data
impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data
        // 忽略 timestamp
    }
}
```

### 示例：自定义 Debug

```rust
struct ApiKey(String);

impl Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiKey[***{}]", &self.0[self.0.len()-4..])
        // 只显示最后 4 位
    }
}
```

---

## 决策树

```
需要调试输出？
├─ 是 → derive(Debug)
└─ 否 → 跳过

需要复制数据？
├─ 包含堆分配（String, Vec）？
│  ├─ 是 → derive(Clone)
│  └─ 否 → derive(Clone, Copy)
└─ 否 → 跳过

需要比较相等？
├─ 是 → derive(PartialEq)
│      └─ 满足自反性？
│         └─ 是 → derive(Eq)
└─ 否 → 跳过

需要排序？
├─ 是 → derive(PartialOrd)
│      └─ 完全排序？
│         └─ 是 → derive(Ord)
└─ 否 → 跳过

用作 HashMap 键？
├─ 是 → derive(Hash, Eq, PartialEq)
└─ 否 → 跳过

需要默认值？
├─ 是 → derive(Default)
└─ 否 → 跳过
```

---

## 最佳实践

### 1. 总是 derive Debug
```rust
// ✅ 好
#[derive(Debug)]
struct MyType { ... }

// ❌ 坏 - 无法调试
struct MyType { ... }
```

### 2. 遵循依赖关系
```rust
// ✅ 正确顺序
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

// ❌ 错误 - Ord 需要 Eq + PartialOrd
#[derive(Ord)]  // 编译错误！
```

### 3. Copy 的使用原则
```rust
// ✅ 好 - 小型栈值
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

// ❌ 坏 - 包含堆分配
#[derive(Copy, Clone)]  // 编译错误！
struct User { name: String }

// ❌ 坏 - 大型结构
#[derive(Copy, Clone)]
struct HugeData { data: [u8; 10000] }  // 复制开销大
```

### 4. 枚举的 Derive
```rust
// ✅ 枚举也可以 derive
#[derive(Debug, Clone, PartialEq)]
enum Status {
    Pending,
    Active { id: u32 },
    Completed,
}
```

### 5. 泛型的 Derive
```rust
// ✅ 泛型自动传播 trait 约束
#[derive(Debug, Clone)]
struct Wrapper<T> {
    value: T,
}
// 相当于：
// impl<T: Debug> Debug for Wrapper<T>
// impl<T: Clone> Clone for Wrapper<T>
```

### 6. 新类型模式
```rust
// ✅ 使用 derive 为新类型快速添加功能
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Email(String);
```

---

## 常见问题

### Q1: Copy 和 Clone 有什么区别？

**Copy**:
- 自动复制（隐式）
- 按位复制（栈上）
- 不能包含堆分配
- 开销小

**Clone**:
- 显式调用 `.clone()`
- 深拷贝（可能涉及堆）
- 允许堆分配
- 开销可能大

```rust
// Copy
let a = 5;
let b = a;  // 自动复制
println!("{}", a);  // OK

// Clone
let s1 = String::from("hello");
let s2 = s1.clone();  // 显式
println!("{}", s1);  // OK
```

---

### Q2: 为什么 Ord 需要 Eq？

因为完全排序要求：
- 如果 `a <= b` 且 `b <= a`，则 `a == b`

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]  // 正确顺序
struct Item { ... }
```

---

### Q3: 什么时候不应该 derive？

1. **需要自定义逻辑**
```rust
// 忽略大小写的比较
impl PartialEq for MyString {
    fn eq(&self, other: &Self) -> bool {
        self.to_lowercase() == other.to_lowercase()
    }
}
```

2. **性能优化**
```rust
// 只比较 ID，忽略其他字段
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
```

3. **保护敏感信息**
```rust
// 不显示密码
impl Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Credentials[username={}]", self.username)
    }
}
```

---

### Q4: Derive 的性能如何？

- **编译时零开销**：derive 在编译时展开，无运行时开销
- **生成的代码效率**：与手写相当
- **Debug 输出**：可能较慢（字符串格式化）
- **Clone**：取决于数据结构大小

---

### Q5: 可以为外部类型 derive 吗？

❌ 不能直接 derive  
✅ 使用新类型模式（Newtype Pattern）

```rust
// ❌ 不能为 Vec<T> 添加 derive
// derive(MyTrait)
// Vec<T> { ... }  // 编译错误

// ✅ 使用新类型
#[derive(Debug, MyTrait)]
struct MyVec<T>(Vec<T>);
```

---

### Q6: derive 的顺序重要吗？

对编译器来说不重要，但建议按约定排序以提高可读性：

```rust
// ✅ 推荐顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]

// 分组理解：
// 1. 基础：Debug
// 2. 复制：Clone, Copy
// 3. 相等：PartialEq, Eq
// 4. 排序：PartialOrd, Ord
// 5. 哈希：Hash
// 6. 默认：Default
```

---

## 快速参考表

| Derive | 用途 | 依赖 | 常见场景 |
|--------|------|------|----------|
| `Debug` | 调试输出 `{:?}` | - | 所有类型 |
| `Clone` | 深拷贝 | - | 需要复制的类型 |
| `Copy` | 自动复制 | `Clone` | 小型栈值 |
| `PartialEq` | `==`, `!=` | - | 比较相等 |
| `Eq` | 完全相等 | `PartialEq` | 可哈希类型 |
| `PartialOrd` | `<`, `>` | `PartialEq` | 可比较大小 |
| `Ord` | 完全排序 | `Eq`, `PartialOrd` | 排序 |
| `Hash` | 哈希 | `Eq` | HashMap 键 |
| `Default` | 默认值 | - | 配置、初始化 |

---

## 总结

1. **Derive 是 Rust 的超能力**：减少样板代码，提高生产力

2. **常用组合**：
   - 基础：`Debug, Clone, PartialEq`
   - 可排序：`+ Eq, PartialOrd, Ord`
   - HashMap 键：`+ Eq, Hash`
   - 简单值：`+ Copy`

3. **遵循依赖**：
   - `Ord` → `Eq` + `PartialOrd`
   - `Eq` → `PartialEq`
   - `Copy` → `Clone`
   - `Hash` → `Eq`

4. **何时手动实现**：
   - 自定义逻辑
   - 性能优化
   - 忽略字段
   - 保护敏感信息

5. **最佳实践**：
   - 总是 derive `Debug`
   - 小心使用 `Copy`（仅小型栈值）
   - 遵循标准顺序
   - 为新类型适当添加 derive

---

**运行示例代码**:
```bash
cargo run --bin derive_macros_detailed
```
