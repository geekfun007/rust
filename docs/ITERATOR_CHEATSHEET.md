# Rust 迭代器速查表

## 核心概念

### Iterator Trait
```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### for 循环展开
```rust
for item in collection {
    // ...
}

// 展开为：
let mut iter = collection.into_iter();
loop {
    match iter.next() {
        Some(item) => { /* ... */ },
        None => break,
    }
}
```

---

## 三种迭代方式

| 方法 | 所有权 | 元素类型 | 原集合 |
|------|--------|----------|--------|
| `iter()` | 借用 | `&T` | 仍有效 |
| `iter_mut()` | 可变借用 | `&mut T` | 仍有效 |
| `into_iter()` | 移动 | `T` | 被消费 |

```rust
let v = vec![1, 2, 3];

for x in &v { }         // iter()
for x in &mut v { }     // iter_mut()
for x in v { }          // into_iter()
```

---

## 创建迭代器

```rust
// 从集合
vec.iter()
arr.iter()
map.keys()
map.values()

// 范围
(0..10)                 // 0 到 9
(0..=10)                // 0 到 10
(0..)                   // 无限

// 工厂函数
std::iter::empty()      // 空迭代器
std::iter::once(x)      // 单元素
std::iter::repeat(x)    // 重复
```

---

## 消费器（终结操作）

### 收集
```rust
.collect()              // 收集到集合
.partition(predicate)   // 分区
```

### 聚合
```rust
.sum()                  // 求和
.product()              // 求积
.fold(init, f)          // 折叠
.reduce(f)              // 归约
```

### 查找
```rust
.find(predicate)        // 查找元素
.position(predicate)    // 查找位置
.any(predicate)         // 任意满足
.all(predicate)         // 全部满足
.max()                  // 最大值
.min()                  // 最小值
```

### 计数
```rust
.count()                // 计数
.nth(n)                 // 第 n 个
.last()                 // 最后一个
```

### 执行
```rust
.for_each(f)            // 遍历
```

---

## 适配器（惰性操作）

### 转换
```rust
.map(f)                 // 映射
.filter_map(f)          // 过滤并映射
.flat_map(f)            // 扁平映射
.flatten()              // 扁平化
```

### 过滤
```rust
.filter(predicate)      // 过滤
.take(n)                // 取前 n 个
.take_while(predicate)  // 取直到条件不满足
.skip(n)                // 跳过前 n 个
.skip_while(predicate)  // 跳过直到条件不满足
```

### 组合
```rust
.chain(other)           // 连接
.zip(other)             // 配对
.enumerate()            // 添加索引
```

### 其他
```rust
.copied()               // 复制（Copy）
.cloned()               // 克隆（Clone）
.cycle()                // 循环
.rev()                  // 反转
.step_by(n)             // 步长
.inspect(f)             // 检查（调试）
.peekable()             // 可预览
```

---

## 常用模式

### 过滤和转换
```rust
data.iter()
    .filter(|&x| x > 0)
    .map(|x| x * 2)
    .collect()
```

### 查找
```rust
// 查找第一个
data.iter().find(|&&x| x > 10)

// 查找所有
data.iter().filter(|&&x| x > 10).collect()
```

### 聚合
```rust
// 求和
data.iter().sum::<i32>()

// 自定义聚合
data.iter().fold(0, |acc, x| acc + x)
```

### 分组
```rust
use std::collections::HashMap;

data.iter().fold(HashMap::new(), |mut acc, item| {
    *acc.entry(item.key).or_insert(0) += 1;
    acc
})
```

### 链式转换
```rust
data.iter()
    .filter(predicate1)
    .map(transform1)
    .filter(predicate2)
    .map(transform2)
    .collect()
```

### 笛卡尔积
```rust
v1.iter()
    .flat_map(|&x| v2.iter().map(move |&y| (x, y)))
    .collect()
```

### 窗口
```rust
// 滑动窗口
data.windows(n)

// 分块
data.chunks(n)
```

### 错误处理
```rust
// 过滤错误
results.iter().filter_map(|r| r.ok())

// 短路
results.into_iter().collect::<Result<Vec<_>, _>>()
```

---

## 自定义迭代器

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

## 性能提示

### ✅ 好
```rust
// 链式操作（零成本）
data.iter()
    .filter(...)
    .map(...)
    .collect()

// 单次分配
vec.with_capacity(size)
```

### ❌ 差
```rust
// 多次 collect（多次分配）
let v1 = data.iter().map(...).collect();
let v2 = v1.iter().filter(...).collect();

// 不必要的 collect
data.iter().map(...).collect::<Vec<_>>().iter().sum()
// 应该直接 .sum()
```

---

## 常见陷阱

### 1. 多次消费
```rust
let iter = vec.into_iter();
iter.sum();    // 消费了迭代器
iter.count();  // ❌ 错误！已被消费
```

### 2. 忘记 collect
```rust
let doubled = vec.iter().map(|x| x * 2);
// doubled 是迭代器，不是 Vec！
```

### 3. 类型推断
```rust
// ❌ 可能需要类型标注
let v = data.iter().collect();

// ✓ 明确类型
let v: Vec<_> = data.iter().collect();
```

---

## 决策树

### 选择迭代方式
```
需要修改元素？
├─ 是 → iter_mut()
└─ 否 → 需要获取所有权？
    ├─ 是 → into_iter()
    └─ 否 → iter()
```

### 选择方法
```
需要收集结果？
├─ 是 → collect()
└─ 否 → 需要聚合？
    ├─ 是 → sum/fold/reduce
    └─ 否 → 查找？
        ├─ 是 → find/any/all
        └─ 否 → 遍历 → for_each
```

---

## 复杂度

| 操作 | 时间 | 空间 | 备注 |
|------|------|------|------|
| `iter()` | O(1) | O(1) | 创建迭代器 |
| `next()` | O(1) | O(1) | 单次迭代 |
| `map/filter` | O(1) | O(1) | 惰性 |
| `collect()` | O(n) | O(n) | 需要分配 |
| `sum/count` | O(n) | O(1) | 单次遍历 |
| `chain` | O(1) | O(1) | 惰性 |
| `flatten` | O(1) | O(1) | 惰性 |

---

## 快速命令

```bash
# 运行示例
cargo run --example iterator_demo

# 运行测试
cargo test --example iterator_demo

# 查看文档
cat docs/ITERATOR_GUIDE.md
```

---

## 实用技巧

### 调试迭代器
```rust
data.iter()
    .inspect(|x| println!("处理: {:?}", x))
    .map(...)
    .collect()
```

### 限制无限迭代器
```rust
(0..)
    .take(10)
    .collect()
```

### 延迟计算
```rust
// 创建（不执行）
let iter = data.iter().map(expensive_fn);

// 只在需要时执行
let first_5: Vec<_> = iter.take(5).collect();
```

### 并行迭代（需要 rayon）
```rust
use rayon::prelude::*;

data.par_iter()
    .map(...)
    .collect()
```

---

**记住：迭代器是惰性的，只有在消费器被调用时才执行！**
