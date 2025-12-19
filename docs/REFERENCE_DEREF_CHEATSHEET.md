# Rust 引用与解引用速查表

## 快速索引

| 操作 | 语法 | 类型转换 | 说明 |
|------|------|----------|------|
| **取引用** | `&v` | `T → &T` | 借用，不转移所有权 |
| **可变引用** | `&mut v` | `T → &mut T` | 可变借用 |
| **解引用** | `*r` | `&T → T` | 获取引用指向的值 |
| **双引用** | `&&v` | `&T → &&T` | 引用的引用 |
| **双解引用** | `**rr` | `&&T → T` | 两次解引用 |

---

## 1. 基础操作

### 创建引用

```rust
let x = 5;
let r = &x;        // 不可变引用
let mr = &mut x;   // 可变引用（需要 mut x）
```

### 解引用

```rust
let x = 5;
let r = &x;
let y = *r;        // y = 5
```

### 修改值

```rust
let mut x = 5;
let mr = &mut x;
*mr += 10;         // x = 15
```

---

## 2. 类型层级

```
T           原始值（所有权）
&T          不可变引用（共享）
&mut T      可变引用（独占）
&&T         双重引用
&&&T        三重引用
```

---

## 3. 常见场景

### 场景 1：函数参数

```rust
// ❌ 不好：转移所有权
fn process(v: Vec<i32>) { }

// ✓ 好：借用
fn process(v: &Vec<i32>) { }

// ✓ 更好：使用切片
fn process(v: &[i32]) { }
```

### 场景 2：迭代器

```rust
let v = vec![1, 2, 3];

// iter() 返回 &i32
for item in v.iter() {
    println!("{}", item);  // item: &i32
}

// filter 参数是 &&i32
v.iter().filter(|&&x| x > 1)
```

### 场景 3：模式匹配

```rust
let x = 5;
let r = &x;

match r {
    &val => println!("{}", val),  // 解引用
}
```

---

## 4. 迭代器中的引用

| 方法 | 返回类型 | 闭包参数 |
|------|----------|----------|
| `iter()` | `&T` | `&T` |
| `iter_mut()` | `&mut T` | `&mut T` |
| `into_iter()` | `T` | `T` |

### Filter

```rust
v.iter().filter(|&&x| x > 0)  // 双重解引用
```

### Map

```rust
v.iter().map(|&x| x * 2)      // 单解引用
```

### 链式调用

```rust
v.iter()
    .filter(|&&x| x > 2)       // &&i32
    .map(|&x| x * 2)           // &i32
    .collect()
```

---

## 5. 自动解引用

### 适用场景

```rust
let x = 5;
let r = &x;

// ✓ 自动解引用
println!("{}", r);        // Display
r.count_ones();           // 方法调用
assert_eq!(r, &5);        // 某些比较

// ❌ 需要显式解引用
let y = r + 1;            // 算术运算 ❌
let y = *r + 1;           // ✓
```

---

## 6. Option 和 Result

### Option<T> vs Option<&T>

```rust
let x = 42;

// 拥有所有权
let opt1: Option<i32> = Some(x);

// 借用
let opt2: Option<&i32> = Some(&x);

// 转换
opt1.as_ref()  // Option<i32> → Option<&i32>
opt2.map(|&x| x)  // Option<&i32> → Option<i32>
```

---

## 7. 常见错误

### 错误 1：忘记解引用

```rust
let v = vec![1, 2, 3];

// ❌ 错误
v.iter().filter(|x| x % 2 == 0)

// ✓ 正确
v.iter().filter(|&&x| x % 2 == 0)
```

### 错误 2：双重引用混淆

```rust
let x = 5;
let r = &x;
let rr = &r;

// ❌ 错误
*rr + 1  // *rr 是 &i32

// ✓ 正确
**rr + 1  // **rr 是 i32
```

### 错误 3：自动解引用过度依赖

```rust
let x = &10;

// ❌ 不能用于算术
x + 1

// ✓ 需要显式
*x + 1
```

---

## 8. 模式匹配解引用

### 基础模式

```rust
let r = &5;

// 模式 1：解引用模式
match r {
    &val => println!("{}", val),
}

// 模式 2：if let
if let &val = r {
    println!("{}", val);
}
```

### 复杂模式

```rust
let v = vec![1, 2, 3];

// filter 双重解引用
v.iter().filter(|&&x| x > 1)

// 等价显式写法
v.iter().filter(|x| **x > 1)
```

---

## 9. 性能考虑

| 操作 | 成本 | 说明 |
|------|------|------|
| `&v` | O(1) | 仅复制指针 |
| `*v` | O(1) | 仅访问内存 |
| `clone()` | O(n) | 深拷贝数据 |

### 优化建议

```rust
// ❌ 不必要的克隆
fn process(v: Vec<i32>) {
    for item in v { }
}

// ✓ 使用引用
fn process(v: &[i32]) {
    for item in v { }
}
```

---

## 10. 决策树

### 何时使用 &v？

```
需要访问但不修改？
├─ 是 → &v
└─ 否 → 需要修改？
   ├─ 是 → &mut v
   └─ 否 → v（获取所有权）
```

### 何时使用 *v？

```
v 是引用？
├─ 是 → 需要值本身？
│  ├─ 算术运算 → 必须 *v
│  ├─ 方法调用 → 自动解引用
│  └─ 打印 → 自动解引用
└─ 否 → 不需要
```

---

## 11. 内存布局

```
值：     x = 5
        ┌────┐
     x  │ 5  │ (4 字节)
        └────┘

引用：   r = &x
        ┌────┐
     x  │ 5  │
        └────┘
         ↑
        ┌┴───┐
     r  │ptr │ (8 字节)
        └────┘

双引用： rr = &r
        ┌────┐
     x  │ 5  │
        └────┘
         ↑
        ┌┴───┐
     r  │ptr │ ←─┐
        └────┘   │
                ┌┴───┐
    rr          │ptr │ (8 字节)
                └────┘
```

---

## 12. 实用技巧

### 技巧 1：切片优于 Vec 引用

```rust
// 不好
fn process(v: &Vec<i32>) { }

// 好
fn process(v: &[i32]) { }
```

### 技巧 2：迭代器模式匹配

```rust
// 简洁
v.iter().filter(|&&x| x > 0)

// 清晰
v.iter().filter(|x| **x > 0)
```

### 技巧 3：Option 引用转换

```rust
let opt: Option<i32> = Some(5);

opt.as_ref()     // Option<&i32>
opt.as_mut()     // Option<&mut i32>
```

---

## 13. 常用组合

```rust
// 过滤 + 映射
v.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)

// 查找
v.iter().find(|&&x| x == target)

// 排序
v.sort_by(|&&a, &&b| a.cmp(&b))

// HashMap 访问
map.get(key)  // Option<&V>
```

---

## 14. 记忆口诀

```
& 向左借，不移交
* 向右取，拿到手
&& 双层包，层层嵌
** 双重取，逐层拆
```

---

## 快速参考

| 想要 | 使用 | 示例 |
|------|------|------|
| 借用值 | `&v` | `let r = &x;` |
| 可变借用 | `&mut v` | `let mr = &mut x;` |
| 获取值 | `*r` | `let y = *r;` |
| 修改值 | `*mr = ...` | `*mr += 1;` |
| 过滤 | `\|&&x\|` | `filter(\|&&x\| ...)` |
| 映射 | `\|&x\|` | `map(\|&x\| ...)` |

---

**运行示例：** `cargo run --example reference_deref_demo`  
**详细文档：** `docs/REFERENCE_DEREF_GUIDE.md`
