//! # Rust Asyncio
//! 
//! 类似 Python asyncio 的异步运行时实现
//! 
//! ## 特性
//! 
//! - 简单的事件循环
//! - 任务调度
//! - 异步 IO 支持
//! - 定时器
//! - 类似 Python asyncio 的 API
//! 
//! ## 示例
//! 
//! ```rust,no_run
//! use rust_asyncio::Runtime;
//! 
//! let mut runtime = Runtime::new();
//! runtime.block_on(async {
//!     println!("Hello, Asyncio!");
//! });
//! ```

pub mod runtime;
pub mod task;
pub mod timer;
pub mod io;
pub mod channel;

pub use runtime::Runtime;
pub use task::Task;

// Python asyncio 风格的 API
pub mod asyncio {
    pub use crate::runtime::Runtime as EventLoop;
    
    /// 创建并运行事件循环
    pub fn run<F>(future: F) -> F::Output
    where
        F: std::future::Future,
    {
        let mut runtime = crate::Runtime::new();
        runtime.block_on(future)
    }
    
    /// 创建一个任务
    pub fn create_task<F>(future: F) -> crate::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        crate::task::spawn(future)
    }
    
    /// 休眠指定时间
    pub async fn sleep(duration: std::time::Duration) {
        crate::timer::sleep(duration).await;
    }
    
    /// 等待多个 future 完成
    pub async fn gather<F>(futures: Vec<F>) -> Vec<F::Output>
    where
        F: std::future::Future,
    {
        futures::future::join_all(futures).await
    }
}
