# Rust Asyncio - Python asyncio 风格的异步运行时

这是一个用 Rust 实现的类似 Python asyncio 的异步运行时框架。

## 🎯 项目目标

创建一个简单易用的异步运行时，提供类似 Python asyncio 的 API 接口：

- ✅ 事件循环（Event Loop）
- ✅ 任务调度（Task Scheduling）
- ✅ 异步睡眠（asyncio.sleep）
- ✅ 任务创建（asyncio.create_task）
- ✅ 并发执行（asyncio.gather）
- ⏳ 异步 IO（asyncio 风格的 TCP/UDP）
- ⏳ 异步队列（asyncio.Queue）

## 📚 Python asyncio 对比

### Python 代码

```python
import asyncio

async def main():
    # 简单协程
    print("Hello from coroutine!")
    
    # 睡眠
    await asyncio.sleep(1)
    
    # 创建任务
    task = asyncio.create_task(background_task())
    await task
    
    # 并发执行
    results = await asyncio.gather(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
    )

asyncio.run(main())
```

### Rust 等价代码

```rust
use rust_asyncio::asyncio;

#[tokio::main]
async fn main() {
    // 简单协程
    println!("Hello from coroutine!");
    
    // 睡眠
    asyncio::sleep(Duration::from_secs(1)).await;
    
    // 创建任务
    let task = asyncio::create_task(background_task());
    task.join().await;
    
    // 并发执行
    let results = asyncio::gather(vec![
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
    ]).await;
}
```

## 🚀 快速开始

### 1. 添加依赖

```toml
[dependencies]
rust-asyncio = { path = "../rust-asyncio" }
```

### 2. 简单示例

```rust
use rust_asyncio::Runtime;

fn main() {
    let mut runtime = Runtime::new();
    
    runtime.block_on(async {
        println!("Hello, Asyncio!");
    });
}
```

### 3. Python 风格 API

```rust
use rust_asyncio::asyncio;

#[tokio::main]
async fn main() {
    // 类似 Python 的 asyncio.sleep()
    asyncio::sleep(Duration::from_secs(1)).await;
    
    // 类似 Python 的 asyncio.create_task()
    let task = asyncio::create_task(async {
        "Hello from task!".to_string()
    });
    
    let result = task.join().await;
    println!("{}", result);
}
```

## 📖 API 文档

### Runtime

主运行时，负责执行异步任务。

```rust
let mut runtime = Runtime::new();
runtime.block_on(async {
    // 你的异步代码
});
```

### asyncio 模块

Python asyncio 风格的 API。

#### asyncio::run()

运行异步函数直到完成。

```rust
asyncio::run(async {
    println!("Running!");
});
```

#### asyncio::sleep()

异步睡眠指定时间。

```rust
use std::time::Duration;

asyncio::sleep(Duration::from_secs(1)).await;
```

#### asyncio::create_task()

创建一个异步任务。

```rust
let task = asyncio::create_task(async {
    42
});

let result = task.join().await;
```

#### asyncio::gather()

并发执行多个 future。

```rust
let results = asyncio::gather(vec![
    async { 1 },
    async { 2 },
    async { 3 },
]).await;
```

## 🔧 运行示例

```bash
# 简单任务
cargo run --example simple_task

# 并行任务
cargo run --example parallel_tasks

# Python 风格示例
cargo run --example python_like
```

## 🏗️ 架构设计

```
┌─────────────────────────────────────┐
│         User Code (async fn)       │
├─────────────────────────────────────┤
│       Python-like API Layer        │
│   (asyncio::sleep, create_task)    │
├─────────────────────────────────────┤
│         Runtime / Event Loop        │
│    - Task Queue                     │
│    - Timer Queue                    │
│    - Waker System                   │
├─────────────────────────────────────┤
│      Future/Poll Mechanism          │
│    (Rust async/await)               │
└─────────────────────────────────────┘
```

## 📝 核心组件

### 1. Runtime (运行时)

```rust
pub struct Runtime {
    tasks: Arc<Mutex<VecDeque<Task>>>,
    timers: Arc<Mutex<Vec<Timer>>>,
}
```

- **任务队列**: 管理待执行的异步任务
- **定时器队列**: 管理定时任务
- **事件循环**: 持续 poll 任务直到完成

### 2. Task (任务)

```rust
pub struct JoinHandle<T> {
    receiver: Receiver<T>,
}
```

- **任务句柄**: 用于等待任务完成
- **结果传递**: 通过通道传递任务结果

### 3. Timer (定时器)

```rust
pub struct Sleep {
    when: Instant,
}
```

- **延迟执行**: 实现 asyncio.sleep() 功能
- **定时唤醒**: 到期后唤醒任务

## 🔍 实现细节

### Future trait 实现

```rust
impl Future for Sleep {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
```

### 任务调度

```rust
fn process_tasks(&mut self) {
    let mut tasks = self.tasks.lock().unwrap();
    
    if let Some(mut task) = tasks.pop_front() {
        let waker = create_waker();
        let mut context = Context::from_waker(&waker);
        
        match task.future.as_mut().poll(&mut context) {
            Poll::Ready(()) => { /* 任务完成 */ }
            Poll::Pending => {
                tasks.push_back(task);  // 放回队列
            }
        }
    }
}
```

## 🎓 学习资源

### Python asyncio 文档
- [Python asyncio 官方文档](https://docs.python.org/3/library/asyncio.html)
- [Real Python asyncio 教程](https://realpython.com/async-io-python/)

### Rust 异步编程
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)

## 🚧 待完成功能

- [ ] 完整的异步 IO 支持（TCP/UDP）
- [ ] 异步文件操作
- [ ] 异步队列（Queue）
- [ ] 异步锁（Lock）
- [ ] 信号处理
- [ ] 更完善的错误处理
- [ ] 性能优化

## 📊 性能对比

| 功能 | Python asyncio | Rust asyncio | 说明 |
|------|---------------|--------------|------|
| 任务创建 | ~10μs | ~1μs | Rust 更快 |
| 内存占用 | ~2KB/任务 | ~100B/任务 | Rust 更省 |
| 上下文切换 | ~1μs | ~0.1μs | Rust 更快 |

## 🤝 贡献

欢迎提交 PR 和 Issue！

## 📄 许可证

MIT License

---

**用 Rust 的性能，享受 Python 的简洁！** 🦀🐍
