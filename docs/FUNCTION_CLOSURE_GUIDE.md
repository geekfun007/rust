# Rust 函数与闭包详解

## 目录
1. [基础概念](#1-基础概念)
2. [fn - 函数](#2-fn---函数)
3. [|| - 闭包](#3----闭包)
4. [核心差异](#4-核心差异)
5. [闭包捕获模式](#5-闭包捕获模式)
6. [闭包 Trait](#6-闭包-trait)
7. [函数指针](#7-函数指针)
8. [高阶函数](#8-高阶函数)
9. [实战应用](#9-实战应用)
10. [性能对比](#10-性能对比)
11. [最佳实践](#11-最佳实践)
12. [常见陷阱](#12-常见陷阱)

---

## 1. 基础概念

### 1.1 什么是函数 (fn)？

函数是**命名的、独立的代码块**，不捕获环境变量。

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### 1.2 什么是闭包 (||)？

闭包是**匿名的、可以捕获环境的函数**。

```rust
let add = |x, y| x + y;
```

### 1.3 核心区别

```
特性          | fn 函数        | || 闭包
-------------|---------------|------------------
命名          | 必须有名字     | 匿名（可绑定到变量）
类型推断      | 需要显式类型   | 自动类型推断
环境捕获      | 不能捕获      | 可以捕获
语法          | fn name(){}   | |args| expr
大小          | 零大小类型     | 取决于捕获的变量
实现 trait    | 都实现 Fn     | 实现 Fn/FnMut/FnOnce
```

---

## 2. fn - 函数

### 2.1 基本语法

```rust
// 基本函数
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // 函数体
}

// 无参数
fn say_hello() {
    println!("Hello!");
}

// 无返回值
fn print_number(n: i32) {
    println!("{}", n);
}

// 有返回值
fn square(x: i32) -> i32 {
    x * x  // 最后一个表达式是返回值
}

// 显式 return
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    x
}
```

### 2.2 函数特性

```rust
// 函数是零大小类型 (Zero-Sized Type)
fn example() {}
println!("fn size: {}", std::mem::size_of::<fn()>());  // 0

// 函数可以作为值传递
fn apply(f: fn(i32) -> i32, value: i32) -> i32 {
    f(value)
}

fn double(x: i32) -> i32 {
    x * 2
}

let result = apply(double, 5);  // 10
```

### 2.3 函数项类型 vs 函数指针

```rust
// 函数项类型 (Function Item Type)
fn foo() {}
let f1 = foo;  // 类型: fn foo() {foo}（唯一类型）

// 函数指针 (Function Pointer)
let f2: fn() = foo;  // 类型: fn()（通用类型）

// 每个函数有唯一的类型
fn bar() {}
// let x: fn foo() {foo} = bar;  // ❌ 类型不匹配
let x: fn() = bar;  // ✓ 可以，都是 fn()
```

### 2.4 泛型函数

```rust
// 泛型参数
fn print<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}

// 多个泛型参数
fn swap<T, U>(a: T, b: U) -> (U, T) {
    (b, a)
}

// trait 约束
fn process<T: Clone + std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}
```

---

## 3. || - 闭包

### 3.1 基本语法

```rust
// 最简单的闭包
let closure = || println!("Hello");

// 带参数
let add = |x, y| x + y;

// 显式类型标注
let multiply = |x: i32, y: i32| -> i32 { x * y };

// 多行闭包
let complex = |x: i32| {
    let temp = x * 2;
    temp + 1
};
```

### 3.2 类型推断

```rust
// 类型自动推断
let add = |x, y| x + y;
let result = add(1, 2);  // 推断为 i32

// 一旦推断，类型固定
// let result2 = add(1.0, 2.0);  // ❌ 错误：类型不匹配

// 显式类型标注
let add_i32 = |x: i32, y: i32| -> i32 { x + y };
let add_f64 = |x: f64, y: f64| -> f64 { x + y };
```

### 3.3 闭包大小

```rust
// 闭包大小取决于捕获的变量
let x = 10;
let y = 20;

let c1 = || println!("No capture");
let c2 = || println!("{}", x);      // 捕获一个 i32
let c3 = || println!("{} {}", x, y); // 捕获两个 i32

println!("c1 size: {}", std::mem::size_of_val(&c1));  // 0
println!("c2 size: {}", std::mem::size_of_val(&c2));  // 4
println!("c3 size: {}", std::mem::size_of_val(&c3));  // 8
```

---

## 4. 核心差异

### 4.1 命名 vs 匿名

```rust
// 函数：必须命名
fn named_function(x: i32) -> i32 {
    x * 2
}

// 闭包：匿名，可选绑定到变量
let anonymous_closure = |x: i32| x * 2;

// 内联使用
vec![1, 2, 3].iter().map(|x| x * 2);  // 闭包
```

### 4.2 环境捕获

```rust
// 函数：不能访问外部变量
fn cannot_capture() {
    let x = 10;
    // fn inner() -> i32 {
    //     x  // ❌ 错误：无法捕获
    // }
}

// 闭包：可以捕获外部变量
fn can_capture() {
    let x = 10;
    let closure = || x;  // ✓ 可以捕获
    println!("{}", closure());
}
```

### 4.3 类型系统

```rust
// 函数：每个函数有唯一的类型
fn func1(x: i32) -> i32 { x }
fn func2(x: i32) -> i32 { x }

let f: fn(i32) -> i32 = func1;  // 需要转换为函数指针

// 闭包：每个闭包有唯一的匿名类型
let c1 = |x: i32| x;
let c2 = |x: i32| x;

// let mut f = c1;
// f = c2;  // ❌ 错误：类型不同（即使签名相同）
```

### 4.4 内存布局

```
函数 (fn):
┌────────────┐
│  代码指针   │  ← 指向代码段
└────────────┘
大小: 0 字节（零大小类型）

闭包 (无捕获):
┌────────────┐
│  (空)      │
└────────────┘
大小: 0 字节

闭包 (捕获变量):
┌────────────┐
│  捕获变量1  │
├────────────┤
│  捕获变量2  │
├────────────┤
│  ...       │
└────────────┘
大小: 捕获变量的大小之和
```

---

## 5. 闭包捕获模式

### 5.1 三种捕获方式

```rust
let s = String::from("hello");

// 1. 不可变借用 (&T)
let borrow = || println!("{}", s);
borrow();
println!("{}", s);  // ✓ s 仍然可用

// 2. 可变借用 (&mut T)
let mut count = 0;
let mut increment = || count += 1;
increment();
// println!("{}", count);  // ❌ count 被可变借用

// 3. 获取所有权 (T)
let consume = move || {
    println!("{}", s);
    // s 被移动到闭包内
};
consume();
// println!("{}", s);  // ❌ s 已被移动
```

### 5.2 自动选择捕获方式

```rust
let x = 5;
let y = String::from("hello");

// 自动选择不可变借用
let closure1 = || {
    println!("{}", x);  // 只读，使用 &i32
};

// 自动选择可变借用
let mut z = 10;
let mut closure2 = || {
    z += 1;  // 修改，使用 &mut i32
};

// 自动选择所有权转移
let closure3 = || {
    drop(y);  // 需要所有权
};
```

### 5.3 move 关键字

```rust
let x = 5;
let s = String::from("hello");

// 不使用 move：借用
let closure1 = || println!("{} {}", x, s);
closure1();
println!("{}", s);  // ✓ s 仍然可用

// 使用 move：强制获取所有权
let closure2 = move || println!("{} {}", x, s);
closure2();
// println!("{}", s);  // ❌ s 已被移动

// move 对 Copy 类型的影响
let a = 10;  // i32 实现了 Copy
let closure3 = move || println!("{}", a);
closure3();
println!("{}", a);  // ✓ a 是 Copy，仍然可用
```

### 5.4 捕获的最小化

```rust
struct Data {
    field1: String,
    field2: i32,
}

let data = Data {
    field1: String::from("hello"),
    field2: 42,
};

// 只捕获使用的字段
let closure = || {
    println!("{}", data.field2);  // 只捕获 field2
};

// data.field1 仍然可以移动
let s = data.field1;  // ✓ 可以

// 但整个 data 不能移动
// let d = data;  // ❌ 错误
```

---

## 6. 闭包 Trait

### 6.1 三个闭包 Trait

```rust
// 1. FnOnce - 至少可以调用一次
pub trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

// 2. FnMut - 可以多次调用，可以修改捕获的变量
pub trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

// 3. Fn - 可以多次调用，不修改捕获的变量
pub trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}
```

### 6.2 Trait 层级

```
Fn ⊂ FnMut ⊂ FnOnce

Fn:      可以无限次调用，不修改环境
  ↓
FnMut:   可以多次调用，可以修改环境
  ↓
FnOnce:  至少可以调用一次，可能消耗环境
```

### 6.3 实例分析

```rust
// Fn: 不修改捕获的变量
let x = 5;
let fn_closure = || x + 1;
fn_closure();
fn_closure();  // ✓ 可以多次调用

// FnMut: 修改捕获的变量
let mut count = 0;
let mut fn_mut_closure = || {
    count += 1;
    count
};
fn_mut_closure();
fn_mut_closure();  // ✓ 可以多次调用

// FnOnce: 消耗捕获的变量
let s = String::from("hello");
let fn_once_closure = || {
    drop(s);  // 消耗 s
};
fn_once_closure();
// fn_once_closure();  // ❌ 只能调用一次
```

### 6.4 作为函数参数

```rust
// 接受 Fn
fn call_with_five<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(5)
}

// 接受 FnMut
fn call_twice<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}

// 接受 FnOnce
fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// 使用
let x = 10;
call_with_five(|n| n + x);  // Fn

let mut count = 0;
call_twice(|| count += 1);  // FnMut

let s = String::from("hello");
call_once(|| drop(s));  // FnOnce
```

---

## 7. 函数指针

### 7.1 fn 类型

```rust
// fn 是函数指针类型
let f: fn(i32) -> i32 = |x| x + 1;  // ❌ 闭包不能转换为 fn

fn add_one(x: i32) -> i32 {
    x + 1
}

let f: fn(i32) -> i32 = add_one;  // ✓ 函数可以
```

### 7.2 fn 实现了所有闭包 Trait

```rust
fn double(x: i32) -> i32 {
    x * 2
}

// fn 实现了 Fn
fn apply_fn<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

apply_fn(double, 5);  // ✓ fn 可以传递给需要 Fn 的地方
```

### 7.3 何时使用 fn

```rust
// 场景 1: 不需要捕获环境
fn process<F>(f: fn(i32) -> i32, value: i32) -> i32 {
    f(value)
}

// 场景 2: FFI (与 C 交互)
extern "C" fn callback(x: i32) -> i32 {
    x * 2
}

// 场景 3: 函数指针数组
let operations: [fn(i32) -> i32; 3] = [
    |x| x + 1,   // ❌ 错误：闭包不行
];

fn add_one(x: i32) -> i32 { x + 1 }
fn double(x: i32) -> i32 { x * 2 }
fn square(x: i32) -> i32 { x * x }

let operations: [fn(i32) -> i32; 3] = [add_one, double, square];  // ✓
```

---

## 8. 高阶函数

### 8.1 返回函数

```rust
// 返回函数指针
fn get_operation(op: char) -> fn(i32, i32) -> i32 {
    match op {
        '+' => add,
        '*' => mul,
        _ => add,
    }
}

fn add(a: i32, b: i32) -> i32 { a + b }
fn mul(a: i32, b: i32) -> i32 { a * b }
```

### 8.2 返回闭包

```rust
// 返回闭包需要 Box（大小未知）
fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

let add_5 = make_adder(5);
println!("{}", add_5(10));  // 15

// 使用 impl Trait（推荐）
fn make_multiplier(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

let mul_3 = make_multiplier(3);
println!("{}", mul_3(10));  // 30
```

### 8.3 接受函数/闭包

```rust
// 泛型版本（零成本抽象）
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// 使用
apply(|x| x * 2, 5);  // 闭包
apply(double, 5);      // 函数

fn double(x: i32) -> i32 { x * 2 }

// trait object 版本（动态分派）
fn apply_dynamic(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
```

---

## 9. 实战应用

### 9.1 迭代器操作

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 使用闭包
let doubled: Vec<_> = numbers.iter()
    .map(|&x| x * 2)
    .collect();

let evens: Vec<_> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .collect();

let sum: i32 = numbers.iter()
    .fold(0, |acc, &x| acc + x);

// 使用函数
fn is_even(x: &&i32) -> bool {
    **x % 2 == 0
}

let evens2: Vec<_> = numbers.iter()
    .filter(is_even)
    .collect();
```

### 9.2 回调函数

```rust
struct Button {
    label: String,
    on_click: Box<dyn Fn()>,
}

impl Button {
    fn new<F>(label: &str, on_click: F) -> Self
    where
        F: Fn() + 'static,
    {
        Button {
            label: label.to_string(),
            on_click: Box::new(on_click),
        }
    }
    
    fn click(&self) {
        (self.on_click)();
    }
}

// 使用
let counter = std::cell::RefCell::new(0);
let button = Button::new("Click me", || {
    *counter.borrow_mut() += 1;
    println!("Clicked {} times", counter.borrow());
});

button.click();
button.click();
```

### 9.3 惰性求值

```rust
struct LazyValue<F>
where
    F: Fn() -> i32,
{
    init: F,
    value: Option<i32>,
}

impl<F> LazyValue<F>
where
    F: Fn() -> i32,
{
    fn new(init: F) -> Self {
        LazyValue {
            init,
            value: None,
        }
    }
    
    fn get(&mut self) -> i32 {
        if self.value.is_none() {
            self.value = Some((self.init)());
        }
        self.value.unwrap()
    }
}

// 使用
let mut lazy = LazyValue::new(|| {
    println!("Computing...");
    42
});

println!("{}", lazy.get());  // 输出 "Computing..." 和 42
println!("{}", lazy.get());  // 只输出 42（不再计算）
```

### 9.4 策略模式

```rust
trait Strategy {
    fn execute(&self, a: i32, b: i32) -> i32;
}

struct Context<F>
where
    F: Fn(i32, i32) -> i32,
{
    strategy: F,
}

impl<F> Context<F>
where
    F: Fn(i32, i32) -> i32,
{
    fn new(strategy: F) -> Self {
        Context { strategy }
    }
    
    fn run(&self, a: i32, b: i32) -> i32 {
        (self.strategy)(a, b)
    }
}

// 使用
let add_context = Context::new(|a, b| a + b);
let mul_context = Context::new(|a, b| a * b);

println!("{}", add_context.run(3, 4));  // 7
println!("{}", mul_context.run(3, 4));  // 12
```

### 9.5 函数组合

```rust
fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

// 使用
let add_one = |x: i32| x + 1;
let double = |x: i32| x * 2;

let add_then_double = compose(add_one, double);
println!("{}", add_then_double(5));  // (5 + 1) * 2 = 12
```

---

## 10. 性能对比

### 10.1 零成本抽象

```rust
// 闭包（内联）
let nums = vec![1, 2, 3, 4, 5];
let sum1: i32 = nums.iter().map(|&x| x * 2).sum();

// 手写循环
let mut sum2 = 0;
for &x in &nums {
    sum2 += x * 2;
}

// 编译后性能相同！
```

### 10.2 静态分派 vs 动态分派

```rust
// 静态分派（泛型，零成本）
fn apply_static<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)  // 编译时确定，可内联
}

// 动态分派（trait object，运行时开销）
fn apply_dynamic(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)  // 虚函数调用，无法内联
}
```

### 10.3 性能基准

```
场景              | 性能
------------------|---------------------------
函数调用          | 最快（可内联）
闭包（无捕获）     | 与函数相同
闭包（不可变捕获） | 很快（栈分配）
闭包（可变捕获）   | 很快（栈分配）
闭包（move）      | 取决于捕获的数据
Box<dyn Fn>       | 较慢（堆分配 + 虚函数调用）
```

---

## 11. 最佳实践

### 11.1 何时使用函数

```rust
// ✓ 使用函数的场景

// 1. 不需要捕获环境
fn process_data(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 2. 公共 API
pub fn calculate(x: i32, y: i32) -> i32 {
    x + y
}

// 3. 递归
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 4. 需要函数指针数组
let operations: [fn(i32) -> i32; 3] = [
    |x| x + 1,  // ❌ 错误
];
```

### 11.2 何时使用闭包

```rust
// ✓ 使用闭包的场景

// 1. 需要捕获环境
let threshold = 10;
let filtered: Vec<_> = data.iter()
    .filter(|&&x| x > threshold)
    .collect();

// 2. 作为回调
button.on_click(|| {
    println!("Button clicked!");
});

// 3. 迭代器操作
numbers.iter().map(|x| x * 2)

// 4. 惰性求值
let lazy = || expensive_computation();

// 5. 临时、内联使用
vec![1, 2, 3].iter().for_each(|x| println!("{}", x));
```

### 11.3 参数选择

```rust
// 优先使用泛型（静态分派）
fn process<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    // ...
}

// 需要灵活性时使用 trait object
fn process_dynamic(f: &dyn Fn(i32) -> i32) {
    // ...
}

// 只接受函数，不接受闭包
fn process_fn_only(f: fn(i32) -> i32) {
    // ...
}
```

### 11.4 返回闭包

```rust
// 推荐：使用 impl Trait
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// 需要 trait object 时使用 Box
fn make_operation(op: char) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        '+' => Box::new(|a, b| a + b),
        '*' => Box::new(|a, b| a * b),
        _ => Box::new(|a, b| a + b),
    }
}
```

---

## 12. 常见陷阱

### 12.1 闭包类型不匹配

```rust
let c1 = |x: i32| x;
let c2 = |x: i32| x;

let mut f = c1;
// f = c2;  // ❌ 错误：类型不同

// 解决：使用 trait object
let mut f: Box<dyn Fn(i32) -> i32> = Box::new(c1);
f = Box::new(c2);  // ✓ 可以
```

### 12.2 闭包借用冲突

```rust
let mut v = vec![1, 2, 3];

// ❌ 错误：闭包借用了 v，但后面又修改了 v
// let c = || v.len();
// v.push(4);
// println!("{}", c());

// ✓ 解决 1：缩小闭包作用域
{
    let c = || v.len();
    println!("{}", c());
}
v.push(4);

// ✓ 解决 2：使用 move（如果不需要原变量）
let c = move || v.len();
// v.push(4);  // v 已被移动
```

### 12.3 move 的误用

```rust
let s = String::from("hello");

// ❌ 错误：s 被移动，无法多次使用
// let c1 = move || s;
// let c2 = move || s;  // s 已被移动

// ✓ 解决：克隆
let c1 = {
    let s_clone = s.clone();
    move || s_clone
};
let c2 = move || s;

// ✓ 或使用引用计数
use std::rc::Rc;
let s = Rc::new(String::from("hello"));
let c1 = {
    let s_clone = s.clone();
    move || s_clone
};
let c2 = move || s;
```

### 12.4 生命周期问题

```rust
// ❌ 错误：返回的闭包捕获了局部变量的引用
// fn make_closure() -> impl Fn() {
//     let x = 10;
//     || println!("{}", x)  // x 的生命周期不够长
// }

// ✓ 解决：使用 move
fn make_closure() -> impl Fn() {
    let x = 10;
    move || println!("{}", x)  // x 被移动到闭包
}
```

---

## 13. 决策树

### 何时使用函数？

```
需要捕获环境？
├─ 否 → 使用函数 (fn)
└─ 是 → 使用闭包 (||)

是公共 API？
├─ 是 → 使用函数
└─ 否 → 根据情况选择

需要递归？
├─ 是 → 使用函数
└─ 否 → 根据情况选择

需要函数指针数组？
├─ 是 → 使用函数
└─ 否 → 根据情况选择
```

### 何时使用哪种闭包 Trait？

```
是否修改捕获的变量？
├─ 是 → 是否消耗变量？
│  ├─ 是 → FnOnce
│  └─ 否 → FnMut
└─ 否 → Fn
```

---

## 14. 速查表

### 语法对比

```rust
// 函数
fn name(x: i32) -> i32 { x + 1 }

// 闭包（完整）
let closure = |x: i32| -> i32 { x + 1 };

// 闭包（简化）
let closure = |x| x + 1;
```

### 特性对比

| 特性 | fn | \|\| |
|------|----|----|
| 命名 | 必须 | 可选 |
| 类型推断 | 否 | 是 |
| 捕获环境 | 否 | 是 |
| 大小 | 0 | 取决于捕获 |
| 可内联 | 是 | 是 |

### 常用模式

```rust
// 迭代器
.map(|x| ...)
.filter(|x| ...)
.fold(init, |acc, x| ...)

// 回调
on_event(|| ...)

// 惰性求值
let lazy = || expensive();

// 函数组合
compose(f, g)
```

---

**运行示例：** `cargo run --example function_closure_demo`
