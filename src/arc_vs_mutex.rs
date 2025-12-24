// Arc VS Mutex 详解
// 
// Arc 和 Mutex 是 Rust 并发编程中的两个核心概念，但它们解决的是不同的问题

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Arc VS Mutex 深度解析 ===\n");
    
    // 1. 基础概念对比
    demo_basic_concepts();
    
    // 2. Arc - 原子引用计数（所有权共享）
    demo_arc_basics();
    
    // 3. Mutex - 互斥锁（数据保护）
    demo_mutex_basics();
    
    // 4. Arc + Mutex - 组合使用（最常见）
    demo_arc_mutex_combined();
    
    // 5. RwLock - 读写锁（优化读多写少场景）
    demo_rwlock();
    
    // 6. 性能对比
    demo_performance_comparison();
    
    // 7. 常见错误和陷阱
    demo_common_mistakes();
    
    // 8. 实战案例
    demo_real_world_examples();
}

// ============================================
// 1. 基础概念对比
// ============================================
fn demo_basic_concepts() {
    println!("--- 1. Arc vs Mutex 核心区别 ---\n");
    
    println!("Arc (Atomic Reference Counted):");
    println!("  - 作用: 允许多个所有者共享同一份数据");
    println!("  - 解决: 所有权问题");
    println!("  - 线程安全: 是（引用计数是原子操作）");
    println!("  - 数据可变: 否（Arc<T> 提供的是不可变引用）");
    println!("  - 开销: 原子操作开销");
    println!();
    
    println!("Mutex (Mutual Exclusion):");
    println!("  - 作用: 保护数据，同一时刻只有一个线程可以访问");
    println!("  - 解决: 数据竞争问题");
    println!("  - 线程安全: 是（通过锁机制）");
    println!("  - 数据可变: 是（Mutex<T> 提供内部可变性）");
    println!("  - 开销: 锁获取/释放开销");
    println!();
    
    println!("常见组合: Arc<Mutex<T>>");
    println!("  - Arc: 让多个线程都能拥有 Mutex");
    println!("  - Mutex: 让多个线程都能安全修改数据");
    println!();
}

// ============================================
// 2. Arc - 原子引用计数
// ============================================
fn demo_arc_basics() {
    println!("--- 2. Arc 详解 ---\n");
    
    // 问题：不使用 Arc，无法跨线程共享所有权
    println!("问题演示：不使用 Arc");
    println!("  let data = vec![1, 2, 3];");
    println!("  // 错误！data 的所有权无法在多个线程间共享");
    println!("  // thread::spawn(|| println!(\"{{:?}}\", data));");
    println!();
    
    // 解决方案：使用 Arc
    println!("解决方案：使用 Arc\n");
    
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("创建 Arc: {:?}", data);
    println!("引用计数: {}", Arc::strong_count(&data));
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data); // 增加引用计数
        println!("线程 {} 克隆后，引用计数: {}", i, Arc::strong_count(&data));
        
        let handle = thread::spawn(move || {
            // 每个线程都有自己的 Arc 实例，指向同一份数据
            println!("  线程 {} 读取数据: {:?}", i, data_clone);
            thread::sleep(Duration::from_millis(10));
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("所有线程结束后，引用计数: {}", Arc::strong_count(&data));
    println!();
    
    // Arc 的限制：不能修改数据
    println!("Arc 的限制：");
    let shared_data = Arc::new(42);
    println!("  let shared_data = Arc::new(42);");
    println!("  // 错误！无法通过 Arc 修改数据");
    println!("  // *shared_data = 100; // 编译错误");
    println!("  实际值: {}", shared_data);
    println!();
}

// ============================================
// 3. Mutex - 互斥锁
// ============================================
fn demo_mutex_basics() {
    println!("--- 3. Mutex 详解 ---\n");
    
    // 单线程中使用 Mutex
    println!("单线程使用 Mutex:");
    let counter = Mutex::new(0);
    
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
        println!("  锁内修改: {}", *num);
        // 离开作用域，锁自动释放
    }
    
    println!("  锁外读取: {}", counter.lock().unwrap());
    println!();
    
    // Mutex 提供内部可变性
    println!("Mutex 提供内部可变性:");
    println!("  即使 Mutex 本身是不可变的，也能修改内部数据");
    let data = Mutex::new(vec![1, 2, 3]);
    data.lock().unwrap().push(4);
    println!("  修改后: {:?}", data.lock().unwrap());
    println!();
    
    // 锁的获取和释放
    println!("锁的生命周期:");
    let value = Mutex::new(5);
    
    {
        let mut guard = value.lock().unwrap();
        println!("  获取锁，当前值: {}", *guard);
        *guard += 10;
        println!("  修改值: {}", *guard);
        // guard 在这里被 drop，锁被释放
    }
    
    println!("  锁已释放，可以再次获取");
    println!("  新值: {}", value.lock().unwrap());
    println!();
}

// ============================================
// 4. Arc + Mutex 组合使用
// ============================================
fn demo_arc_mutex_combined() {
    println!("--- 4. Arc + Mutex 组合（最常见） ---\n");
    
    println!("为什么需要组合？");
    println!("  - Arc: 让多个线程共享所有权");
    println!("  - Mutex: 让数据可以被安全修改");
    println!("  - Arc<Mutex<T>>: 多个线程共享并修改数据\n");
    
    // 实例：多线程计数器
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    println!("启动 5 个线程，每个线程增加计数器 100 次");
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                // 锁在这里自动释放
            }
            println!("  线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数: {}", counter.lock().unwrap());
    println!("预期: 500 (5 线程 × 100 次)\n");
    
    // 共享状态示例
    println!("共享状态示例 - 购物车:");
    
    #[derive(Debug, Clone)]
    struct Product {
        name: String,
        price: f64,
    }
    
    let cart = Arc::new(Mutex::new(Vec::<Product>::new()));
    let mut handles = vec![];
    
    let products = vec![
        ("苹果", 5.0),
        ("香蕉", 3.0),
        ("橙子", 4.0),
    ];
    
    for (name, price) in products {
        let cart = Arc::clone(&cart);
        let handle = thread::spawn(move || {
            let product = Product {
                name: name.to_string(),
                price,
            };
            cart.lock().unwrap().push(product);
            println!("  添加商品: {}", name);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let cart = cart.lock().unwrap();
    println!("购物车内容: {:?}", *cart);
    let total: f64 = cart.iter().map(|p| p.price).sum();
    println!("总价: {:.2}\n", total);
}

// ============================================
// 5. RwLock - 读写锁
// ============================================
fn demo_rwlock() {
    println!("--- 5. RwLock - 读写锁优化 ---\n");
    
    println!("RwLock vs Mutex:");
    println!("  Mutex: 任何访问都需要独占锁");
    println!("  RwLock: 多个读取可以并发，写入需要独占\n");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // 多个读线程（可以并发）
    println!("启动 3 个读线程（可并发）:");
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data.read().unwrap();
            println!("  读线程 {}: {:?}", i, *reader);
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }
    
    // 1 个写线程（需要独占）
    println!("启动 1 个写线程（独占）:");
    let data_write = Arc::clone(&data);
    let write_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(15));
        let mut writer = data_write.write().unwrap();
        writer.push(6);
        println!("  写线程: 添加元素 6");
    });
    handles.push(write_handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终数据: {:?}", data.read().unwrap());
    println!();
    
    println!("使用场景:");
    println!("  ✓ 读多写少的场景");
    println!("  ✓ 配置数据、缓存");
    println!("  ✗ 写入频繁的场景（开销比 Mutex 大）");
    println!();
}

// ============================================
// 6. 性能对比
// ============================================
fn demo_performance_comparison() {
    println!("--- 6. 性能对比 ---\n");
    
    println!("开销分析:");
    println!();
    
    println!("Arc:");
    println!("  - 原子操作：每次 clone/drop 需要原子增减");
    println!("  - 内存：额外存储引用计数（2 个 usize）");
    println!("  - 适用：需要跨线程共享所有权");
    println!();
    
    println!("Mutex:");
    println!("  - 锁获取：可能导致线程阻塞");
    println!("  - 锁释放：需要唤醒等待线程");
    println!("  - 竞争：高竞争时性能下降明显");
    println!("  - 适用：需要修改共享数据");
    println!();
    
    println!("RwLock:");
    println!("  - 开销：比 Mutex 稍大");
    println!("  - 优势：读操作可以并发");
    println!("  - 劣势：写操作需要等待所有读完成");
    println!("  - 适用：读多写少");
    println!();
    
    println!("选择建议:");
    println!("  1. 只需共享不可变数据: Arc<T>");
    println!("  2. 需要修改共享数据: Arc<Mutex<T>>");
    println!("  3. 读多写少: Arc<RwLock<T>>");
    println!("  4. 单线程可变: RefCell<T>");
    println!();
}

// ============================================
// 7. 常见错误和陷阱
// ============================================
fn demo_common_mistakes() {
    println!("--- 7. 常见错误和陷阱 ---\n");
    
    println!("❌ 错误 1: 忘记释放锁");
    println!("代码示例:");
    println!("  let data = Mutex::new(5);");
    println!("  let guard = data.lock().unwrap();");
    println!("  // 忘记 drop(guard)");
    println!("  // let guard2 = data.lock().unwrap(); // 死锁！");
    println!();
    println!("✅ 正确做法:");
    println!("  {{");
    println!("      let guard = data.lock().unwrap();");
    println!("      // 使用数据");
    println!("  }} // guard 自动 drop");
    println!();
    
    println!("❌ 错误 2: 锁嵌套导致死锁");
    let _mutex1 = Arc::new(Mutex::new(1));
    let _mutex2 = Arc::new(Mutex::new(2));
    
    println!("示例：两个线程以不同顺序获取锁");
    println!("  线程 A: lock(mutex1) -> lock(mutex2)");
    println!("  线程 B: lock(mutex2) -> lock(mutex1)");
    println!("  结果: 可能死锁！");
    println!();
    println!("✅ 解决方案: 始终以相同顺序获取锁");
    println!();
    
    println!("❌ 错误 3: 在锁内做耗时操作");
    println!("代码示例:");
    println!("  let mut data = counter.lock().unwrap();");
    println!("  *data += 1;");
    println!("  thread::sleep(Duration::from_secs(1)); // ❌ 锁被长时间持有");
    println!();
    println!("✅ 正确做法: 尽快释放锁");
    println!("  {{");
    println!("      let mut data = counter.lock().unwrap();");
    println!("      *data += 1;");
    println!("  }} // 立即释放锁");
    println!("  thread::sleep(Duration::from_secs(1)); // ✅ 在锁外执行");
    println!();
    
    println!("❌ 错误 4: 过度使用 Arc");
    println!("  不是所有共享都需要 Arc");
    println!("  - 单线程: 使用引用 &T");
    println!("  - 单所有者: 直接传递所有权");
    println!("  - 函数参数: 使用 &T 或 &mut T");
    println!();
    
    println!("❌ 错误 5: Arc<Mutex<Arc<T>>>（过度嵌套）");
    println!("  通常 Arc<Mutex<T>> 就够了");
    println!();
}

// ============================================
// 8. 实战案例
// ============================================
fn demo_real_world_examples() {
    println!("--- 8. 实战案例 ---\n");
    
    // 案例 1: 线程池任务计数
    println!("案例 1: 线程池任务计数器\n");
    thread_pool_counter();
    
    // 案例 2: 共享缓存
    println!("\n案例 2: 共享缓存（读多写少）\n");
    shared_cache_example();
    
    // 案例 3: 生产者-消费者
    println!("\n案例 3: 简单的生产者-消费者模式\n");
    producer_consumer_example();
}

fn thread_pool_counter() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct TaskCounter {
        completed: AtomicUsize,
        total: usize,
    }
    
    let counter = Arc::new(TaskCounter {
        completed: AtomicUsize::new(0),
        total: 10,
    });
    
    let mut handles = vec![];
    
    for i in 0..counter.total {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 模拟任务
            thread::sleep(Duration::from_millis(10));
            counter.completed.fetch_add(1, Ordering::SeqCst);
            println!("  任务 {} 完成", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("总任务: {}", counter.total);
    println!("已完成: {}", counter.completed.load(Ordering::SeqCst));
    
    println!("\n说明: 使用 AtomicUsize 代替 Mutex<usize> 性能更好");
}

fn shared_cache_example() {
    use std::collections::HashMap;
    
    type Cache = Arc<RwLock<HashMap<String, String>>>;
    
    let cache: Cache = Arc::new(RwLock::new(HashMap::new()));
    
    // 写入数据
    {
        let mut cache_write = cache.write().unwrap();
        cache_write.insert("key1".to_string(), "value1".to_string());
        cache_write.insert("key2".to_string(), "value2".to_string());
    }
    
    let mut handles = vec![];
    
    // 多个读线程
    for i in 0..3 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let cache_read = cache.read().unwrap();
            let value = cache_read.get("key1");
            println!("  读线程 {}: {:?}", i, value);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n说明: RwLock 允许多个读操作并发执行");
}

fn producer_consumer_example() {
    let queue = Arc::new(Mutex::new(Vec::new()));
    let producer_queue = Arc::clone(&queue);
    let consumer_queue = Arc::clone(&queue);
    
    // 生产者
    let producer = thread::spawn(move || {
        for i in 0..5 {
            let mut q = producer_queue.lock().unwrap();
            q.push(i);
            println!("  生产者: 添加 {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    // 消费者
    let consumer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(10));
            let mut q = consumer_queue.lock().unwrap();
            if let Some(item) = q.pop() {
                println!("  消费者: 取出 {}", item);
            }
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    println!("\n说明: Mutex 确保队列的线程安全访问");
}

// ============================================
// 总结
// ============================================

/*
=== 核心总结 ===

1. Arc 和 Mutex 的本质区别：

   Arc (所有权管理):
   - 解决：如何让多个线程共享所有权
   - 类比：多人共用一本书的"借书卡"
   - 不提供：数据修改能力

   Mutex (访问控制):
   - 解决：如何安全地修改共享数据
   - 类比：书本的"阅览室"，一次只能一人进入
   - 提供：内部可变性

2. 常见组合模式：

   Arc<T>           -> 多线程共享不可变数据
   Mutex<T>         -> 单线程/单所有者可变数据
   Arc<Mutex<T>>    -> 多线程共享可变数据（最常见）
   Arc<RwLock<T>>   -> 多线程共享，读多写少

3. 选择指南：

   问题                          | 解决方案
   ------------------------------|-------------------
   多线程读取不可变数据          | Arc<T>
   单线程修改数据                | Mutex<T>
   多线程修改数据                | Arc<Mutex<T>>
   多线程读多写少                | Arc<RwLock<T>>
   高频读写，简单类型            | Arc<AtomicXxx>

4. 性能考虑：

   - Arc: 轻量级，只有引用计数开销
   - Mutex: 锁开销，可能阻塞线程
   - RwLock: 比 Mutex 稍重，但读操作可并发
   - Atomic: 最轻量，但仅支持简单类型

5. 最佳实践：

   ✅ 尽快释放锁
   ✅ 避免嵌套锁
   ✅ 锁内避免耗时操作
   ✅ 优先使用作用域控制锁的生命周期
   ✅ 读多写少使用 RwLock
   ✅ 简单计数器使用 Atomic

6. 记忆口诀：

   "Arc 管所有，Mutex 保安全，
    组合使用最常见，读写分离用 RwLock"

运行示例：
  cargo run --bin arc_vs_mutex
*/
