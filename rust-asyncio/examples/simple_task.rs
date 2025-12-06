use rust_asyncio::Runtime;

fn main() {
    let mut runtime = Runtime::new();
    
    runtime.block_on(async {
        println!("Hello from async task!");
        
        // 模拟异步工作
        for i in 0..5 {
            println!("Task iteration: {}", i);
        }
        
        println!("Task completed!");
    });
}
