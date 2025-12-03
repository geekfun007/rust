# Rust 数据类型与方法深入详解

## 目录
- [1. 基础数据类型详解](#1-基础数据类型详解)
- [2. 智能指针](#2-智能指针)
- [3. 方法与关联函数](#3-方法与关联函数)
- [4. Trait 深入](#4-trait-深入)
- [5. 高级类型](#5-高级类型)
- [6. 类型转换](#6-类型转换)

---

## 1. 基础数据类型详解

### 1.1 数值类型

#### 整数类型

```rust
fn integer_types() {
    // 有符号整数
    let a: i8 = -128;              // -128 到 127
    let b: i16 = -32768;           // -32,768 到 32,767
    let c: i32 = -2147483648;      // -2^31 到 2^31-1（默认）
    let d: i64 = -9223372036854775808;
    let e: i128 = 0;               // -2^127 到 2^127-1
    
    // 无符号整数
    let f: u8 = 255;               // 0 到 255
    let g: u16 = 65535;            // 0 到 65,535
    let h: u32 = 4294967295;       // 0 到 2^32-1
    let i: u64 = 18446744073709551615;
    let j: u128 = 0;               // 0 到 2^128-1
    
    // 平台相关
    let k: isize = -1;             // 32位系统：i32，64位系统：i64
    let l: usize = 1;              // 32位系统：u32，64位系统：u64
    
    // 数字字面量
    let decimal = 98_222;          // 十进制
    let hex = 0xff;                // 十六进制：255
    let octal = 0o77;              // 八进制：63
    let binary = 0b1111_0000;      // 二进制：240
    let byte = b'A';               // 字节（u8）：65
}
```

#### 整数方法

```rust
fn integer_methods() {
    let num: i32 = 42;
    
    // 数学运算
    println!("绝对值: {}", num.abs());
    println!("幂运算: {}", num.pow(2));              // 42^2
    println!("平方根: {}", (num as f64).sqrt());
    
    // 检查方法
    println!("是否为正: {}", num.is_positive());
    println!("是否为负: {}", num.is_negative());
    
    // 溢出处理
    let result = num.checked_add(100);              // 返回 Option<i32>
    let wrapped = num.wrapping_add(100);            // 环绕溢出
    let saturated = num.saturating_add(100);        // 饱和到最大值
    let (value, overflow) = num.overflowing_add(100); // 返回值和溢出标志
    
    // 位操作
    println!("前导零: {}", num.leading_zeros());
    println!("尾随零: {}", num.trailing_zeros());
    println!("位数: {}", num.count_ones());
    println!("字节交换: {}", num.swap_bytes());
    println!("位反转: {}", num.reverse_bits());
    
    // 转换
    println!("转字节数组: {:?}", num.to_be_bytes());  // 大端序
    println!("转字节数组: {:?}", num.to_le_bytes());  // 小端序
    
    // 范围
    println!("最小值: {}", i32::MIN);
    println!("最大值: {}", i32::MAX);
}
```

#### 浮点类型

```rust
fn float_types() {
    let a: f32 = 3.14;             // 单精度：32位
    let b: f64 = 2.718281828;      // 双精度：64位（默认）
    
    // 特殊值
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    // 浮点方法
    let x = 3.5_f64;
    
    println!("向下取整: {}", x.floor());              // 3.0
    println!("向上取整: {}", x.ceil());               // 4.0
    println!("四舍五入: {}", x.round());              // 4.0
    println!("取整数部分: {}", x.trunc());            // 3.0
    println!("小数部分: {}", x.fract());              // 0.5
    
    println!("绝对值: {}", x.abs());
    println!("符号: {}", x.signum());                 // 1.0
    println!("平方根: {}", x.sqrt());
    println!("立方根: {}", x.cbrt());
    println!("指数: {}", x.exp());
    println!("对数: {}", x.ln());
    println!("幂运算: {}", x.powf(2.0));
    
    // 三角函数
    let angle = std::f64::consts::PI / 4.0;
    println!("sin: {}", angle.sin());
    println!("cos: {}", angle.cos());
    println!("tan: {}", angle.tan());
    
    // 检查
    println!("是否为 NaN: {}", x.is_nan());
    println!("是否有限: {}", x.is_finite());
    println!("是否无限: {}", x.is_infinite());
    println!("是否正常: {}", x.is_normal());
}
```

### 1.2 字符串类型深入

#### String 方法详解

```rust
fn string_methods() {
    let mut s = String::from("Hello");
    
    // 创建方法
    let s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = "Hello".to_string();
    let s4 = String::with_capacity(10);  // 预分配容量
    
    // 追加方法
    s.push_str(", World");               // 追加字符串切片
    s.push('!');                         // 追加单个字符
    
    // 插入方法
    s.insert(5, ',');                    // 在索引 5 插入字符
    s.insert_str(5, " there");           // 在索引 5 插入字符串
    
    // 删除方法
    s.pop();                             // 删除并返回最后一个字符
    s.remove(5);                         // 删除指定索引的字符
    s.truncate(5);                       // 截断到指定长度
    s.clear();                           // 清空字符串
    
    s = String::from("Hello, World!");
    
    // 替换方法
    let s5 = s.replace("World", "Rust");              // 替换所有匹配
    let s6 = s.replacen("l", "L", 2);                 // 替换前 n 个
    
    // 查询方法
    println!("长度: {}", s.len());
    println!("容量: {}", s.capacity());
    println!("是否为空: {}", s.is_empty());
    println!("包含: {}", s.contains("World"));
    println!("开头: {}", s.starts_with("Hello"));
    println!("结尾: {}", s.ends_with("!"));
    
    // 查找方法
    if let Some(index) = s.find("World") {
        println!("找到位置: {}", index);
    }
    
    // 分割方法
    for word in s.split(',') {
        println!("单词: {}", word.trim());
    }
    
    let parts: Vec<&str> = s.split_whitespace().collect();
    let lines: Vec<&str> = "line1\nline2\nline3".lines().collect();
    
    // 大小写转换
    println!("大写: {}", s.to_uppercase());
    println!("小写: {}", s.to_lowercase());
    
    // 修剪
    let s7 = "  Hello  ".trim();                      // 两端空白
    let s8 = "  Hello  ".trim_start();                // 开头空白
    let s9 = "  Hello  ".trim_end();                  // 结尾空白
    let s10 = "***Hello***".trim_matches('*');        // 指定字符
    
    // 解析
    let number: Result<i32, _> = "42".parse();
    
    // 重复
    let repeated = "ab".repeat(3);                    // "ababab"
    
    // 字节操作
    let bytes = s.as_bytes();
    let s_from_bytes = String::from_utf8(bytes.to_vec()).unwrap();
    
    // 容量管理
    s.reserve(100);                                   // 预留额外容量
    s.shrink_to_fit();                                // 收缩到实际大小
}
```

#### &str 方法

```rust
fn str_methods() {
    let s: &str = "Hello, World!";
    
    // 切片操作
    let hello = &s[0..5];
    let world = &s[7..12];
    
    // 字符迭代
    for c in s.chars() {
        println!("字符: {}", c);
    }
    
    // 字节迭代
    for b in s.bytes() {
        println!("字节: {}", b);
    }
    
    // 字符索引迭代
    for (i, c) in s.char_indices() {
        println!("位置 {}: {}", i, c);
    }
    
    // 分割
    let words: Vec<&str> = s.split_whitespace().collect();
    let parts: Vec<&str> = s.split(',').collect();
    let lines: Vec<&str> = s.lines().collect();
    
    // 模式匹配
    let matches: Vec<&str> = s.matches("o").collect();
    let rmatches: Vec<&str> = s.rmatches("o").collect();  // 反向
    
    // 检查
    println!("是否为空: {}", s.is_empty());
    println!("是否为 ASCII: {}", s.is_ascii());
    println!("包含: {}", s.contains("World"));
    
    // 转换
    let owned: String = s.to_owned();
    let upper = s.to_uppercase();
    let lower = s.to_lowercase();
}
```

### 1.3 集合类型深入

#### Vec<T> 详解

```rust
fn vec_methods() {
    // 创建方法
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    let v3 = Vec::with_capacity(10);
    let v4 = vec![0; 5];                              // [0, 0, 0, 0, 0]
    
    // 添加元素
    v1.push(1);
    v1.push(2);
    v1.extend([3, 4, 5]);
    v1.extend_from_slice(&[6, 7, 8]);
    v1.append(&mut v2);                               // 移动 v2 的元素
    
    // 插入元素
    v1.insert(0, 100);                                // 在索引 0 插入
    
    // 删除元素
    v1.pop();                                         // 删除并返回最后一个
    v1.remove(0);                                     // 删除指定索引
    v1.swap_remove(0);                                // 快速删除（不保持顺序）
    v1.truncate(5);                                   // 截断到指定长度
    v1.clear();                                       // 清空
    
    v1 = vec![1, 2, 3, 4, 5];
    
    // 访问元素
    let first = &v1[0];                               // 可能 panic
    let second = v1.get(1);                           // 返回 Option<&T>
    let last = v1.last();                             // 最后一个元素
    let first_mut = v1.first_mut();                   // 可变引用
    
    // 切片
    let slice = &v1[1..4];                            // [2, 3, 4]
    
    // 迭代
    for item in &v1 {
        println!("{}", item);
    }
    
    for item in &mut v1 {
        *item *= 2;
    }
    
    // 容量管理
    println!("长度: {}", v1.len());
    println!("容量: {}", v1.capacity());
    println!("是否为空: {}", v1.is_empty());
    
    v1.reserve(100);                                  // 预留容量
    v1.shrink_to_fit();                               // 收缩
    
    // 排序
    v1.sort();                                        // 升序
    v1.sort_by(|a, b| b.cmp(a));                     // 降序
    v1.sort_unstable();                               // 不稳定排序（更快）
    
    // 去重
    v1.dedup();                                       // 删除连续重复元素
    
    // 反转
    v1.reverse();
    
    // 旋转
    v1.rotate_left(2);                                // 左旋 2 位
    v1.rotate_right(1);                               // 右旋 1 位
    
    // 分割
    let (left, right) = v1.split_at(3);
    let chunks: Vec<&[i32]> = v1.chunks(2).collect();
    let windows: Vec<&[i32]> = v1.windows(3).collect();
    
    // 查找
    println!("包含 3: {}", v1.contains(&3));
    if let Some(pos) = v1.iter().position(|&x| x == 3) {
        println!("找到位置: {}", pos);
    }
    
    // 保留
    v1.retain(|&x| x > 2);                            // 只保留 > 2 的元素
    
    // 转换
    let doubled: Vec<i32> = v1.iter().map(|x| x * 2).collect();
    let filtered: Vec<i32> = v1.iter().filter(|&&x| x > 2).copied().collect();
}
```

#### HashMap<K, V> 详解

```rust
use std::collections::HashMap;

fn hashmap_methods() {
    // 创建
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 100);
    map.insert("key2".to_string(), 200);
    
    // 从迭代器创建
    let tuples = vec![("a", 1), ("b", 2)];
    let map2: HashMap<_, _> = tuples.into_iter().collect();
    
    // 预分配容量
    let mut map3 = HashMap::with_capacity(10);
    
    // 插入和更新
    map.insert("key3".to_string(), 300);              // 插入或更新
    
    // entry API（推荐）
    map.entry("key1".to_string())
        .and_modify(|v| *v += 10)
        .or_insert(50);
    
    map.entry("key4".to_string())
        .or_insert(400);
    
    // 只在不存在时插入
    map.entry("key5".to_string())
        .or_insert_with(|| expensive_computation());
    
    // 获取值
    let value = map.get("key1");                      // Option<&V>
    let value_mut = map.get_mut("key1");              // Option<&mut V>
    let value_default = map.get("nonexistent")
        .unwrap_or(&0);
    
    // 删除
    map.remove("key1");                               // 返回 Option<V>
    let (k, v) = map.remove_entry("key2").unwrap();  // 返回键值对
    
    // 检查
    println!("包含键: {}", map.contains_key("key1"));
    println!("长度: {}", map.len());
    println!("是否为空: {}", map.is_empty());
    
    // 迭代
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    for key in map.keys() {
        println!("键: {}", key);
    }
    
    for value in map.values() {
        println!("值: {}", value);
    }
    
    for value in map.values_mut() {
        *value *= 2;
    }
    
    // 保留
    map.retain(|_k, &mut v| v > 100);
    
    // 清空
    map.clear();
}

fn expensive_computation() -> i32 {
    println!("执行复杂计算...");
    500
}
```

#### HashSet<T> 详解

```rust
use std::collections::HashSet;

fn hashset_methods() {
    let mut set1: HashSet<i32> = HashSet::new();
    let mut set2: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    
    // 插入
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    // 删除
    set1.remove(&2);
    
    // 检查
    println!("包含 1: {}", set1.contains(&1));
    println!("长度: {}", set1.len());
    println!("是否为空: {}", set1.is_empty());
    
    // 集合操作
    let set3: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();
    
    // 并集
    let union: HashSet<_> = set1.union(&set3).cloned().collect();
    
    // 交集
    let intersection: HashSet<_> = set1.intersection(&set3).cloned().collect();
    
    // 差集
    let difference: HashSet<_> = set1.difference(&set3).cloned().collect();
    
    // 对称差集
    let symmetric_diff: HashSet<_> = set1.symmetric_difference(&set3)
        .cloned()
        .collect();
    
    // 子集和超集
    println!("是子集: {}", set1.is_subset(&set2));
    println!("是超集: {}", set1.is_superset(&set2));
    println!("不相交: {}", set1.is_disjoint(&set3));
    
    // 迭代
    for item in &set1 {
        println!("{}", item);
    }
    
    // 保留
    set1.retain(|&x| x > 1);
    
    // 清空
    set1.clear();
}
```

---

## 2. 智能指针

### 2.1 Box<T> - 堆分配

```rust
fn box_examples() {
    // 基本用法
    let b = Box::new(5);
    println!("b = {}", b);
    
    // 递归类型
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // 大对象
    struct LargeData {
        data: [u8; 1000000],
    }
    
    let large = Box::new(LargeData {
        data: [0; 1000000],
    });
    
    // Box 方法
    let boxed = Box::new(vec![1, 2, 3]);
    let leaked: &'static mut Vec<i32> = Box::leak(boxed);  // 泄漏内存获得 'static
    
    // 从原始指针创建
    let x = Box::new(5);
    let ptr = Box::into_raw(x);
    let x = unsafe { Box::from_raw(ptr) };
}
```

### 2.2 Rc<T> - 引用计数

```rust
use std::rc::Rc;

fn rc_examples() {
    // 创建
    let a = Rc::new(5);
    
    // 克隆（增加引用计数）
    let b = Rc::clone(&a);
    let c = a.clone();
    
    // 引用计数
    println!("引用计数: {}", Rc::strong_count(&a));  // 3
    println!("弱引用计数: {}", Rc::weak_count(&a));  // 0
    
    // 获取内部值
    println!("值: {}", *a);
    
    // 尝试获取可变引用
    if let Some(value) = Rc::get_mut(&mut a) {
        *value = 10;  // 只有当引用计数为 1 时才成功
    }
    
    // 图结构示例
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }
    
    let leaf = Rc::new(Node {
        value: 3,
        children: vec![],
    });
    
    let branch = Rc::new(Node {
        value: 5,
        children: vec![Rc::clone(&leaf)],
    });
}
```

### 2.3 Arc<T> - 原子引用计数

```rust
use std::sync::Arc;
use std::thread;

fn arc_examples() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {} 看到: {:?}", i, data);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 引用计数
    println!("强引用: {}", Arc::strong_count(&data));
    println!("弱引用: {}", Arc::weak_count(&data));
}
```

### 2.4 RefCell<T> - 内部可变性

```rust
use std::cell::RefCell;

fn refcell_examples() {
    let data = RefCell::new(5);
    
    // 借用
    {
        let borrowed = data.borrow();  // 不可变借用
        println!("值: {}", *borrowed);
    }
    
    {
        let mut borrowed_mut = data.borrow_mut();  // 可变借用
        *borrowed_mut += 1;
    }
    
    // 尝试借用
    if let Ok(borrowed) = data.try_borrow() {
        println!("成功借用: {}", *borrowed);
    }
    
    // 替换值
    let old_value = data.replace(10);
    println!("旧值: {}", old_value);
    
    // 交换值
    let other = RefCell::new(20);
    data.swap(&other);
    
    // 获取原始指针
    let ptr = data.as_ptr();
    
    // Mock 对象示例
    trait Messenger {
        fn send(&self, msg: &str);
    }
    
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
}
```

### 2.5 Cell<T> - 简单内部可变性

```rust
use std::cell::Cell;

fn cell_examples() {
    let data = Cell::new(5);
    
    // 获取值（Copy 类型）
    let value = data.get();
    println!("值: {}", value);
    
    // 设置值
    data.set(10);
    
    // 替换值
    let old = data.replace(20);
    println!("旧值: {}", old);
    
    // 交换值
    let other = Cell::new(30);
    data.swap(&other);
    
    // 更新值
    data.update(|x| x + 10);
    
    // 示例：计数器
    struct Counter {
        count: Cell<u32>,
    }
    
    impl Counter {
        fn new() -> Counter {
            Counter {
                count: Cell::new(0),
            }
        }
        
        fn increment(&self) {
            let count = self.count.get();
            self.count.set(count + 1);
        }
        
        fn get(&self) -> u32 {
            self.count.get()
        }
    }
}
```

---

## 3. 方法与关联函数

### 3.1 方法定义

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数（构造器）
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
    // 不可变方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 可变方法
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    // 消费 self 的方法
    fn into_square(self) -> Rectangle {
        let size = self.width.max(self.height);
        Rectangle::square(size)
    }
}

fn method_examples() {
    // 使用关联函数
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(25);
    
    // 调用方法
    println!("面积: {}", rect1.area());
    println!("周长: {}", rect1.perimeter());
    println!("能容纳: {}", rect1.can_hold(&rect2));
    
    // 可变方法
    let mut rect3 = Rectangle::new(10, 20);
    rect3.scale(2);
    rect3.set_width(30);
    
    // 消费方法
    let square = rect3.into_square();
}
```

### 3.2 多个 impl 块

```rust
struct Point {
    x: f64,
    y: f64,
}

// 基本方法
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

// 计算方法
impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

// 修改方法
impl Point {
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
    
    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}
```

### 3.3 泛型方法

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
    
    fn set(&mut self, value: T) {
        self.value = value;
    }
    
    fn into_inner(self) -> T {
        self.value
    }
}

// 为特定类型实现方法
impl Container<String> {
    fn len(&self) -> usize {
        self.value.len()
    }
    
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

// 泛型约束
impl<T: std::fmt::Display> Container<T> {
    fn display(&self) {
        println!("容器包含: {}", self.value);
    }
}

impl<T: Clone> Container<T> {
    fn duplicate(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
```

---

## 4. Trait 深入

### 4.1 常用标准 Trait

#### Debug 和 Display

```rust
use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 自定义 Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn display_examples() {
    let p = Point { x: 10, y: 20 };
    
    println!("Debug: {:?}", p);        // 使用 Debug
    println!("Pretty: {:#?}", p);      // 美化输出
    println!("Display: {}", p);        // 使用 Display
}
```

#### Clone 和 Copy

```rust
// Copy（自动实现 Clone）
#[derive(Copy, Clone, Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

// Clone（包含非 Copy 类型）
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn clone_examples() {
    // Copy 类型
    let p1 = Point2D { x: 1, y: 2 };
    let p2 = p1;  // 自动复制
    println!("{:?} {:?}", p1, p2);  // p1 仍然有效
    
    // Clone 类型
    let person1 = Person::new("Alice", 30);
    let person2 = person1.clone();  // 显式克隆
    // person1 仍然有效
}
```

#### PartialEq 和 Eq

```rust
#[derive(Debug, PartialEq)]
struct Book {
    title: String,
    author: String,
    isbn: String,
}

impl Book {
    fn new(title: &str, author: &str, isbn: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
        }
    }
}

// 自定义相等比较（只比较 ISBN）
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

impl Eq for Book {}  // Eq 是 PartialEq 的超集
```

#### PartialOrd 和 Ord

```rust
#[derive(Debug, PartialEq, Eq)]
struct Student {
    name: String,
    grade: u32,
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 先按成绩，再按名字排序
        self.grade.cmp(&other.grade)
            .then_with(|| self.name.cmp(&other.name))
    }
}

fn ord_examples() {
    let mut students = vec![
        Student { name: "Alice".to_string(), grade: 85 },
        Student { name: "Bob".to_string(), grade: 92 },
        Student { name: "Charlie".to_string(), grade: 85 },
    ];
    
    students.sort();  // 可以直接排序
    
    for student in &students {
        println!("{}: {}", student.name, student.grade);
    }
}
```

#### Default

```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            timeout: 30,
        }
    }
}

fn default_examples() {
    let config1 = Config::default();
    
    // 使用默认值 + 结构体更新语法
    let config2 = Config {
        port: 3000,
        ..Default::default()
    };
}
```

### 4.2 From 和 Into

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// 实现 From
impl From<(&str, u32)> for Person {
    fn from((name, age): (&str, u32)) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

// From 自动提供 Into
fn conversion_examples() {
    // 使用 From
    let p1 = Person::from(("Alice", 30));
    
    // 使用 Into（需要类型注解）
    let p2: Person = ("Bob", 25).into();
    
    // 字符串转换
    let s1: String = "hello".into();
    let s2 = String::from("world");
}
```

### 4.3 TryFrom 和 TryInto

```rust
use std::convert::TryFrom;

#[derive(Debug)]
struct Age(u8);

#[derive(Debug)]
struct AgeError;

impl TryFrom<i32> for Age {
    type Error = AgeError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value <= 120 {
            Ok(Age(value as u8))
        } else {
            Err(AgeError)
        }
    }
}

fn try_conversion_examples() {
    // 使用 TryFrom
    match Age::try_from(25) {
        Ok(age) => println!("有效年龄: {:?}", age),
        Err(_) => println!("无效年龄"),
    }
    
    // 使用 TryInto
    let result: Result<Age, _> = 150.try_into();
    println!("{:?}", result);  // Err
}
```

### 4.4 AsRef 和 AsMut

```rust
fn process_string<S: AsRef<str>>(s: S) {
    let s = s.as_ref();
    println!("处理: {}", s);
}

fn asref_examples() {
    // 可以接受多种类型
    process_string("字符串字面量");
    process_string(String::from("String"));
    process_string(&String::from("&String"));
}
```

---

## 5. 高级类型

### 5.1 类型别名

```rust
type Kilometers = i32;
type Result<T> = std::result::Result<T, std::io::Error>;

fn type_alias_examples() {
    let distance: Kilometers = 100;
    
    fn read_file() -> Result<String> {
        std::fs::read_to_string("file.txt")
    }
}
```

### 5.2 Never 类型

```rust
fn never_type_examples() {
    // never 类型 !
    fn diverges() -> ! {
        panic!("这个函数永不返回");
    }
    
    // continue 的类型是 !
    let x: i32 = loop {
        break 5;
    };
}
```

### 5.3 动态大小类型（DST）

```rust
fn dst_examples() {
    // str 是 DST
    let s1: &str = "hello";
    
    // [T] 是 DST
    let arr: &[i32] = &[1, 2, 3];
    
    // trait 对象是 DST
    trait Animal {
        fn sound(&self) -> &str;
    }
    
    let animal: &dyn Animal;
}
```

---

## 6. 类型转换

### 6.1 as 转换

```rust
fn as_conversion() {
    // 数值转换
    let i = 42i32;
    let f = i as f64;
    let u = i as u32;
    
    // 指针转换
    let ptr = &i as *const i32;
    let addr = ptr as usize;
    
    // 截断转换
    let x: i64 = 1000;
    let y: i32 = x as i32;  // 可能溢出
}
```

### 6.2 安全转换

```rust
fn safe_conversion() {
    let x: i32 = 42;
    
    // 使用 TryFrom
    use std::convert::TryFrom;
    if let Ok(y) = i16::try_from(x) {
        println!("转换成功: {}", y);
    }
    
    // 使用 From
    let y: i64 = i64::from(x);
}
```

---

继续下一部分：HTTP + ORM + DAL 服务端开发详解...
