// 异步编程 async/await

use tokio::time::{sleep, Duration};
use tokio::task;

/// # async/await 基础
pub async fn async_basics_demo() {
    println!("\n=== async/await 基础 ===");
    
    // async 函数返回 Future
    async fn say_hello() {
        println!("  Hello from async function!");
    }
    
    // await 等待 Future 完成
    say_hello().await;
    
    // async 块
    let future = async {
        println!("  Hello from async block!");
    };
    
    future.await;
    
    println!("\nasync/await 特点:");
    println!("  - 非阻塞执行");
    println!("  - 编译时转换为状态机");
    println!("  - 高效的并发");
}

/// # 异步函数
pub async fn async_functions_demo() {
    println!("\n=== 异步函数 ===");
    
    async fn fetch_data() -> String {
        sleep(Duration::from_millis(100)).await;
        String::from("data")
    }
    
    async fn process_data(data: String) -> String {
        sleep(Duration::from_millis(100)).await;
        data.to_uppercase()
    }
    
    println!("获取数据...");
    let data = fetch_data().await;
    println!("数据: {}", data);
    
    println!("处理数据...");
    let processed = process_data(data).await;
    println!("处理后: {}", processed);
}

/// # 并发执行多个异步任务
pub async fn concurrent_tasks_demo() {
    println!("\n=== 并发执行 ===");
    
    async fn task1() -> i32 {
        sleep(Duration::from_millis(100)).await;
        println!("  任务1完成");
        1
    }
    
    async fn task2() -> i32 {
        sleep(Duration::from_millis(100)).await;
        println!("  任务2完成");
        2
    }
    
    async fn task3() -> i32 {
        sleep(Duration::from_millis(100)).await;
        println!("  任务3完成");
        3
    }
    
    // join! 并发执行
    let (r1, r2, r3) = tokio::join!(task1(), task2(), task3());
    println!("结果: {} + {} + {} = {}", r1, r2, r3, r1 + r2 + r3);
}

/// # select! 宏
pub async fn select_demo() {
    println!("\n=== select! 宏 ===");
    
    async fn task1() -> &'static str {
        sleep(Duration::from_millis(100)).await;
        "task1"
    }
    
    async fn task2() -> &'static str {
        sleep(Duration::from_millis(150)).await;
        "task2"
    }
    
    // select! 等待第一个完成的任务
    tokio::select! {
        result = task1() => {
            println!("task1 先完成: {}", result);
        }
        result = task2() => {
            println!("task2 先完成: {}", result);
        }
    }
}

/// # 异步任务生成
pub async fn spawn_tasks_demo() {
    println!("\n=== 生成异步任务 ===");
    
    let handle1 = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("  异步任务1完成");
        42
    });
    
    let handle2 = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("  异步任务2完成");
        100
    });
    
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    
    println!("结果: {} + {} = {}", result1, result2, result1 + result2);
}

/// # 异步流 (Stream)
pub async fn async_stream_demo() {
    println!("\n=== 异步流 ===");
    
    use tokio_stream::{self as stream, StreamExt};
    
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    println!("处理流:");
    while let Some(value) = stream.next().await {
        println!("  值: {}", value);
    }
}

/// # 超时处理
pub async fn timeout_demo() {
    println!("\n=== 超时处理 ===");
    
    use tokio::time::timeout;
    
    async fn long_task() -> String {
        sleep(Duration::from_secs(2)).await;
        String::from("completed")
    }
    
    let result = timeout(Duration::from_millis(500), long_task()).await;
    
    match result {
        Ok(value) => println!("任务完成: {}", value),
        Err(_) => println!("任务超时！"),
    }
}

/// # 异步互斥锁
pub async fn async_mutex_demo() {
    println!("\n=== 异步互斥锁 ===");
    
    use tokio::sync::Mutex;
    use std::sync::Arc;
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = task::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
            println!("  任务 {} 增加计数", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("最终计数: {}", *counter.lock().await);
}

/// # 异步通道
pub async fn async_channel_demo() {
    println!("\n=== 异步通道 ===");
    
    use tokio::sync::mpsc;
    
    let (tx, mut rx) = mpsc::channel(10);
    
    task::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            println!("  发送: {}", i);
            sleep(Duration::from_millis(100)).await;
        }
    });
    
    while let Some(value) = rx.recv().await {
        println!("接收: {}", value);
    }
}

/// # 实战示例：并发HTTP请求
pub async fn http_requests_demo() {
    println!("\n=== 并发HTTP请求示例 ===");
    
    async fn fetch_url(id: i32) -> String {
        sleep(Duration::from_millis(100)).await;
        format!("Response from URL {}", id)
    }
    
    println!("发起3个并发请求...");
    
    let futures = vec![
        fetch_url(1),
        fetch_url(2),
        fetch_url(3),
    ];
    
    let results = futures_util::future::join_all(futures).await;
    
    for result in results {
        println!("  {}", result);
    }
}

/// # 实战示例：异步任务队列
pub async fn task_queue_demo() {
    println!("\n=== 异步任务队列 ===");
    
    use tokio::sync::mpsc;
    
    #[derive(Debug)]
    struct Task {
        id: i32,
        name: String,
    }
    
    let (tx, mut rx) = mpsc::channel::<Task>(10);
    
    // 生产者
    let producer = task::spawn(async move {
        for i in 0..5 {
            let task = Task {
                id: i,
                name: format!("Task {}", i),
            };
            tx.send(task).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
    });
    
    // 消费者
    let consumer = task::spawn(async move {
        while let Some(task) = rx.recv().await {
            println!("  处理: {:?}", task);
            sleep(Duration::from_millis(100)).await;
        }
    });
    
    producer.await.unwrap();
    consumer.await.unwrap();
}

/// # 错误处理
pub async fn error_handling_demo() {
    println!("\n=== 异步错误处理 ===");
    
    async fn may_fail(succeed: bool) -> Result<String, String> {
        sleep(Duration::from_millis(100)).await;
        
        if succeed {
            Ok(String::from("Success"))
        } else {
            Err(String::from("Failed"))
        }
    }
    
    match may_fail(true).await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match may_fail(false).await {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("错误: {}", e),
    }
}

/// # async trait
pub async fn async_trait_demo() {
    println!("\n=== async trait ===");
    
    use async_trait::async_trait;
    
    #[async_trait]
    trait AsyncProcessor {
        async fn process(&self, data: String) -> String;
    }
    
    struct UpperCaseProcessor;
    
    #[async_trait]
    impl AsyncProcessor for UpperCaseProcessor {
        async fn process(&self, data: String) -> String {
            sleep(Duration::from_millis(100)).await;
            data.to_uppercase()
        }
    }
    
    let processor = UpperCaseProcessor;
    let result = processor.process(String::from("hello")).await;
    println!("处理结果: {}", result);
}

/// # 异步最佳实践
pub async fn async_best_practices_demo() {
    println!("\n=== 异步最佳实践 ===");
    
    println!("1. 选择合适的运行时:");
    println!("   - tokio: 功能完整，适合网络应用");
    println!("   - async-std: 类似标准库API");
    println!("   - smol: 轻量级");
    
    println!("\n2. 避免阻塞:");
    println!("   - 使用 spawn_blocking 执行CPU密集任务");
    println!("   - 不要在异步代码中使用标准库的阻塞操作");
    
    println!("\n3. 错误处理:");
    println!("   - 使用 Result 类型");
    println!("   - 考虑使用 anyhow 或 thiserror");
    
    println!("\n4. 性能优化:");
    println!("   - 使用 join! 而不是串行 await");
    println!("   - 合理使用缓冲通道");
    println!("   - 注意任务的生命周期");
    
    println!("\n5. 测试:");
    println!("   - 使用 #[tokio::test]");
    println!("   - 测试超时场景");
    println!("   - 测试并发安全性");
}

/// 运行所有异步示例
pub async fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 异步编程详解 async/await ║");
    println!("╚════════════════════════════════════╝");
    
    async_basics_demo().await;
    async_functions_demo().await;
    concurrent_tasks_demo().await;
    select_demo().await;
    spawn_tasks_demo().await;
    async_stream_demo().await;
    timeout_demo().await;
    async_mutex_demo().await;
    async_channel_demo().await;
    http_requests_demo().await;
    task_queue_demo().await;
    error_handling_demo().await;
    async_trait_demo().await;
    async_best_practices_demo().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_async_function() {
        async fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        let result = add(2, 3).await;
        assert_eq!(result, 5);
    }
    
    #[tokio::test]
    async fn test_concurrent_execution() {
        async fn task() -> i32 {
            sleep(Duration::from_millis(10)).await;
            42
        }
        
        let (r1, r2) = tokio::join!(task(), task());
        assert_eq!(r1, 42);
        assert_eq!(r2, 42);
    }
}
