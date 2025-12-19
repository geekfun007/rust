# Rust 迭代器详解

## 目录
1. [Iterator Trait 核心](#1-iterator-trait-核心)
2. [for 循环底层原理](#2-for-循环底层原理)
3. [创建迭代器](#3-创建迭代器)
4. [消费器方法](#4-消费器方法)
5. [适配器方法](#5-适配器方法)
6. [自定义迭代器](#6-自定义迭代器)
7. [性能分析](#7-性能分析)
8. [实战模式](#8-实战模式)

---

## 1. Iterator Trait 核心

### 1.1 基本定义

```rust
pub trait Iterator {
    type Item;  // 关联类型：迭代的元素类型
    
    fn next(&mut self) -> Option<Self::Item>;  // 核心方法
    
    // ... 还有约 70 个默认实现的方法
}
```

### 1.2 工作原理

```
迭代器是惰性的（Lazy）：

创建     适配器链     消费器
 │         │           │
 ├──> map()────> filter()────> collect()
 │    不执行    不执行      触发执行
 │
 └─> 只有在消费器被调用时，整个链才开始执行

内存布局：
┌──────────────┐
│ Iterator     │
│ ┌──────────┐ │
│ │ 内部状态 │ │  例如：当前索引、剩余元素
│ └──────────┘ │
└──────────────┘
```

### 1.3 三种迭代方式

```rust
let v = vec![1, 2, 3];

// 1. into_iter() - 消耗集合，获取所有权
for item in v.into_iter() {
    println!("{}", item);  // item: i32
}
// v 已被移动，不能再使用

// 2. iter() - 不可变借用
let v = vec![1, 2, 3];
for item in v.iter() {
    println!("{}", item);  // item: &i32
}
// v 仍然有效

// 3. iter_mut() - 可变借用
let mut v = vec![1, 2, 3];
for item in v.iter_mut() {
    *item *= 2;  // item: &mut i32
}
// v = [2, 4, 6]
```

### 1.4 内存布局对比

```
Vec<i32>: [1, 2, 3]
│
├─ into_iter()
│  ┌──────────────────┐
│  │ Vec 内部状态     │ 移动整个 Vec
│  │ 当前位置: 0      │
│  └──────────────────┘
│
├─ iter()
│  ┌──────────────────┐
│  │ ptr: &Vec        │ 只借用引用
│  │ 当前位置: 0      │
│  └──────────────────┘
│
└─ iter_mut()
   ┌──────────────────┐
   │ ptr: &mut Vec    │ 可变借用
   │ 当前位置: 0      │
   └──────────────────┘
```

---

## 2. for 循环底层原理

### 2.1 语法糖展开

```rust
// 源代码
for item in collection {
    // 使用 item
}

// 展开后（简化）
{
    let mut iter = IntoIterator::into_iter(collection);
    loop {
        match iter.next() {
            Some(item) => {
                // 使用 item
            },
            None => break,
        }
    }
}
```

### 2.2 IntoIterator Trait

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}

// Vec 的三种实现
impl<T> IntoIterator for Vec<T> {
    // into_iter() - 消耗
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    // iter() - 不可变借用
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    // iter_mut() - 可变借用
}
```

### 2.3 自动选择

```rust
let v = vec![1, 2, 3];

// 根据上下文自动选择
for x in v { }          // 调用 v.into_iter()
for x in &v { }         // 调用 (&v).into_iter()
for x in &mut v { }     // 调用 (&mut v).into_iter()
```

---

## 3. 创建迭代器

### 3.1 从集合创建

```rust
// Vec
let v = vec![1, 2, 3];
let iter = v.iter();

// 数组
let arr = [1, 2, 3];
let iter = arr.iter();

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("a", 1);

for (key, value) in &map { }        // 迭代键值对
for key in map.keys() { }           // 只迭代键
for value in map.values() { }       // 只迭代值
for value in map.values_mut() { }   // 可变迭代值
```

### 3.2 范围迭代器

```rust
// 范围
for i in 0..5 { }        // 0, 1, 2, 3, 4
for i in 0..=5 { }       // 0, 1, 2, 3, 4, 5

// 无限迭代器（需要 take 限制）
let iter = (0..).take(5);  // 0, 1, 2, 3, 4

// 步长
for i in (0..10).step_by(2) { }  // 0, 2, 4, 6, 8
```

### 3.3 工厂函数

```rust
// 重复元素
let iter = std::iter::repeat(5).take(3);  // 5, 5, 5

// 重复函数调用
let iter = std::iter::repeat_with(|| rand::random::<u32>());

// 空迭代器
let iter: std::iter::Empty<i32> = std::iter::empty();

// 单元素迭代器
let iter = std::iter::once(42);

// 从函数生成
let iter = std::iter::from_fn(|| Some(42));

// 成功迭代器
let results = vec![Ok(1), Err("error"), Ok(2)];
let iter = results.iter().filter_map(|r| r.as_ref().ok());
```

---

## 4. 消费器方法

**消费器（Consumer）**：消耗迭代器，产生最终结果。

### 4.1 收集结果

```rust
let v = vec![1, 2, 3];

// collect - 收集到集合
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// 收集到不同类型
let set: HashSet<i32> = v.into_iter().collect();
let btree: BTreeSet<i32> = v.into_iter().collect();

// partition - 分区
let (even, odd): (Vec<i32>, Vec<i32>) = 
    (1..10).partition(|x| x % 2 == 0);
```

### 4.2 聚合操作

```rust
let v = vec![1, 2, 3, 4, 5];

// sum - 求和
let sum: i32 = v.iter().sum();  // 15

// product - 求积
let product: i32 = v.iter().product();  // 120

// fold - 折叠（左折叠）
let sum = v.iter().fold(0, |acc, x| acc + x);

// reduce - 类似 fold，但返回 Option
let sum = v.iter().copied().reduce(|acc, x| acc + x);  // Some(15)

// scan - 带状态的 map
let iter = v.iter().scan(0, |state, x| {
    *state += x;
    Some(*state)
});
// 产生：1, 3, 6, 10, 15（累加和）
```

### 4.3 查找操作

```rust
let v = vec![1, 2, 3, 4, 5];

// find - 查找第一个满足条件的元素
let found = v.iter().find(|&&x| x > 3);  // Some(&4)

// position - 查找位置
let pos = v.iter().position(|&x| x == 3);  // Some(2)

// any - 是否有元素满足条件
let has_even = v.iter().any(|&x| x % 2 == 0);  // true

// all - 是否所有元素满足条件
let all_positive = v.iter().all(|&x| x > 0);  // true

// max/min - 最大/最小值
let max = v.iter().max();  // Some(&5)
let min = v.iter().min();  // Some(&1)

// max_by/min_by - 自定义比较
let max = v.iter().max_by(|a, b| a.cmp(b));

// max_by_key/min_by_key - 按键比较
let max = v.iter().max_by_key(|&&x| x);
```

### 4.4 计数与比较

```rust
let v = vec![1, 2, 3, 4, 5];

// count - 计数
let count = v.iter().count();  // 5

// nth - 获取第 n 个元素
let third = v.iter().nth(2);  // Some(&3)

// last - 获取最后一个元素
let last = v.iter().last();  // Some(&5)

// eq - 比较两个迭代器
let v2 = vec![1, 2, 3, 4, 5];
let equal = v.iter().eq(v2.iter());  // true
```

### 4.5 执行操作

```rust
let v = vec![1, 2, 3];

// for_each - 对每个元素执行操作
v.iter().for_each(|x| println!("{}", x));

// try_for_each - 可短路的 for_each
let result: Result<(), &str> = v.iter().try_for_each(|&x| {
    if x > 0 {
        Ok(())
    } else {
        Err("负数")
    }
});
```

---

## 5. 适配器方法

**适配器（Adapter）**：转换迭代器，返回新的迭代器（惰性）。

### 5.1 转换元素

```rust
let v = vec![1, 2, 3];

// map - 转换每个元素
let doubled = v.iter().map(|x| x * 2);

// map_while - 映射直到遇到 None
let iter = (0..).map_while(|x| {
    if x < 3 { Some(x * 2) } else { None }
});  // 0, 2, 4

// filter_map - 过滤并转换
let parsed: Vec<i32> = vec!["1", "two", "3"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();  // [1, 3]

// flat_map - 扁平化映射
let nested = vec![vec![1, 2], vec![3, 4]];
let flattened: Vec<i32> = nested
    .iter()
    .flat_map(|v| v.iter())
    .copied()
    .collect();  // [1, 2, 3, 4]

// flatten - 扁平化
let nested = vec![vec![1, 2], vec![3, 4]];
let flattened: Vec<&i32> = nested
    .iter()
    .flatten()
    .collect();
```

### 5.2 过滤元素

```rust
let v = vec![1, 2, 3, 4, 5];

// filter - 过滤元素
let even: Vec<i32> = v.iter()
    .copied()
    .filter(|&x| x % 2 == 0)
    .collect();  // [2, 4]

// take - 取前 n 个元素
let first_three: Vec<i32> = v.iter()
    .copied()
    .take(3)
    .collect();  // [1, 2, 3]

// take_while - 取元素直到条件不满足
let taken: Vec<i32> = v.iter()
    .copied()
    .take_while(|&x| x < 4)
    .collect();  // [1, 2, 3]

// skip - 跳过前 n 个元素
let skipped: Vec<i32> = v.iter()
    .copied()
    .skip(2)
    .collect();  // [3, 4, 5]

// skip_while - 跳过元素直到条件不满足
let skipped: Vec<i32> = v.iter()
    .copied()
    .skip_while(|&x| x < 3)
    .collect();  // [3, 4, 5]
```

### 5.3 组合迭代器

```rust
let v1 = vec![1, 2, 3];
let v2 = vec![4, 5, 6];

// chain - 连接两个迭代器
let chained: Vec<i32> = v1.iter()
    .chain(v2.iter())
    .copied()
    .collect();  // [1, 2, 3, 4, 5, 6]

// zip - 配对两个迭代器
let zipped: Vec<(i32, i32)> = v1.iter()
    .zip(v2.iter())
    .map(|(&a, &b)| (a, b))
    .collect();  // [(1, 4), (2, 5), (3, 6)]

// enumerate - 添加索引
let enumerated: Vec<(usize, i32)> = v1.iter()
    .copied()
    .enumerate()
    .collect();  // [(0, 1), (1, 2), (2, 3)]
```

### 5.4 复制与克隆

```rust
let v = vec![1, 2, 3];

// copied - 复制元素（需要 Copy）
let copied: Vec<i32> = v.iter().copied().collect();

// cloned - 克隆元素（需要 Clone）
let cloned: Vec<i32> = v.iter().cloned().collect();
```

### 5.5 检查与窥视

```rust
let v = vec![1, 2, 3];

// inspect - 检查每个元素（调试用）
let sum: i32 = v.iter()
    .copied()
    .inspect(|x| println!("处理: {}", x))
    .sum();

// peekable - 可以预览下一个元素
let mut iter = v.iter().peekable();
while let Some(&x) = iter.peek() {
    println!("下一个是: {}", x);
    iter.next();
}
```

### 5.6 循环与反转

```rust
let v = vec![1, 2, 3];

// cycle - 无限循环迭代器
let cycled: Vec<i32> = v.iter()
    .copied()
    .cycle()
    .take(7)
    .collect();  // [1, 2, 3, 1, 2, 3, 1]

// rev - 反转迭代器
let reversed: Vec<i32> = v.iter()
    .copied()
    .rev()
    .collect();  // [3, 2, 1]
```

---

## 6. 自定义迭代器

### 6.1 简单计数器

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
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

// 使用
let counter = Counter::new(5);
for n in counter {
    println!("{}", n);  // 1, 2, 3, 4, 5
}
```

### 6.2 斐波那契数列

```rust
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

// 使用
let fib = Fibonacci::new().take(10);
for n in fib {
    print!("{} ", n);  // 0 1 1 2 3 5 8 13 21 34
}
```

### 6.3 范围步长迭代器

```rust
struct StepRange {
    current: i32,
    end: i32,
    step: i32,
}

impl StepRange {
    fn new(start: i32, end: i32, step: i32) -> Self {
        StepRange {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for StepRange {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

// 使用
for n in StepRange::new(0, 10, 2) {
    print!("{} ", n);  // 0 2 4 6 8
}
```

### 6.4 实现 IntoIterator

```rust
struct MyCollection {
    data: Vec<i32>,
}

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// 为引用实现
impl<'a> IntoIterator for &'a MyCollection {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

// 使用
let collection = MyCollection {
    data: vec![1, 2, 3],
};

for item in &collection {
    println!("{}", item);
}
```

---

## 7. 性能分析

### 7.1 零成本抽象

```rust
// 高级写法
let sum: i32 = (1..1000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum();

// 等价的低级写法
let mut sum = 0;
for x in 1..1000 {
    if x % 2 == 0 {
        sum += x * 2;
    }
}

// 编译后性能完全相同！
```

### 7.2 内联优化

```
编译过程：
1. 高级迭代器代码
   ↓
2. 编译器内联所有迭代器方法
   ↓
3. LLVM 优化（常量折叠、循环展开等）
   ↓
4. 生成高效机器码（可能使用 SIMD）

结果：与手写循环性能相同或更好
```

### 7.3 性能对比

```
操作                  时间复杂度    空间复杂度    备注
─────────────────────────────────────────────────
for 循环              O(n)          O(1)          基准
迭代器链              O(n)          O(1)          零成本
collect()             O(n)          O(n)          需要分配
filter/map            O(1)          O(1)          惰性
fold/reduce           O(n)          O(1)          单次遍历
```

### 7.4 避免不必要的分配

```rust
// ❌ 慢：多次分配
let v = vec![1, 2, 3, 4, 5];
let doubled = v.iter().map(|x| x * 2).collect::<Vec<_>>();
let filtered = doubled.iter().filter(|&&x| x > 5).collect::<Vec<_>>();

// ✓ 快：单次分配
let result: Vec<_> = v.iter()
    .map(|x| x * 2)
    .filter(|&x| x > 5)
    .collect();
```

---

## 8. 实战模式

### 8.1 管道模式

```rust
// 数据处理管道
let result: Vec<_> = data
    .iter()
    .filter(|x| x.is_valid())      // 过滤
    .map(|x| x.transform())         // 转换
    .take(10)                       // 限制数量
    .collect();                     // 收集
```

### 8.2 分组统计

```rust
use std::collections::HashMap;

let words = vec!["hello", "world", "hello", "rust"];
let counts: HashMap<&str, usize> = words.iter()
    .fold(HashMap::new(), |mut acc, &word| {
        *acc.entry(word).or_insert(0) += 1;
        acc
    });
```

### 8.3 窗口处理

```rust
let data = vec![1, 2, 3, 4, 5];

// 滑动窗口（size = 2）
let windows: Vec<_> = data.windows(2).collect();
// [[1, 2], [2, 3], [3, 4], [4, 5]]

// 块（size = 2）
let chunks: Vec<_> = data.chunks(2).collect();
// [[1, 2], [3, 4], [5]]
```

### 8.4 多源合并

```rust
let iter1 = vec![1, 3, 5].into_iter();
let iter2 = vec![2, 4, 6].into_iter();

// 交替合并
use itertools::Itertools;  // 需要 itertools crate
let merged: Vec<_> = iter1.interleave(iter2).collect();
// [1, 2, 3, 4, 5, 6]
```

### 8.5 错误处理

```rust
let results = vec![Ok(1), Err("error"), Ok(2), Ok(3)];

// 收集所有成功的结果
let successes: Vec<_> = results.iter()
    .filter_map(|r| r.as_ref().ok())
    .collect();

// 短路：遇到错误立即返回
let collected: Result<Vec<_>, _> = results.into_iter().collect();
```

### 8.6 延迟计算

```rust
// 创建无限序列（惰性）
let fibonacci = std::iter::successors(Some((0, 1)), |&(a, b)| {
    Some((b, a + b))
})
.map(|(a, _)| a);

// 只在需要时计算
let first_10: Vec<_> = fibonacci.take(10).collect();
```

---

## 9. 常见陷阱

### 9.1 多次消费

```rust
let v = vec![1, 2, 3];

// ❌ 错误：迭代器已被消费
let iter = v.into_iter();
let sum: i32 = iter.sum();  // iter 被消费
let count = iter.count();   // 错误！iter 已失效

// ✓ 正确：重新创建或克隆
let v = vec![1, 2, 3];
let sum: i32 = v.iter().sum();
let count = v.len();
```

### 9.2 忘记收集

```rust
// ❌ 错误：忘记 collect
let doubled = vec![1, 2, 3].iter().map(|x| x * 2);
// doubled 是迭代器，不是 Vec！

// ✓ 正确
let doubled: Vec<_> = vec![1, 2, 3].iter().map(|x| x * 2).collect();
```

### 9.3 不必要的 collect

```rust
// ❌ 慢：不必要的中间分配
let sum: i32 = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>()  // 不必要
    .iter()
    .sum();

// ✓ 快：直接链式
let sum: i32 = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .sum();
```

---

## 10. 速查表

### 创建迭代器
```rust
collection.iter()         // 不可变引用
collection.iter_mut()     // 可变引用
collection.into_iter()    // 获取所有权
(0..10)                   // 范围
std::iter::repeat(x)      // 重复
std::iter::once(x)        // 单次
```

### 转换（适配器）
```rust
.map(f)                   // 转换
.filter(p)                // 过滤
.take(n)                  // 取前 n 个
.skip(n)                  // 跳过前 n 个
.chain(other)             // 连接
.zip(other)               // 配对
.enumerate()              // 添加索引
.flatten()                // 扁平化
.rev()                    // 反转
```

### 消费（终结）
```rust
.collect()                // 收集到集合
.sum()                    // 求和
.product()                // 求积
.count()                  // 计数
.find(p)                  // 查找
.any(p)                   // 任意
.all(p)                   // 全部
.fold(init, f)            // 折叠
.for_each(f)              // 遍历
```

---

**更多信息：**
- 运行示例：`cargo run --example iterator_demo`
- 查看测试：`cargo test --example iterator_demo`
- 官方文档：https://doc.rust-lang.org/std/iter/
