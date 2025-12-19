# Rust 类型标注与泛型声明指南

## 一、let 何时需要声明类型

### ✅ 不需要声明的情况（编译器自动推断）

```rust
// 1. 简单字面量 - 编译器知道类型
let x = 5;              // i32
let y = 3.14;           // f64
let z = true;           // bool
let s = "hello";        // &str

// 2. 从函数返回值推断
let s = String::from("hello");  // String

// 3. 从后续使用推断
let mut vec = Vec::new();  // 类型未知
vec.push(1);               // 现在知道是 Vec<i32>
```

### ❌ 必须声明的情况

#### 1️⃣ 延迟初始化

```rust
// ❌ 错误
let x;
x = 5;

// ✅ 正确
let x: i32;
x = 5;
```

#### 2️⃣ 有多种可能的类型（歧义）

```rust
// ❌ 错误 - collect 可以收集到多种类型
let numbers = [1, 2, 3].iter().collect();

// ✅ 正确 - 方式A：变量类型标注
let numbers: Vec<i32> = [1, 2, 3].iter().copied().collect();

// ✅ 正确 - 方式B：Turbofish 语法
let numbers = [1, 2, 3].iter().copied().collect::<Vec<i32>>();
```

#### 3️⃣ 空容器（无法从元素推断）

```rust
// ❌ 错误 - 不知道存储什么类型
let v = Vec::new();

// ✅ 正确
let v: Vec<String> = Vec::new();
let v = Vec::<String>::new();
```

#### 4️⃣ parse 等有多个实现的方法

```rust
let s = "42";

// ❌ 错误 - parse 可以解析为多种数字类型
let num = s.parse().unwrap();

// ✅ 正确 - 方式A
let num: i32 = s.parse().unwrap();

// ✅ 正确 - 方式B
let num = s.parse::<i32>().unwrap();
```

#### 5️⃣ 多个 trait 实现

```rust
let data = "5.5";

// 需要指定是 f64 还是 f32
let f: f64 = data.parse().unwrap();
// 或
let f = data.parse::<f32>().unwrap();
```

---

## 二、泛型类型声明

### 1. 结构体泛型

```rust
// 声明泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

// 使用时指定类型
let p1: Point<i32> = Point { x: 5, y: 10 };
let p2: Point<f64> = Point { x: 1.0, y: 4.0 };

// 多个泛型参数
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair: Pair<i32, String> = Pair {
    first: 42,
    second: String::from("hello"),
};
```

### 2. 枚举泛型

```rust
// 标准库的 Option
enum Option<T> {
    Some(T),
    None,
}

let some_num: Option<i32> = Some(5);
let no_val: Option<i32> = None;

// 标准库的 Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}

let result: Result<i32, String> = Ok(42);
```

### 3. 函数泛型

```rust
// 声明泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 使用
let numbers = vec![34, 50, 25, 100, 65];
let max = largest(&numbers);  // 编译器推断 T = i32
```

### 4. impl 块泛型

```rust
struct Container<T> {
    value: T,
}

// 泛型 impl
impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
}

// 特定类型的 impl
impl Container<i32> {
    fn is_positive(&self) -> bool {
        self.value > 0
    }
}
```

---

## 三、Turbofish 语法 `::<T>`

Turbofish 用于在方法调用时显式指定泛型类型参数。

### 常见场景

```rust
// 1. collect 方法
let v = vec![1, 2, 3]
    .iter()
    .copied()
    .collect::<Vec<i32>>();

// 2. parse 方法
let n = "42".parse::<i32>().unwrap();

// 3. from_iter
let set = HashSet::<i32>::from_iter(vec![1, 2, 3]);

// 4. 链式调用
let result = "1,2,3"
    .split(',')
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
```

### Turbofish vs 变量标注

```rust
// 方式A：变量类型标注
let numbers: Vec<i32> = input.iter().copied().collect();

// 方式B：Turbofish（方法调用时指定）
let numbers = input.iter().copied().collect::<Vec<i32>>();

// 两种方式等价，选择更清晰的一种
```

---

## 四、复杂泛型场景

### 1. 带生命周期的泛型

```rust
struct Wrapper<'a, T> {
    value: &'a T,
}

let num = 42;
let wrapper: Wrapper<i32> = Wrapper { value: &num };
```

### 2. 带 trait 约束

```rust
// 简单约束
fn print<T: Debug>(value: T) {
    println!("{:?}", value);
}

// 多个约束
fn compare<T: PartialEq + Debug>(a: T, b: T) -> bool {
    println!("比较 {:?} 和 {:?}", a, b);
    a == b
}

// where 子句（更清晰）
fn process<T, U>(a: T, b: U) -> String
where
    T: Display + Clone,
    U: Debug,
{
    format!("{} {:?}", a, b)
}
```

### 3. 关联类型

```rust
use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

let sum = add(5, 3);        // i32
let sum = add(1.5, 2.5);    // f64
```

### 4. 嵌套泛型

```rust
// Vec 嵌套
let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];

// HashMap 嵌套
let map: HashMap<String, Vec<i32>> = HashMap::new();

// Option 嵌套
let opt: Option<Result<i32, String>> = Some(Ok(42));
```

---

## 五、实战建议

### ✅ 最佳实践

1. **优先让编译器推断**
   ```rust
   // 好 - 类型显而易见
   let name = String::from("Alice");
   let age = 30;
   ```

2. **歧义时明确标注**
   ```rust
   // 好 - 避免歧义
   let numbers: Vec<i32> = data.iter().copied().collect();
   ```

3. **复杂类型使用类型别名**
   ```rust
   type Result<T> = std::result::Result<T, Box<dyn Error>>;
   type Cache = HashMap<String, Vec<User>>;
   ```

4. **API 设计中明确类型**
   ```rust
   // 好 - 公共 API 明确类型
   pub fn process(data: Vec<u8>) -> Result<String, Error> {
       // ...
   }
   ```

### ❌ 避免

1. **过度标注**
   ```rust
   // 不好 - 不必要的标注
   let x: i32 = 5;
   let s: String = String::from("hello");
   ```

2. **类型标注不一致**
   ```rust
   // 混乱 - 有时用变量标注，有时用 turbofish
   let v1: Vec<i32> = data.collect();
   let v2 = data.collect::<Vec<i32>>();
   ```

---

## 六、速查表

| 场景 | 需要标注 | 示例 |
|------|---------|------|
| 字面量 | ❌ | `let x = 5;` |
| 函数返回值 | ❌ | `let s = String::from("hi");` |
| 延迟初始化 | ✅ | `let x: i32; x = 5;` |
| collect() | ✅ | `let v: Vec<i32> = iter.collect();` |
| parse() | ✅ | `let n: i32 = s.parse()?;` |
| 空容器 | ✅ | `let v: Vec<String> = Vec::new();` |
| 泛型结构体 | ✅ | `let p: Point<i32> = Point { x: 1, y: 2 };` |

---

## 七、常见错误与解决

### 错误1：类型无法推断

```rust
// ❌ 错误
error[E0282]: type annotations needed
let v = Vec::new();

// ✅ 解决
let v: Vec<i32> = Vec::new();
let v = Vec::<i32>::new();
```

### 错误2：多个实现冲突

```rust
// ❌ 错误
error[E0283]: type annotations needed
let n = "42".parse().unwrap();

// ✅ 解决
let n: i32 = "42".parse().unwrap();
let n = "42".parse::<i32>().unwrap();
```

### 错误3：生命周期标注缺失

```rust
// ❌ 错误
struct Wrapper<T> {
    value: &T,  // 缺少生命周期
}

// ✅ 解决
struct Wrapper<'a, T> {
    value: &'a T,
}
```

---

## 八、总结

### 核心原则

1. **让编译器推断** - 能推断就不标注
2. **歧义必须明确** - 多种可能时必须指定
3. **公共 API 明确** - 提高代码可读性
4. **保持一致性** - 团队统一风格

### 记忆口诀

```
字面值不用标，函数能推断
歧义必明确，空容器要管
parse 和 collect，类型要指点
泛型看场景，Turbofish 不乱
```

---

运行完整示例：
```bash
cargo run --example type_annotations
```
