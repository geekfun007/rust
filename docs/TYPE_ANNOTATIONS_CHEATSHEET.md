# Rust 类型标注速查卡 🚀

## 快速决策树

```
需要类型标注吗？
│
├─ 有字面量/明确函数返回？
│  └─ ❌ 不需要
│      let x = 5;
│      let s = String::from("hi");
│
├─ 延迟初始化？
│  └─ ✅ 需要
│      let x: i32;
│      x = 5;
│
├─ 使用 collect/parse？
│  └─ ✅ 需要
│      let v: Vec<i32> = iter.collect();
│      let n: i32 = s.parse()?;
│
├─ 空容器？
│  └─ ✅ 需要
│      let v: Vec<String> = Vec::new();
│
└─ 泛型类型？
   └─ ✅ 需要
       let p: Point<i32> = Point { x: 1, y: 2 };
```

## 一句话总结

### let 类型标注

| 情况 | 需要？ | 示例 |
|-----|--------|------|
| 字面量 | ❌ | `let x = 5;` |
| 延迟初始化 | ✅ | `let x: i32; x = 5;` |
| collect() | ✅ | `let v: Vec<i32> = iter.collect();` |
| parse() | ✅ | `let n: i32 = "42".parse()?;` |
| 空容器 | ✅ | `let v: Vec<i32> = Vec::new();` |

### 泛型声明

```rust
// 结构体
struct Point<T> { x: T, y: T }
let p: Point<i32> = Point { x: 1, y: 2 };

// 枚举
enum Option<T> { Some(T), None }
let x: Option<i32> = Some(5);

// 函数
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

// Turbofish
let v = iter.collect::<Vec<i32>>();
let n = "42".parse::<i32>()?;
```

## 两种标注方式对比

```rust
// 方式A：变量标注 (推荐：更清晰)
let numbers: Vec<i32> = data.iter().copied().collect();
let value: i32 = "42".parse().unwrap();

// 方式B：Turbofish (推荐：链式调用)
let numbers = data.iter().copied().collect::<Vec<i32>>();
let value = "42".parse::<i32>().unwrap();
```

## 记忆技巧

```
📌 五个必须标注的情况（记住首字母 DCEPM）:

D - Delay      延迟初始化
C - Collect    collect() 方法
E - Empty      空容器
P - Parse      parse() 方法
M - Multiple   多个实现（歧义）
```

## 常见错误速查

```rust
// ❌ 错误：cannot infer type
let v = Vec::new();
// ✅ 修复
let v: Vec<i32> = Vec::new();

// ❌ 错误：type annotations needed
let n = s.parse().unwrap();
// ✅ 修复
let n: i32 = s.parse().unwrap();

// ❌ 错误：collect 类型不明
let v = iter.collect();
// ✅ 修复
let v: Vec<_> = iter.collect();  // _ 让编译器推断内部类型
```

## 实用模板

### 模板1：读取配置文件

```rust
use std::collections::HashMap;

// 明确类型
let config: HashMap<String, String> = 
    read_config_file(path)?
    .lines()
    .filter_map(|line| {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    })
    .collect();
```

### 模板2：数据转换

```rust
// 字符串 -> 数字
let numbers: Vec<i32> = strings
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();

// 或使用 Turbofish
let numbers = strings
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<i32>>();
```

### 模板3：泛型容器

```rust
struct Cache<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Cache<K, V> 
where
    K: Eq + Hash,
{
    fn new() -> Self {
        Cache { data: HashMap::new() }
    }
}

// 使用
let cache: Cache<String, User> = Cache::new();
```

## 命令速查

```bash
# 运行完整示例
cargo run --example type_annotations

# 查看详细文档
cat docs/TYPE_ANNOTATIONS_GUIDE.md

# 检查类型（使用 rust-analyzer）
# 在 VS Code 中：光标悬停 -> 查看推断类型
```

---

**打印提示：** 建议打印此速查卡，放在桌面随时查阅！ 📄
