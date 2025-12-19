// 多线程并发

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};

/// # 创建线程
pub fn creating_threads_demo() {
    println!("\n=== 创建线程 ===");
    
    // spawn 创建新线程
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  子线程: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // 主线程继续执行
    for i in 1..=3 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // 等待子线程完成
    handle.join().unwrap();
    println!("子线程已完成");
}

/// # 线程与move闭包
pub fn move_closures_demo() {
    println!("\n=== move 闭包 ===");
    
    let v = vec![1, 2, 3];
    
    // 使用 move 转移所有权到线程
    let handle = thread::spawn(move || {
        println!("  线程中的向量: {:?}", v);
    });
    
    // v 已被移动，不能再使用
    // println!("{:?}", v);  // 错误！
    
    handle.join().unwrap();
}

/// # 线程间通信：使用Channel
pub fn thread_communication_demo() {
    println!("\n=== 线程间通信 ===");
    
    use std::sync::mpsc;
    
    // 创建通道
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi from thread");
        tx.send(val).unwrap();
    });
    
    // 接收消息
    let received = rx.recv().unwrap();
    println!("收到消息: {}", received);
    
    // 发送多个消息
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    println!("\n接收多个消息:");
    for received in rx {
        println!("  {}", received);
    }
}

/// # 共享状态：Mutex
pub fn mutex_demo() {
    println!("\n=== Mutex 互斥锁 ===");
    
    // 单线程 Mutex
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // 锁在这里自动释放
    
    println!("m = {:?}", m);
    
    // 多线程共享 Mutex
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
    
    println!("计数结果: {}", *counter.lock().unwrap());
}

/// # Arc - 原子引用计数
pub fn arc_demo() {
    println!("\n=== Arc 原子引用计数 ===");
    
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("  线程 {}: {:?}", i, data);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\nArc 用途:");
    println!("  - 多线程共享不可变数据");
    println!("  - 原子引用计数，线程安全");
    println!("  - 配合 Mutex 实现共享可变数据");
}

/// # RwLock - 读写锁
pub fn rwlock_demo() {
    println!("\n=== RwLock 读写锁 ===");
    
    let lock = RwLock::new(5);
    
    // 多个读锁可以同时存在
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("读取: {} {}", *r1, *r2);
    }
    
    // 写锁是独占的
    {
        let mut w = lock.write().unwrap();
        *w += 1;
    }
    
    println!("写入后: {:?}", lock);
    
    println!("\nRwLock 特点:");
    println!("  - 允许多个读者或一个写者");
    println!("  - 适合读多写少的场景");
    println!("  - 性能优于 Mutex");
}

/// # 线程池示例
pub fn thread_pool_demo() {
    println!("\n=== 线程池示例 ===");
    
    use std::sync::mpsc;
    
    struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    
    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }
    
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                
                match job {
                    Ok(job) => {
                        println!("  Worker {} 执行任务", id);
                        job();
                    }
                    Err(_) => {
                        println!("  Worker {} 断开连接", id);
                        break;
                    }
                }
            });
            
            Worker { id, thread }
        }
    }
    
    impl ThreadPool {
        fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            
            let mut workers = Vec::with_capacity(size);
            
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            
            ThreadPool { workers, sender }
        }
        
        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }
    
    let pool = ThreadPool::new(4);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("    任务 {} 执行", i);
            thread::sleep(Duration::from_millis(100));
        });
    }
    
    thread::sleep(Duration::from_secs(1));
}

/// # 线程本地存储
pub fn thread_local_demo() {
    println!("\n=== 线程本地存储 ===");
    
    use std::cell::RefCell;
    
    thread_local! {
        static COUNTER: RefCell<u32> = RefCell::new(0);
    }
    
    COUNTER.with(|c| {
        *c.borrow_mut() += 1;
        println!("主线程计数: {}", c.borrow());
    });
    
    let handle = thread::spawn(|| {
        COUNTER.with(|c| {
            *c.borrow_mut() += 1;
            println!("  子线程计数: {}", c.borrow());
        });
    });
    
    handle.join().unwrap();
    
    COUNTER.with(|c| {
        println!("主线程计数（不变）: {}", c.borrow());
    });
}

/// # 并行迭代器
pub fn parallel_iteration_demo() {
    println!("\n=== 并行处理示例 ===");
    
    let data: Vec<i32> = (1..=100).collect();
    let chunk_size = 25;
    let mut handles = vec![];
    
    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            sum
        });
        handles.push(handle);
    }
    
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    
    println!("并行计算总和: {}", total);
}

/// # 死锁预防
pub fn deadlock_prevention_demo() {
    println!("\n=== 死锁预防 ===");
    
    println!("避免死锁的方法:");
    println!("  1. 总是以相同顺序获取锁");
    println!("  2. 使用 try_lock 避免永久阻塞");
    println!("  3. 使用超时机制");
    println!("  4. 减小锁的范围");
    println!("  5. 使用无锁数据结构");
    
    // try_lock 示例
    let m1 = Arc::new(Mutex::new(1));
    
    let m1_clone = Arc::clone(&m1);
    
    let handle = thread::spawn(move || {
        match m1_clone.try_lock() {
            Ok(guard) => {
                println!("  线程获取了锁1: {}", *guard);
            }
            Err(_) => {
                println!("  线程无法获取锁1");
            }
        }
    });
    
    let _guard1 = m1.lock().unwrap();
    println!("主线程持有锁1");
    
    handle.join().unwrap();
}

/// # 线程最佳实践
pub fn thread_best_practices_demo() {
    println!("\n=== 线程最佳实践 ===");
    
    println!("1. 选择合适的并发原语:");
    println!("   - Mutex: 独占访问");
    println!("   - RwLock: 读多写少");
    println!("   - Channel: 消息传递");
    
    println!("\n2. 避免过度使用锁:");
    println!("   - 减小临界区");
    println!("   - 考虑无锁算法");
    println!("   - 使用原子操作");
    
    println!("\n3. 错误处理:");
    println!("   - 使用 Result 而不是 panic");
    println!("   - 正确处理锁中毒");
    println!("   - 超时机制");
    
    println!("\n4. 性能考虑:");
    println!("   - 避免频繁创建线程");
    println!("   - 使用线程池");
    println!("   - 考虑 rayon 等并行库");
}

/// 运行所有线程示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        Rust 多线程并发详解         ║");
    println!("╚════════════════════════════════════╝");
    
    creating_threads_demo();
    move_closures_demo();
    thread_communication_demo();
    mutex_demo();
    arc_demo();
    rwlock_demo();
    thread_pool_demo();
    thread_local_demo();
    parallel_iteration_demo();
    deadlock_prevention_demo();
    thread_best_practices_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thread_creation() {
        let handle = thread::spawn(|| {
            42
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_mutex() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        assert_eq!(*m.lock().unwrap(), 6);
    }
}
