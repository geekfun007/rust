/// Python asyncio 风格的示例
/// 
/// 展示如何使用类似 Python asyncio 的 API

use rust_asyncio::asyncio;

#[tokio::main]
async fn main() {
    println!("=== Python asyncio 风格示例 ===\n");
    
    // 类似 Python: asyncio.run(main())
    println!("1. 简单的 async 函数");
    simple_coroutine().await;
    
    println!("\n2. 使用 asyncio.sleep()");
    sleep_example().await;
    
    println!("\n3. 创建任务");
    create_task_example().await;
    
    println!("\n4. 并发执行多个任务");
    gather_example().await;
}

// 类似 Python:
// async def simple_coroutine():
//     print("Hello from coroutine!")
async fn simple_coroutine() {
    println!("Hello from coroutine!");
}

// 类似 Python:
// async def sleep_example():
//     await asyncio.sleep(1)
//     print("Slept for 1 second")
async fn sleep_example() {
    use std::time::Duration;
    
    println!("开始睡眠...");
    asyncio::sleep(Duration::from_secs(1)).await;
    println!("睡眠 1 秒后醒来");
}

// 类似 Python:
// async def create_task_example():
//     task = asyncio.create_task(background_task())
//     await task
async fn create_task_example() {
    async fn background_task() -> String {
        use std::time::Duration;
        asyncio::sleep(Duration::from_millis(500)).await;
        "任务完成!".to_string()
    }
    
    println!("创建后台任务...");
    let task = asyncio::create_task(background_task());
    let result = task.join().await;
    println!("任务结果: {}", result);
}

// 类似 Python:
// async def gather_example():
//     results = await asyncio.gather(
//         fetch_data(1),
//         fetch_data(2),
//         fetch_data(3),
//     )
async fn gather_example() {
    async fn fetch_data(id: i32) -> String {
        use std::time::Duration;
        asyncio::sleep(Duration::from_millis(100 * id as u64)).await;
        format!("数据 {}", id)
    }
    
    println!("并发执行多个任务...");
    let futures = vec![
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
    ];
    
    let results = asyncio::gather(futures).await;
    println!("所有结果:");
    for (i, result) in results.iter().enumerate() {
        println!("  任务 {}: {}", i + 1, result);
    }
}
