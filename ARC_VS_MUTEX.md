# Arc vs Mutex 深度对比指南

## 🎯 核心概念

### Arc (Atomic Reference Counted)
**原子引用计数 - 解决所有权共享问题**

```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data); // 引用计数 +1
// 两个变量指向同一份数据
```

**特点**:
- ✅ 允许多个所有者共享数据
- ✅ 线程安全（引用计数是原子操作）
- ❌ 不提供数据修改能力（只读）
- 📊 开销：原子操作 + 额外内存（引用计数）

### Mutex (Mutual Exclusion)
**互斥锁 - 解决数据竞争问题**

```rust
use std::sync::Mutex;

let data = Mutex::new(vec![1, 2, 3]);
let mut guard = data.lock().unwrap();
guard.push(4); // 可以修改
// guard drop 时自动释放锁
```

**特点**:
- ✅ 保护数据，防止并发修改
- ✅ 提供内部可变性
- ✅ 自动解锁（RAII）
- 📊 开销：锁获取/释放 + 可能的线程阻塞

---

## 📊 对比表格

| 特性 | Arc<T> | Mutex<T> | Arc<Mutex<T>> |
|------|--------|----------|---------------|
| **主要用途** | 共享所有权 | 保护数据 | 共享可变数据 |
| **是否可变** | ❌ 不可变 | ✅ 可变 | ✅ 可变 |
| **跨线程** | ✅ 支持 | ⚠️ 不可直接跨线程 | ✅ 支持 |
| **并发读取** | ✅ 无限制 | ❌ 独占 | ❌ 独占 |
| **并发写入** | N/A | ❌ 独占 | ❌ 独占 |
| **性能开销** | 低 | 中 | 中高 |
| **典型场景** | 只读共享 | 单线程可变 | 多线程可变 |

---

## 🔄 使用场景决策树

```
需要跨线程访问？
├─ 否 → 使用普通引用 &T 或 Box<T>
└─ 是 ↓
    需要修改数据？
    ├─ 否 → Arc<T>
    └─ 是 ↓
        读多写少？
        ├─ 是 → Arc<RwLock<T>>
        └─ 否 → Arc<Mutex<T>>
```

---

## 💡 实战示例

### 示例 1: 只读共享（Arc）

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3, 4, 5]);
let mut handles = vec![];

for i in 0..3 {
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("线程 {}: {:?}", i, data);
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

**适用**: 配置数据、只读缓存、不可变状态

### 示例 2: 多线程计数器（Arc + Mutex）

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("结果: {}", counter.lock().unwrap());
```

**适用**: 共享状态、计数器、队列

### 示例 3: 读多写少（Arc + RwLock）

```rust
use std::sync::{Arc, RwLock};
use std::thread;

let cache = Arc::new(RwLock::new(HashMap::new()));

// 写入（独占）
{
    let mut cache = cache.write().unwrap();
    cache.insert("key", "value");
}

// 多个读取可以并发
let mut handles = vec![];
for i in 0..5 {
    let cache = Arc::clone(&cache);
    let handle = thread::spawn(move || {
        let cache = cache.read().unwrap();
        println!("线程 {}: {:?}", i, cache.get("key"));
    });
    handles.push(handle);
}
```

**适用**: 缓存、配置、读多写少的共享数据

---

## ⚠️ 常见陷阱

### 陷阱 1: 忘记释放锁（死锁）

```rust
// ❌ 错误
let data = Mutex::new(5);
let guard = data.lock().unwrap();
// 忘记释放锁
let guard2 = data.lock().unwrap(); // 死锁！

// ✅ 正确
{
    let guard = data.lock().unwrap();
    // 使用数据
} // guard 自动 drop，锁释放
```

### 陷阱 2: 锁内耗时操作

```rust
// ❌ 错误
{
    let mut data = counter.lock().unwrap();
    *data += 1;
    expensive_operation(); // 锁被长时间持有
}

// ✅ 正确
{
    let mut data = counter.lock().unwrap();
    *data += 1;
} // 先释放锁
expensive_operation(); // 在锁外执行
```

### 陷阱 3: 锁顺序不一致（死锁）

```rust
// ❌ 可能死锁
// 线程 A: lock(m1) -> lock(m2)
// 线程 B: lock(m2) -> lock(m1)

// ✅ 正确：始终以相同顺序获取锁
// 线程 A: lock(m1) -> lock(m2)
// 线程 B: lock(m1) -> lock(m2)
```

### 陷阱 4: 过度使用 Arc

```rust
// ❌ 不必要的 Arc
fn process(data: Arc<Vec<i32>>) {
    // 函数内部不需要 Arc
}

// ✅ 使用引用
fn process(data: &[i32]) {
    // 更高效
}
```

---

## 🚀 性能优化建议

### 1. 简单类型用 Atomic

```rust
// ❌ 开销较大
let counter = Arc::new(Mutex::new(0));

// ✅ 更高效
use std::sync::atomic::{AtomicUsize, Ordering};
let counter = Arc::new(AtomicUsize::new(0));
counter.fetch_add(1, Ordering::SeqCst);
```

### 2. 读多写少用 RwLock

```rust
// 读操作频繁，写操作少
let cache = Arc::new(RwLock::new(HashMap::new()));

// 多个读取可以并发
let data = cache.read().unwrap();

// 写入时独占
let mut data = cache.write().unwrap();
```

### 3. 减小锁的粒度

```rust
// ❌ 锁粒度太大
struct Data {
    counter: i32,
    list: Vec<i32>,
}
let data = Arc::new(Mutex::new(Data { ... }));

// ✅ 分开锁
struct Data {
    counter: Arc<AtomicI32>,
    list: Arc<Mutex<Vec<i32>>>,
}
```

---

## 📝 选择清单

选择 **Arc<T>** 当：
- ✅ 多线程只读访问
- ✅ 不需要修改数据
- ✅ 需要共享所有权
- 例：配置、常量、只读缓存

选择 **Mutex<T>** 当：
- ✅ 单线程或单所有者
- ✅ 需要内部可变性
- ✅ 不需要跨线程传递
- 例：局部状态、缓冲区

选择 **Arc<Mutex<T>>** 当：
- ✅ 多线程共享
- ✅ 需要修改数据
- ✅ 写操作频繁
- 例：共享状态、计数器、队列

选择 **Arc<RwLock<T>>** 当：
- ✅ 多线程共享
- ✅ 读多写少
- ✅ 读操作可以并发
- 例：缓存、配置、索引

选择 **Arc<AtomicXxx>** 当：
- ✅ 简单类型（整数、布尔）
- ✅ 频繁读写
- ✅ 需要最高性能
- 例：计数器、标志位

---

## 🎓 记忆口诀

```
Arc 管所有，Mutex 保安全
组合使用最常见，读写分离用 RwLock
简单计数 Atomic，复杂数据 Mutex 锁
锁内快进出，避免嵌套防死锁
```

---

## 🔗 相关资源

- **运行示例**: `cargo run --bin arc_vs_mutex`
- **源代码**: `src/arc_vs_mutex.rs`
- **官方文档**: 
  - [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
  - [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
  - [RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html)

---

## 📊 快速对比总结

| 需求 | 方案 | 理由 |
|------|------|------|
| 多线程只读 | `Arc<T>` | 无需锁，性能最好 |
| 多线程可变 | `Arc<Mutex<T>>` | 标准方案 |
| 读多写少 | `Arc<RwLock<T>>` | 读操作可并发 |
| 简单计数 | `Arc<AtomicUsize>` | 无锁，最快 |
| 单线程可变 | `Mutex<T>` | 不需要 Arc |
| 不可变数据 | `&T` | 不需要任何包装 |

**最后更新**: 2025-12-22
