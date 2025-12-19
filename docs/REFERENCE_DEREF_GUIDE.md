# Rust 引用与解引用详解

## 目录
1. [基础概念](#1-基础概念)
2. [&v - 创建引用](#2-v---创建引用)
3. [*v - 解引用](#3-v---解引用)
4. [&&v - 双重引用](#4-v---双重引用)
5. [**v - 双重解引用](#5-v---双重解引用)
6. [模式匹配中的引用](#6-模式匹配中的引用)
7. [闭包中的引用](#7-闭包中的引用)
8. [实战应用](#8-实战应用)
9. [常见陷阱](#9-常见陷阱)

---

## 1. 基础概念

### 1.1 值、引用、解引用

```
类型层级：

T           原始值（所有权）
&T          不可变引用
&mut T      可变引用
&&T         引用的引用
&&&T        三重引用
...

操作符：
&           取引用（借用）
*           解引用（访问引用指向的值）
```

### 1.2 内存布局

```
栈内存示例：

let x = 42;              栈
                        ┌────┐
                     x  │ 42 │
                        └────┘

let r = &x;             栈
                        ┌────┐
                     x  │ 42 │
                        └────┘
                         ↑
                        ┌┴───┐
                     r  │ptr │
                        └────┘

let rr = &r;            栈
                        ┌────┐
                     x  │ 42 │
                        └────┘
                         ↑
                        ┌┴───┐
                     r  │ptr │ ←─┐
                        └────┘   │
                                ┌┴───┐
                    rr          │ptr │
                                └────┘
```

---

## 2. &v - 创建引用

### 2.1 基本用法

```rust
let x = 5;
let r = &x;      // 创建不可变引用

let mut y = 10;
let mr = &mut y; // 创建可变引用
```

### 2.2 内存视图

```
&x 的含义：
1. 不移动 x 的所有权
2. 创建指向 x 的指针
3. 返回类型 &i32

栈布局：
x   ┌────┐
    │ 5  │
    └────┘
     ↑
r   ┌┴───┐
    │ptr │  ← 类型 &i32，大小 8 字节
    └────┘
```

### 2.3 引用规则

```rust
let mut x = 5;

// 规则 1：多个不可变引用可以共存
let r1 = &x;
let r2 = &x;
let r3 = &x;
println!("{}, {}, {}", r1, r2, r3);

// 规则 2：可变引用是独占的
let mr = &mut x;
// let r4 = &x;     // ❌ 错误：不能同时存在
// let mr2 = &mut x; // ❌ 错误：不能有两个可变引用
*mr += 1;
println!("{}", mr);
```

### 2.4 自动解引用

```rust
let x = 5;
let r = &x;

// 这些都是等价的
println!("{}", *r);   // 显式解引用
println!("{}", r);    // 自动解引用（Display trait）

// 方法调用也会自动解引用
let s = String::from("hello");
let sr = &s;
println!("{}", sr.len());  // 自动解引用调用方法
```

---

## 3. *v - 解引用

### 3.1 基本用法

```rust
let x = 5;
let r = &x;

// *r 解引用，获取 r 指向的值
let y = *r;  // y = 5
println!("x = {}, *r = {}, y = {}", x, *r, y);

// 可变引用的解引用
let mut z = 10;
let mr = &mut z;
*mr += 5;    // 通过解引用修改值
println!("z = {}", z);  // 15
```

### 3.2 解引用操作

```
解引用的作用：

&i32  ──*──>  i32
&&i32 ──*──>  &i32
&&&i32──*──>  &&i32

每个 * 去掉一层引用
```

### 3.3 解引用 vs 自动解引用

```rust
let x = 5;
let r = &x;

// 场景 1：赋值需要显式解引用
let y = *r;      // 必须显式

// 场景 2：比较可以自动解引用
assert_eq!(r, &5);   // 自动

// 场景 3：方法调用自动解引用
let s = String::from("hello");
let sr = &s;
sr.len();            // 自动解引用

// 场景 4：算术运算需要显式解引用
let a = &10;
let b = &20;
let sum = *a + *b;   // 必须显式
```

### 3.4 Deref Trait

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}

let x = MyBox(5);
let y = *x;  // 调用 deref，等价于 *(x.deref())
```

---

## 4. &&v - 双重引用

### 4.1 创建双重引用

```rust
let x = 5;
let r = &x;     // &i32
let rr = &r;    // &&i32

println!("x = {}", x);     // 5
println!("*r = {}", *r);   // 5
println!("**rr = {}", **rr); // 5
```

### 4.2 内存布局

```
三层数据：

栈内存
┌────────┐
│ x: 5   │ ← 实际值
└────────┘
    ↑
┌───┴───┐
│ r:ptr │ ← 第一层引用
└───────┘
    ↑
┌───┴───┐
│rr:ptr │ ← 第二层引用
└───────┘

类型演变：
x   : i32
r   : &i32
rr  : &&i32
```

### 4.3 常见场景

#### 场景 1：迭代器与引用

```rust
let v = vec![1, 2, 3];

// iter() 返回 &i32
for item in v.iter() {
    // item 的类型是 &i32
    println!("{}", item);
}

// 如果再取引用
for item in v.iter() {
    let r = &item;  // r 的类型是 &&i32
    println!("{}", **r);
}
```

#### 场景 2：引用作为参数

```rust
fn print_ref(r: &i32) {
    println!("{}", r);
}

let x = 5;
let r = &x;
print_ref(&r);  // 传递 &&i32，函数接收 &i32
                // 自动解引用强制转换
```

#### 场景 3：闭包捕获

```rust
let v = vec![1, 2, 3];
let item = &v[0];  // &i32

// 闭包通过引用捕获 item
let closure = || {
    // 这里 item 被捕获为 &&i32
    println!("{}", item);
};
```

### 4.4 解引用操作

```rust
let x = 5;
let r = &x;     // &i32
let rr = &r;    // &&i32

// 逐层解引用
println!("{}", rr);    // 自动解引用: 5
println!("{}", *rr);   // 解引用一次: &5
println!("{}", **rr);  // 解引用两次: 5

// 类型检查
let _: &&i32 = rr;     // ✓
let _: &i32 = *rr;     // ✓
let _: i32 = **rr;     // ✓
```

---

## 5. **v - 双重解引用

### 5.1 基本用法

```rust
let x = 5;
let r = &x;      // &i32
let rr = &r;     // &&i32

// 双重解引用
let y = **rr;    // i32
println!("y = {}", y);  // 5

// 等价于
let temp = *rr;  // temp: &i32
let y = *temp;   // y: i32
```

### 5.2 复杂嵌套

```rust
let x = 5;
let r1 = &x;      // &i32
let r2 = &r1;     // &&i32
let r3 = &r2;     // &&&i32

// 逐层解引用
println!("{}", ***r3);  // 5

// 每次解引用去掉一层
let a = *r3;     // a: &&i32
let b = **r3;    // b: &i32
let c = ***r3;   // c: i32
```

### 5.3 可变引用的双重解引用

```rust
let mut x = 5;
let mr = &mut x;     // &mut i32
let mmr = &mut mr;   // &mut &mut i32

// 修改值需要双重解引用
**mmr += 10;
println!("x = {}", x);  // 15

// 内存视图
// x 的值: 5 -> 15
// mr 指向 x
// mmr 指向 mr
```

---

## 6. 模式匹配中的引用

### 6.1 match 中的引用

```rust
let x = 5;
let r = &x;

// 模式 1：匹配引用
match r {
    &val => println!("值: {}", val),  // 解构引用
}

// 模式 2：使用 ref
match x {
    ref val => println!("引用: {}", val),  // 创建引用
}

// 模式 3：直接匹配
match r {
    _ => println!("引用: {}", r),
}
```

### 6.2 if let 中的引用

```rust
let v = vec![1, 2, 3];

// 迭代器返回 &i32
for item in v.iter() {
    // 模式 1：不解引用
    if let _ = item {
        println!("item 类型: &i32");
    }
    
    // 模式 2：解引用
    if let &val = item {
        println!("val 类型: i32, 值: {}", val);
    }
}
```

### 6.3 复杂模式

```rust
let data = vec![1, 2, 3, 4, 5];

// filter 的闭包参数是 &&i32
let even: Vec<_> = data.iter()
    .filter(|&&x| x % 2 == 0)  // 双重解引用
    .collect();

// 等价写法
let even: Vec<_> = data.iter()
    .filter(|x| **x % 2 == 0)  // 显式双重解引用
    .collect();

// 不解引用版本
let even: Vec<_> = data.iter()
    .filter(|x| *x % 2 == 0)   // ❌ 错误：类型不匹配
    .collect();
```

---

## 7. 闭包中的引用

### 7.1 闭包捕获

```rust
let x = 5;

// 捕获 x 的引用
let closure1 = || {
    println!("{}", x);  // 捕获 &x
};

// 捕获 x 的可变引用
let mut y = 10;
let mut closure2 = || {
    y += 1;  // 捕获 &mut y
};

// move 捕获所有权
let closure3 = move || {
    println!("{}", x);  // 捕获 x 的所有权
};
```

### 7.2 迭代器闭包

```rust
let v = vec![1, 2, 3, 4, 5];

// map 闭包参数是 &i32
v.iter().map(|x| {
    // x: &i32
    x * 2  // 自动解引用
});

// filter 闭包参数是 &&i32
v.iter().filter(|&&x| {
    // 双重解引用模式
    x > 2
});

// 显式类型标注
v.iter().filter(|x: &&i32| {
    **x > 2  // 显式双重解引用
});
```

### 7.3 引用捕获的陷阱

```rust
let v = vec![1, 2, 3];

// ❌ 错误：闭包捕获引用，但引用的生命周期不够长
// let closure = {
//     let temp = &v[0];
//     || println!("{}", temp)  // temp 的生命周期太短
// };

// ✓ 正确：使用 move 捕获所有权
let closure = {
    let temp = v[0];
    move || println!("{}", temp)
};
```

---

## 8. 实战应用

### 8.1 Vec 迭代模式

```rust
let v = vec![1, 2, 3, 4, 5];

// 模式 1：iter() - 不可变引用
for item in v.iter() {
    // item: &i32
    println!("{}", item);
}

// 模式 2：iter_mut() - 可变引用
let mut v = vec![1, 2, 3];
for item in v.iter_mut() {
    // item: &mut i32
    *item *= 2;  // 解引用修改
}

// 模式 3：into_iter() - 获取所有权
for item in v.into_iter() {
    // item: i32
    println!("{}", item);
}
```

### 8.2 Option<&T> vs Option<T>

```rust
let x = 5;
let r = &x;

// Option<&i32>
let opt1: Option<&i32> = Some(r);
match opt1 {
    Some(&val) => println!("值: {}", val),  // 解引用
    None => {}
}

// Option<i32>
let opt2: Option<i32> = Some(x);
match opt2 {
    Some(val) => println!("值: {}", val),
    None => {}
}

// 转换
let opt3 = opt1.map(|&x| x);  // Option<&i32> -> Option<i32>
let opt4 = opt2.as_ref();     // Option<i32> -> Option<&i32>
```

### 8.3 HashMap 查找

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", 42);

// get 返回 Option<&V>
if let Some(&value) = map.get("key") {
    // value: i32（解引用）
    println!("{}", value);
}

// 不解引用
if let Some(value) = map.get("key") {
    // value: &i32
    println!("{}", value);
}
```

### 8.4 自定义比较

```rust
let v = vec![3, 1, 4, 1, 5];

// sort_by 的闭包参数是 &&i32
let mut v_sorted = v.clone();
v_sorted.sort_by(|a, b| {
    // a: &&i32, b: &&i32
    (**a).cmp(&**b)  // 双重解引用
});

// 或使用模式匹配
v_sorted.sort_by(|&&a, &&b| {
    a.cmp(&b)
});
```

### 8.5 链式方法调用

```rust
let data = vec![1, 2, 3, 4, 5];

// 复杂的引用链
let result: Vec<_> = data
    .iter()                    // Iterator<Item = &i32>
    .filter(|&&x| x > 2)       // 双重解引用
    .map(|&x| x * 2)           // 单重解引用
    .collect();

println!("{:?}", result);  // [6, 8, 10]
```

### 8.6 引用传递优化

```rust
// ❌ 不好：不必要的克隆
fn process_bad(v: Vec<i32>) {
    for item in v {
        println!("{}", item);
    }
}

// ✓ 好：使用引用
fn process_good(v: &Vec<i32>) {
    for item in v {
        println!("{}", item);
    }
}

// ✓ 更好：使用切片
fn process_best(v: &[i32]) {
    for item in v {
        println!("{}", item);
    }
}
```

---

## 9. 常见陷阱

### 9.1 双重引用混淆

```rust
let v = vec![1, 2, 3];

// ❌ 常见错误
// v.iter().filter(|&x| x % 2 == 0)  // 类型错误
// x 是 &i32，不能取模

// ✓ 正确
v.iter().filter(|&&x| x % 2 == 0)    // 双重解引用
v.iter().filter(|x| **x % 2 == 0)    // 显式
```

### 9.2 自动解引用困惑

```rust
let x = 5;
let r = &x;

// 有时可以自动解引用
println!("{}", r);      // ✓ 自动

// 有时必须显式
let y = r + 1;          // ❌ 错误
let y = *r + 1;         // ✓ 正确
```

### 9.3 可变引用的多层嵌套

```rust
let mut x = 5;
let mr = &mut x;
let mmr = &mut mr;

// 需要双重解引用
**mmr += 10;

// 但方法调用可以自动
// (虽然这种情况很少见)
```

### 9.4 闭包参数类型

```rust
let v = vec![1, 2, 3];

// filter 的参数是 &&i32，不是 &i32
v.iter()
    .filter(|&&x| x > 1)        // ✓ 正确
    // .filter(|&x| x > 1)      // ❌ 类型错误
    // .filter(|x| x > 1)       // ❌ 类型错误
    .collect::<Vec<_>>();
```

---

## 10. 决策树

### 何时使用 &v？
```
需要借用而不是移动？
├─ 是 → 需要修改？
│  ├─ 是 → &mut v
│  └─ 否 → &v
└─ 否 → v
```

### 何时使用 *v？
```
v 是引用且需要值本身？
├─ 是 → 用于算术运算？
│  ├─ 是 → 必须用 *v
│  └─ 否 → 方法调用？
│     ├─ 是 → 自动解引用，可省略
│     └─ 否 → 打印等？
│        └─ 自动解引用，可省略
└─ 否 → 不需要
```

### 何时使用 &&v？
```
通常不需要主动创建 &&v
自然出现场景：
├─ 迭代器 + 引用捕获
├─ 嵌套数据结构
└─ 某些 API 设计
```

---

## 11. 性能考虑

```
操作         | 成本    | 说明
------------|---------|------------------
&v          | O(1)    | 仅复制指针
*v          | O(1)    | 仅解引用
&&v         | O(1)    | 仅复制指针
**v         | O(1)    | 两次解引用
自动解引用   | O(1)    | 编译时处理
```

---

## 12. 速查表

### 类型转换

```rust
T    →  &T      &value
&T   →  T       *reference
&T   →  &&T     &reference
&&T  →  &T      *reference
&&T  →  T       **reference
```

### 常见模式

```rust
// 引用
let r = &x;

// 解引用
let val = *r;

// 双重引用（通常自动）
// let rr = &r;  // 很少手动创建

// 双重解引用（迭代器闭包）
data.iter().filter(|&&x| x > 0)
```

---

**记住：**
- `&` 创建引用（借用）
- `*` 解引用（获取值）
- `&&v` 通常自动产生，很少手动创建
- `**v` 在迭代器闭包中常见
- 自动解引用适用于方法调用和某些操作符

**运行示例：** `cargo run --example reference_deref_demo`
