# Rust 异步编程深入详解与实战

## 目录
- [1. 异步编程基础](#1-异步编程基础)
- [2. Future 与 Poll 机制](#2-future-与-poll-机制)
- [3. async/await 语法详解](#3-asyncawait-语法详解)
- [4. Tokio 运行时深入](#4-tokio-运行时深入)
- [5. 异步 IO 详解](#5-异步-io-详解)
- [6. 异步并发模式](#6-异步并发模式)
- [7. 异步生态系统](#7-异步生态系统)
- [8. 性能优化](#8-性能优化)
- [9. 实战案例](#9-实战案例)

---

## 1. 异步编程基础

### 1.1 什么是异步编程

```rust
// ❌ 同步代码（阻塞）
fn fetch_data() -> String {
    std::thread::sleep(std::time::Duration::from_secs(1));
    "数据".to_string()
}

fn main() {
    let data1 = fetch_data(); // 阻塞 1 秒
    let data2 = fetch_data(); // 再阻塞 1 秒
    println!("总耗时：2秒");
}

// ✅ 异步代码（非阻塞）
async fn fetch_data_async() -> String {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "数据".to_string()
}

#[tokio::main]
async fn main() {
    let (data1, data2) = tokio::join!(
        fetch_data_async(),
        fetch_data_async()
    );
    println!("总耗时：1秒（并发执行）");
}
```

### 1.2 异步 vs 多线程

```rust
use tokio;
use std::thread;
use std::time::{Duration, Instant};

// 多线程方式
fn multi_thread_approach() {
    let start = Instant::now();
    
    let mut handles = vec![];
    for i in 0..1000 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            i * 2
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("多线程耗时: {:?}", start.elapsed());
    // 可能耗时很长，每个线程占用 ~2MB 栈空间
}

// 异步方式
#[tokio::main]
async fn async_approach() {
    let start = Instant::now();
    
    let mut tasks = vec![];
    for i in 0..1000 {
        let task = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            i * 2
        });
        tasks.push(task);
    }
    
    for task in tasks {
        task.await.unwrap();
    }
    
    println!("异步耗时: {:?}", start.elapsed());
    // 约 100ms，每个任务只占用很小的内存
}
```

### 1.3 异步编程的优势

```rust
/// 优势演示：高并发场景
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    println!("服务器启动在 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        
        // 为每个连接生成一个轻量级任务
        tokio::spawn(async move {
            handle_connection(socket, addr).await;
        });
        
        // 可以轻松处理数万个并发连接
        // 如果用线程，1万个连接 = 20GB 内存
        // 用异步任务，1万个连接 = ~100MB 内存
    }
}

async fn handle_connection(socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    println!("新连接来自: {}", addr);
    // 处理连接...
}
```

---

## 2. Future 与 Poll 机制

### 2.1 Future Trait 详解

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Future trait 定义
pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

/// Poll 枚举
pub enum Poll<T> {
    Ready(T),    // 任务完成，返回结果
    Pending,     // 任务未完成，需要等待
}
```

### 2.2 手动实现 Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// 延迟 Future
pub struct Delay {
    when: Instant,
}

impl Delay {
    pub fn new(duration: Duration) -> Self {
        Self {
            when: Instant::now() + duration,
        }
    }
}

impl Future for Delay {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 检查是否到时间
        if Instant::now() >= self.when {
            println!("延迟完成！");
            Poll::Ready(())
        } else {
            // 还没到时间，需要唤醒
            println!("还在等待...");
            
            // 通知执行器稍后再次 poll
            cx.waker().wake_by_ref();
            
            Poll::Pending
        }
    }
}

// 使用
async fn use_delay() {
    println!("开始延迟");
    Delay::new(Duration::from_secs(2)).await;
    println!("延迟结束");
}
```

### 2.3 复杂 Future 实现

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 计数器 Future
pub struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Self { count: 0, max }
    }
}

impl Future for Counter {
    type Output = u32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        
        println!("计数: {}/{}", self.count, self.max);
        
        if self.count >= self.max {
            Poll::Ready(self.count)
        } else {
            // 继续计数
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let result = Counter::new(5).await;
    println!("最终计数: {}", result);
}
```

### 2.4 组合 Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Join Future - 等待两个 Future 都完成
pub struct Join<F1, F2> {
    future1: Option<F1>,
    future2: Option<F2>,
}

impl<F1, F2> Join<F1, F2> {
    pub fn new(future1: F1, future2: F2) -> Self {
        Self {
            future1: Some(future1),
            future2: Some(future2),
        }
    }
}

impl<F1, F2> Future for Join<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    type Output = (F1::Output, F2::Output);
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut both_ready = true;
        
        // Poll 第一个 future
        if let Some(ref mut f1) = self.future1 {
            if let Poll::Ready(output1) = Pin::new(f1).poll(cx) {
                // 第一个完成，保存结果
                self.future1 = None;
                // 但我们需要保存 output1...
            } else {
                both_ready = false;
            }
        }
        
        // Poll 第二个 future
        if let Some(ref mut f2) = self.future2 {
            if let Poll::Ready(output2) = Pin::new(f2).poll(cx) {
                self.future2 = None;
            } else {
                both_ready = false;
            }
        }
        
        if both_ready {
            // 两个都完成了
            // Poll::Ready((output1, output2))
            Poll::Pending // 简化示例
        } else {
            Poll::Pending
        }
    }
}
```

---

## 3. async/await 语法详解

### 3.1 async 函数

```rust
// async fn 返回一个 impl Future
async fn simple_async() -> i32 {
    42
}

// 等价于
fn simple_async_desugared() -> impl Future<Output = i32> {
    async {
        42
    }
}

// async 块
fn example() {
    let future = async {
        let x = 10;
        let y = 20;
        x + y
    };
    
    // future 是一个 impl Future<Output = i32>
}

// async 闭包（nightly）
// let closure = async || {
//     println!("异步闭包");
// };
```

### 3.2 await 关键字

```rust
async fn fetch_user(id: u64) -> User {
    // 模拟数据库查询
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    User { id, name: "Alice".to_string() }
}

async fn fetch_posts(user_id: u64) -> Vec<Post> {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    vec![Post { id: 1, user_id, title: "Hello".to_string() }]
}

// 串行执行
async fn serial_execution() {
    let user = fetch_user(1).await;           // 等待 100ms
    let posts = fetch_posts(user.id).await;   // 再等待 100ms
    // 总耗时：200ms
}

// 并行执行
async fn parallel_execution() {
    let user_future = fetch_user(1);
    let posts_future = fetch_posts(1);
    
    // 同时等待两个
    let (user, posts) = tokio::join!(user_future, posts_future);
    // 总耗时：100ms
}
```

### 3.3 .await 的工作原理

```rust
// 这段代码
async fn example() {
    let result = some_async_fn().await;
    println!("结果: {}", result);
}

// 大致等价于
fn example_desugared() -> impl Future<Output = ()> {
    async move {
        match some_async_fn().poll() {
            Poll::Ready(result) => {
                println!("结果: {}", result);
            }
            Poll::Pending => {
                // 让出控制权，等待唤醒
                // 唤醒后继续从这里执行
            }
        }
    }
}
```

### 3.4 async 中的生命周期

```rust
// ❌ 错误：借用跨越 .await
async fn wrong_borrow() {
    let mut data = vec![1, 2, 3];
    let slice = &data[..];
    
    some_async_fn().await;  // 错误：slice 跨越 await
    
    println!("{:?}", slice);
}

// ✅ 正确：在 await 之前结束借用
async fn correct_borrow() {
    let mut data = vec![1, 2, 3];
    
    {
        let slice = &data[..];
        println!("{:?}", slice);
    }  // 借用在这里结束
    
    some_async_fn().await;  // 正确
}

// ✅ 正确：使用 Arc 共享所有权
use std::sync::Arc;

async fn with_arc() {
    let data = Arc::new(vec![1, 2, 3]);
    let data_clone = Arc::clone(&data);
    
    tokio::spawn(async move {
        println!("{:?}", data_clone);
    });
    
    some_async_fn().await;
    println!("{:?}", data);
}
```

---

## 4. Tokio 运行时深入

### 4.1 运行时配置

```rust
use tokio::runtime::{Builder, Runtime};

fn main() {
    // 方式 1：使用宏（推荐）
    #[tokio::main]
    async fn main_async() {
        println!("使用默认运行时");
    }
    
    // 方式 2：手动构建运行时
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("手动运行时");
    });
    
    // 方式 3：自定义配置
    let rt = Builder::new_multi_thread()
        .worker_threads(4)                    // 工作线程数
        .thread_name("my-worker")             // 线程名称
        .thread_stack_size(3 * 1024 * 1024)  // 栈大小
        .enable_all()                         // 启用所有功能
        .build()
        .unwrap();
    
    rt.block_on(async {
        println!("自定义运行时");
    });
    
    // 方式 4：单线程运行时（适合简单场景）
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        println!("单线程运行时");
    });
}
```

### 4.2 任务生成和管理

```rust
use tokio;

#[tokio::main]
async fn main() {
    // 生成任务
    let handle = tokio::spawn(async {
        println!("任务执行中...");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        42
    });
    
    // 等待任务完成
    let result = handle.await.unwrap();
    println!("任务结果: {}", result);
    
    // 生成多个任务
    let mut handles = vec![];
    
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100 * i)).await;
            i * 2
        });
        handles.push(handle);
    }
    
    // 等待所有任务
    for handle in handles {
        let result = handle.await.unwrap();
        println!("结果: {}", result);
    }
}
```

### 4.3 任务取消

```rust
use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        loop {
            println!("工作中...");
            sleep(Duration::from_secs(1)).await;
        }
    });
    
    // 等待 3 秒
    sleep(Duration::from_secs(3)).await;
    
    // 取消任务
    handle.abort();
    
    // 检查任务是否被取消
    match handle.await {
        Ok(_) => println!("任务完成"),
        Err(e) if e.is_cancelled() => println!("任务被取消"),
        Err(e) => println!("任务错误: {}", e),
    }
}
```

### 4.4 select! 宏

```rust
use tokio::{select, time::{sleep, Duration}};

async fn task1() -> i32 {
    sleep(Duration::from_secs(1)).await;
    1
}

async fn task2() -> i32 {
    sleep(Duration::from_secs(2)).await;
    2
}

#[tokio::main]
async fn main() {
    // 等待第一个完成的任务
    select! {
        result = task1() => {
            println!("task1 完成: {}", result);
        }
        result = task2() => {
            println!("task2 完成: {}", result);
        }
    }
    
    // 带超时
    select! {
        _ = sleep(Duration::from_secs(5)) => {
            println!("超时");
        }
        result = long_running_task() => {
            println!("任务完成: {}", result);
        }
    }
    
    // 监听多个事件
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut counter = 0;
    
    loop {
        select! {
            _ = interval.tick() => {
                counter += 1;
                println!("计数: {}", counter);
                if counter >= 5 {
                    break;
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("收到中断信号");
                break;
            }
        }
    }
}

async fn long_running_task() -> i32 {
    sleep(Duration::from_secs(3)).await;
    42
}
```

### 4.5 JoinSet - 管理多个任务

```rust
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    
    // 添加任务
    for i in 0..10 {
        set.spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100 * i)).await;
            i * 2
        });
    }
    
    // 等待所有任务完成
    while let Some(res) = set.join_next().await {
        let output = res.unwrap();
        println!("任务完成: {}", output);
    }
    
    // 或者使用 join_all
    let results: Vec<_> = set.join_all().await;
}
```

---

## 5. 异步 IO 详解

### 5.1 异步 TCP 服务器

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器启动: 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("新连接: {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("处理连接错误: {}", e);
            }
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    
    loop {
        // 读取数据
        let n = socket.read(&mut buffer).await?;
        
        if n == 0 {
            // 连接关闭
            return Ok(());
        }
        
        // 回显数据
        socket.write_all(&buffer[..n]).await?;
    }
}
```

### 5.2 异步 TCP 客户端

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 连接服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("已连接到服务器");
    
    // 发送数据
    stream.write_all(b"Hello, Server!").await?;
    
    // 读取响应
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    
    println!("收到响应: {}", String::from_utf8_lossy(&buffer[..n]));
    
    Ok(())
}
```

### 5.3 异步文件操作

```rust
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 异步读取文件
    let mut file = File::open("test.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("文件内容: {}", contents);
    
    // 异步写入文件
    let mut file = File::create("output.txt").await?;
    file.write_all(b"Hello, Tokio!").await?;
    
    // 使用缓冲读取
    let file = File::open("large_file.txt").await?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    
    use tokio::io::AsyncBufReadExt;
    while reader.read_line(&mut line).await? > 0 {
        println!("行: {}", line.trim());
        line.clear();
    }
    
    // 使用缓冲写入
    let file = File::create("buffered_output.txt").await?;
    let mut writer = BufWriter::new(file);
    
    for i in 0..1000 {
        writer.write_all(format!("Line {}\n", i).as_bytes()).await?;
    }
    
    writer.flush().await?;
    
    Ok(())
}
```

### 5.4 异步 HTTP 请求

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET 请求
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("状态: {}", response.status());
    
    let body = response.text().await?;
    println!("响应体: {}", body);
    
    // POST 请求
    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .json(&serde_json::json!({
            "key": "value"
        }))
        .send()
        .await?;
    
    println!("POST 响应: {}", response.status());
    
    // 并发请求
    let urls = vec![
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2",
        "https://httpbin.org/delay/3",
    ];
    
    let futures: Vec<_> = urls
        .iter()
        .map(|url| reqwest::get(*url))
        .collect();
    
    let results = futures::future::join_all(futures).await;
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(response) => println!("请求 {} 完成: {}", i, response.status()),
            Err(e) => println!("请求 {} 失败: {}", i, e),
        }
    }
    
    Ok(())
}
```

---

## 6. 异步并发模式

### 6.1 任务池模式

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 限制并发数为 5
    let semaphore = Arc::new(Semaphore::new(5));
    let mut handles = vec![];
    
    for i in 0..20 {
        let permit = semaphore.clone();
        let handle = tokio::spawn(async move {
            // 获取许可
            let _guard = permit.acquire().await.unwrap();
            
            println!("任务 {} 开始执行", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("任务 {} 完成", i);
            
            // guard 被 drop 时自动释放许可
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### 6.2 生产者-消费者模式

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    
    // 生产者
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
            println!("生产: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
    
    // 消费者
    while let Some(value) = rx.recv().await {
        println!("消费: {}", value);
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}
```

### 6.3 工作窃取模式

```rust
use tokio::sync::mpsc;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    // 创建多个工作者
    for worker_id in 0..4 {
        let rx = Arc::new(tokio::sync::Mutex::new(rx.clone()));
        
        tokio::spawn(async move {
            loop {
                let mut rx = rx.lock().await;
                if let Some(task) = rx.recv().await {
                    drop(rx);  // 释放锁
                    
                    println!("工作者 {} 处理任务: {}", worker_id, task);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                } else {
                    break;
                }
            }
        });
    }
    
    // 分发任务
    for i in 0..20 {
        tx.send(i).unwrap();
    }
    
    drop(tx);  // 关闭通道
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
```

---

继续下一部分...
