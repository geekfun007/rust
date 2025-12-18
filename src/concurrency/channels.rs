// 通道 (Channels) - 消息传递

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// # 基本通道
pub fn basic_channel_demo() {
    println!("\n=== 基本通道 ===");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}

/// # 发送多个值
pub fn multiple_values_demo() {
    println!("\n=== 发送多个值 ===");
    
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
    
    for received in rx {
        println!("收到: {}", received);
    }
}

/// # 多个生产者
pub fn multiple_producers_demo() {
    println!("\n=== 多个生产者 ===");
    
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("线程1: hi"),
            String::from("线程1: from"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("线程2: more"),
            String::from("线程2: messages"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("收到: {}", received);
    }
}

/// # 同步通道
pub fn sync_channel_demo() {
    println!("\n=== 同步通道 ===");
    
    // 容量为0的同步通道
    let (tx, rx) = mpsc::sync_channel(0);
    
    thread::spawn(move || {
        println!("  发送前");
        tx.send(1).unwrap();
        println!("  发送后");
    });
    
    thread::sleep(Duration::from_millis(100));
    println!("接收前");
    let value = rx.recv().unwrap();
    println!("接收后: {}", value);
    
    // 带缓冲的同步通道
    println!("\n带缓冲的同步通道:");
    let (tx, rx) = mpsc::sync_channel(2);
    
    tx.send(1).unwrap();
    println!("  发送1（不阻塞）");
    tx.send(2).unwrap();
    println!("  发送2（不阻塞）");
    
    // 第三个发送会阻塞，直到接收
    thread::spawn(move || {
        println!("  尝试发送3（会阻塞）...");
        tx.send(3).unwrap();
        println!("  发送3成功");
    });
    
    thread::sleep(Duration::from_millis(100));
    println!("接收: {}", rx.recv().unwrap());
    println!("接收: {}", rx.recv().unwrap());
    thread::sleep(Duration::from_millis(100));
    println!("接收: {}", rx.recv().unwrap());
}

/// # try_recv 非阻塞接收
pub fn try_recv_demo() {
    println!("\n=== try_recv 非阻塞接收 ===");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx.send(String::from("delayed")).unwrap();
    });
    
    // 立即尝试接收（可能失败）
    match rx.try_recv() {
        Ok(val) => println!("立即收到: {}", val),
        Err(mpsc::TryRecvError::Empty) => println!("通道为空"),
        Err(mpsc::TryRecvError::Disconnected) => println!("通道已断开"),
    }
    
    // 等待后再尝试
    thread::sleep(Duration::from_millis(300));
    match rx.try_recv() {
        Ok(val) => println!("等待后收到: {}", val),
        Err(_) => println!("接收失败"),
    }
}

/// # recv_timeout 超时接收
pub fn recv_timeout_demo() {
    println!("\n=== recv_timeout 超时接收 ===");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send(String::from("delayed")).unwrap();
    });
    
    // 短超时（会超时）
    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(val) => println!("收到: {}", val),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("接收超时"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("通道已断开"),
    }
    
    // 长超时（会成功）
    match rx.recv_timeout(Duration::from_millis(500)) {
        Ok(val) => println!("收到: {}", val),
        Err(_) => println!("接收失败"),
    }
}

/// # 实战示例：生产者-消费者
pub fn producer_consumer_demo() {
    println!("\n=== 生产者-消费者模式 ===");
    
    let (tx, rx) = mpsc::sync_channel(5);
    
    // 生产者
    let producer = thread::spawn(move || {
        for i in 0..10 {
            println!("  生产: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // 消费者
    let consumer = thread::spawn(move || {
        for received in rx {
            println!("  消费: {}", received);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

/// # 实战示例：任务分发
pub fn task_distribution_demo() {
    println!("\n=== 任务分发 ===");
    
    println!("注意：标准库的 mpsc::Receiver 不支持多个消费者");
    println!("实际项目中可使用 crossbeam-channel 等支持 MPMC 的库");
    
    let (tx, rx) = mpsc::channel();
    let _num_workers = 3;
    
    // 创建工作线程
    let mut workers = vec![];
    {
        let _rx = rx;
        
        let worker = thread::spawn(move || {
            for task in _rx {
                println!("  Worker 处理任务: {}", task);
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        workers.push(worker);
    }
    
    // 发送任务
    for i in 0..9 {
        tx.send(format!("Task {}", i)).unwrap();
    }
    
    // 关闭通道
    drop(tx);
    
    // 等待所有工作线程完成
    for worker in workers {
        worker.join().unwrap();
    }
}

/// # 实战示例：扇入模式
pub fn fan_in_demo() {
    println!("\n=== 扇入模式 ===");
    
    let (tx, rx) = mpsc::channel();
    
    // 多个生产者
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for j in 0..3 {
                tx.send(format!("源{}-消息{}", i, j)).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
    
    drop(tx);  // 丢弃原始发送者
    
    // 单个消费者
    for received in rx {
        println!("  收到: {}", received);
    }
}

/// # 实战示例：扇出模式
pub fn fan_out_demo() {
    println!("\n=== 扇出模式 ===");
    
    println!("注意：mpsc::Receiver 不能被克隆");
    println!("扇出模式通常使用多个通道或其他并发原语实现");
    println!("例如使用 crossbeam 的多生产者多消费者通道");
    
    let (tx, rx) = mpsc::channel();
    
    // 单个生产者
    thread::spawn(move || {
        for i in 0..9 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // 单个消费者
    let mut consumers = vec![];
    {
        let _rx = rx;
        
        let consumer = thread::spawn(move || {
            for value in _rx {
                println!("  消费者收到: {}", value);
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        consumers.push(consumer);
    }
    
    for consumer in consumers {
        consumer.join().unwrap();
    }
}

/// # 实战示例：管道模式
pub fn pipeline_demo() {
    println!("\n=== 管道模式 ===");
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    // 阶段1: 生成数据
    thread::spawn(move || {
        for i in 1..=5 {
            tx1.send(i).unwrap();
        }
    });
    
    // 阶段2: 处理数据
    thread::spawn(move || {
        for value in rx1 {
            let processed = value * 2;
            println!("  阶段2: {} * 2 = {}", value, processed);
            tx2.send(processed).unwrap();
        }
    });
    
    // 阶段3: 汇总结果
    let sum: i32 = rx2.iter().sum();
    println!("最终结果: {}", sum);
}

/// # 通道选择
pub fn channel_select_demo() {
    println!("\n=== 通道选择 (使用 recv_timeout) ===");
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx1.send(String::from("通道1")).unwrap();
    });
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx2.send(String::from("通道2")).unwrap();
    });
    
    // 轮询两个通道
    loop {
        match rx1.recv_timeout(Duration::from_millis(50)) {
            Ok(val) => {
                println!("从通道1收到: {}", val);
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                match rx2.recv_timeout(Duration::from_millis(50)) {
                    Ok(val) => {
                        println!("从通道2收到: {}", val);
                        break;
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(_) => break,
                }
            }
            Err(_) => break,
        }
    }
}

/// # 通道最佳实践
pub fn channel_best_practices_demo() {
    println!("\n=== 通道最佳实践 ===");
    
    println!("1. 选择合适的通道类型:");
    println!("   - mpsc::channel: 异步，无界");
    println!("   - mpsc::sync_channel: 同步，有界");
    println!("   - 考虑使用 crossbeam 或 flume");
    
    println!("\n2. 资源管理:");
    println!("   - 及时 drop 不需要的发送者/接收者");
    println!("   - 避免通道泄漏");
    println!("   - 使用有界通道防止内存溢出");
    
    println!("\n3. 错误处理:");
    println!("   - 处理 send/recv 的错误");
    println!("   - 使用 try_recv 或 recv_timeout 避免阻塞");
    
    println!("\n4. 性能优化:");
    println!("   - 批量发送减少系统调用");
    println!("   - 选择合适的缓冲区大小");
    println!("   - 考虑使用无锁队列");
    
    println!("\n5. 设计模式:");
    println!("   - 生产者-消费者");
    println!("   - 任务分发");
    println!("   - 管道处理");
    println!("   - 扇入/扇出");
}

/// 运行所有通道示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 通道详解 (消息传递)      ║");
    println!("╚════════════════════════════════════╝");
    
    basic_channel_demo();
    multiple_values_demo();
    multiple_producers_demo();
    sync_channel_demo();
    try_recv_demo();
    recv_timeout_demo();
    producer_consumer_demo();
    task_distribution_demo();
    fan_in_demo();
    fan_out_demo();
    pipeline_demo();
    channel_select_demo();
    channel_best_practices_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel() {
        let (tx, rx) = mpsc::channel();
        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }
    
    #[test]
    fn test_multiple_senders() {
        let (tx, rx) = mpsc::channel();
        let tx2 = tx.clone();
        
        thread::spawn(move || tx.send(1).unwrap());
        thread::spawn(move || tx2.send(2).unwrap());
        
        let mut values = vec![rx.recv().unwrap(), rx.recv().unwrap()];
        values.sort();
        assert_eq!(values, vec![1, 2]);
    }
}
