# Rust 函数与闭包速查表

## 快速对比

| 特性 | fn 函数 | \|\| 闭包 |
|------|---------|----------|
| **命名** | 必须有名字 | 匿名（可绑定） |
| **语法** | `fn name() {}` | `\|args\| expr` |
| **类型推断** | 需要显式类型 | 自动推断 |
| **环境捕获** | ❌ 不能 | ✅ 可以 |
| **大小** | 0 字节（ZST） | 取决于捕获 |
| **内联优化** | ✅ 可以 | ✅ 可以 |
| **递归** | ✅ 可以 | ❌ 困难 |

---

## 1. 基础语法

### 函数

```rust
// 完整语法
fn name(param: Type) -> ReturnType {
    // body
}

// 示例
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### 闭包

```rust
// 完整语法
|param: Type| -> ReturnType { body }

// 类型推断
|param| expr

// 示例
let add = |x, y| x + y;
```

---

## 2. 闭包捕获模式

### 三种捕获方式

```rust
let x = 5;
let s = String::from("hello");

// 1. 不可变借用 (&T)
let borrow = || println!("{} {}", x, s);

// 2. 可变借用 (&mut T)
let mut count = 0;
let mut mutate = || count += 1;

// 3. 获取所有权 (move)
let consume = move || drop(s);
```

### move 关键字

```rust
// 不使用 move：借用
let x = 10;
let c1 = || x;        // 借用 x
println!("{}", x);    // ✓ x 仍可用

// 使用 move：所有权转移
let c2 = move || x;   // 移动 x
println!("{}", x);    // ✓ i32 是 Copy

let s = String::from("hello");
let c3 = move || s;   // 移动 s
// println!("{}", s); // ❌ s 已被移动
```

---

## 3. 闭包 Trait

### Trait 层级

```
Fn ⊂ FnMut ⊂ FnOnce

Fn:      不修改环境，可多次调用
FnMut:   可修改环境，可多次调用
FnOnce:  可能消耗环境，至少调用一次
```

### 判断规则

```rust
// Fn: 只读捕获
let x = 5;
let fn_closure = || x;

// FnMut: 修改捕获
let mut count = 0;
let mut fn_mut_closure = || count += 1;

// FnOnce: 消耗捕获
let s = String::from("hello");
let fn_once_closure = || drop(s);
```

### 作为参数

```rust
// 接受 Fn
fn call_fn<F: Fn()>(f: F) { f(); }

// 接受 FnMut
fn call_fn_mut<F: FnMut()>(mut f: F) { f(); }

// 接受 FnOnce
fn call_fn_once<F: FnOnce()>(f: F) { f(); }
```

---

## 4. 函数指针

### fn 类型

```rust
// 函数可以转换为 fn
fn double(x: i32) -> i32 { x * 2 }
let f: fn(i32) -> i32 = double;  // ✓

// 闭包不能转换为 fn
let c: fn(i32) -> i32 = |x| x * 2;  // ❌
```

### fn 实现所有闭包 Trait

```rust
fn add(x: i32, y: i32) -> i32 { x + y }

// fn 可以用于需要 Fn 的地方
fn apply<F: Fn(i32, i32) -> i32>(f: F) -> i32 {
    f(1, 2)
}

apply(add);  // ✓
```

---

## 5. 高阶函数

### 返回闭包

```rust
// 使用 impl Trait（推荐）
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// 使用 Box<dyn Fn>
fn make_op(op: char) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        '+' => Box::new(|a, b| a + b),
        '*' => Box::new(|a, b| a * b),
        _ => Box::new(|a, b| a + b),
    }
}
```

### 接受闭包

```rust
// 泛型（静态分派，零成本）
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

// trait object（动态分派）
fn apply_dyn(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
```

---

## 6. 常用模式

### 迭代器

```rust
let v = vec![1, 2, 3, 4, 5];

// map
v.iter().map(|&x| x * 2)

// filter
v.iter().filter(|&&x| x > 2)

// fold
v.iter().fold(0, |acc, &x| acc + x)

// for_each
v.iter().for_each(|x| println!("{}", x))
```

### 链式调用

```rust
v.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .take(3)
    .collect::<Vec<_>>()
```

### 回调

```rust
struct Button<F: Fn()> {
    on_click: F,
}

impl<F: Fn()> Button<F> {
    fn click(&self) {
        (self.on_click)();
    }
}
```

---

## 7. 决策树

### 何时使用函数？

```
需要捕获环境？
├─ 否 → 使用函数
└─ 是 → 使用闭包

需要递归？
├─ 是 → 使用函数
└─ 否 → 根据情况

是公共 API？
├─ 是 → 使用函数
└─ 否 → 根据情况

需要函数指针数组？
├─ 是 → 使用函数
└─ 否 → 根据情况
```

### 选择闭包 Trait

```
是否修改捕获的变量？
├─ 是 → 是否消耗？
│  ├─ 是 → FnOnce
│  └─ 否 → FnMut
└─ 否 → Fn
```

---

## 8. 常见场景

### 场景 1: 迭代器操作

```rust
// 推荐：闭包
v.iter().map(|x| x * 2)

// 可选：函数
fn double(x: &i32) -> i32 { x * 2 }
v.iter().map(double)
```

### 场景 2: 配置/策略

```rust
// 推荐：闭包
let strategy = |a, b| a + b;

// 或：函数
fn add(a: i32, b: i32) -> i32 { a + b }
```

### 场景 3: 回调

```rust
// 推荐：闭包（可捕获环境）
button.on_click(|| {
    counter += 1;
});
```

### 场景 4: 工厂函数

```rust
// 推荐：返回闭包
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
```

---

## 9. 性能提示

### 零成本抽象

```rust
// 闭包（会被内联）
let sum: i32 = v.iter().map(|x| x * 2).sum();

// 等价于手写循环（性能相同）
let mut sum = 0;
for x in &v {
    sum += x * 2;
}
```

### 静态 vs 动态分派

```rust
// 静态分派（推荐，零成本）
fn apply<F: Fn(i32) -> i32>(f: F) { }

// 动态分派（有开销）
fn apply(f: &dyn Fn(i32) -> i32) { }
```

### 大小考虑

```rust
// 无捕获：0 字节
let c1 = || {};

// 捕获 i32：4 字节
let x = 10;
let c2 = || x;

// 捕获 String：24 字节
let s = String::from("hello");
let c3 = || s;
```

---

## 10. 常见错误

### 错误 1: 闭包类型不匹配

```rust
let c1 = |x: i32| x;
let c2 = |x: i32| x;
let mut f = c1;
// f = c2;  // ❌ 类型不同

// 解决：trait object
let mut f: Box<dyn Fn(i32) -> i32> = Box::new(c1);
f = Box::new(c2);  // ✓
```

### 错误 2: 借用冲突

```rust
let mut v = vec![1, 2, 3];
let c = || v.len();
// v.push(4);  // ❌ v 被借用

// 解决：缩小作用域
{
    let c = || v.len();
    c();
}
v.push(4);  // ✓
```

### 错误 3: move 误用

```rust
let s = String::from("hello");
let c1 = move || s;
// let c2 = move || s;  // ❌ s 已移动

// 解决：克隆
let c1 = {
    let s_clone = s.clone();
    move || s_clone
};
let c2 = move || s;  // ✓
```

---

## 11. 实用技巧

### 技巧 1: 函数组合

```rust
fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

let f = compose(|x| x + 1, |x| x * 2);
```

### 技巧 2: 惰性求值

```rust
struct Lazy<F: FnOnce() -> T, T> {
    init: Option<F>,
    value: Option<T>,
}

impl<F: FnOnce() -> T, T> Lazy<F, T> {
    fn get(&mut self) -> &T {
        if self.value.is_none() {
            let init = self.init.take().unwrap();
            self.value = Some(init());
        }
        self.value.as_ref().unwrap()
    }
}
```

### 技巧 3: 使用 move 避免生命周期问题

```rust
// ❌ 错误
fn make_closure() -> impl Fn() {
    let x = 10;
    || println!("{}", x)  // x 生命周期不够
}

// ✓ 正确
fn make_closure() -> impl Fn() {
    let x = 10;
    move || println!("{}", x)
}
```

---

## 12. 速记口诀

```
函数有名闭包匿，
捕获环境看需要。
Fn 只读能复用，
FnMut 改值也不愁。
FnOnce 一次就消耗，
move 关键转所有。
```

---

## 13. 快速参考

### 定义

```rust
fn func(x: i32) -> i32 { x }     // 函数
let closure = |x| x;              // 闭包
let closure = |x: i32| -> i32 { x }; // 完整闭包
```

### 调用

```rust
func(5);
closure(5);
(closure)(5);  // 显式调用
```

### 传递

```rust
fn take_fn<F: Fn(i32) -> i32>(f: F) {}
take_fn(func);      // 函数
take_fn(closure);   // 闭包
take_fn(|x| x);     // 内联闭包
```

### 返回

```rust
fn ret_fn() -> fn(i32) -> i32 { func }
fn ret_closure() -> impl Fn(i32) -> i32 { |x| x }
fn ret_box() -> Box<dyn Fn(i32) -> i32> { Box::new(|x| x) }
```

---

**运行示例：** `cargo run --example function_closure_demo`  
**详细文档：** `docs/FUNCTION_CLOSURE_GUIDE.md`
