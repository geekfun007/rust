# Python asyncio vs Rust async 完整对比

## 目录
- [1. 基础概念对比](#1-基础概念对比)
- [2. API 对比](#2-api-对比)
- [3. 代码示例对比](#3-代码示例对比)
- [4. 性能对比](#4-性能对比)
- [5. 生态系统对比](#5-生态系统对比)
- [6. 最佳实践](#6-最佳实践)

---

## 1. 基础概念对比

### 1.1 协程 (Coroutine)

**Python:**
```python
# async def 定义协程
async def my_coroutine():
    await asyncio.sleep(1)
    return "Done"

# 协程对象
coro = my_coroutine()
```

**Rust:**
```rust
// async fn 返回 impl Future
async fn my_coroutine() -> &'static str {
    tokio::time::sleep(Duration::from_secs(1)).await;
    "Done"
}

// Future 对象
let future = my_coroutine();
```

**对比：**
- Python: 协程是第一类对象，可以传递和存储
- Rust: Future 是 trait，由编译器生成实现

### 1.2 事件循环 (Event Loop)

**Python:**
```python
import asyncio

# 获取事件循环
loop = asyncio.get_event_loop()

# 运行直到完成
loop.run_until_complete(my_coroutine())

# 或使用 asyncio.run()
asyncio.run(my_coroutine())
```

**Rust:**
```rust
use tokio;

// 使用 Tokio 运行时
#[tokio::main]
async fn main() {
    my_coroutine().await;
}

// 或手动创建
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(my_coroutine());
```

**对比：**
- Python: 显式的事件循环对象
- Rust: 隐式的运行时（通过宏或显式创建）

---

## 2. API 对比

### 2.1 睡眠 (Sleep)

| 功能 | Python | Rust |
|------|--------|------|
| **异步睡眠** | `await asyncio.sleep(1)` | `tokio::time::sleep(Duration::from_secs(1)).await` |
| **睡眠到指定时间** | `await asyncio.sleep_until(time)` | `tokio::time::sleep_until(instant).await` |

**Python:**
```python
import asyncio
from datetime import datetime, timedelta

async def sleep_example():
    # 睡眠 1 秒
    await asyncio.sleep(1)
    
    # 睡眠到指定时间
    target_time = datetime.now() + timedelta(seconds=5)
    await asyncio.sleep((target_time - datetime.now()).total_seconds())
```

**Rust:**
```rust
use tokio::time::{sleep, sleep_until, Duration, Instant};

async fn sleep_example() {
    // 睡眠 1 秒
    sleep(Duration::from_secs(1)).await;
    
    // 睡眠到指定时间
    let target_time = Instant::now() + Duration::from_secs(5);
    sleep_until(target_time).await;
}
```

### 2.2 任务创建 (Task Creation)

| 功能 | Python | Rust |
|------|--------|------|
| **创建任务** | `task = asyncio.create_task(coro())` | `handle = tokio::spawn(async {})` |
| **等待任务** | `await task` | `handle.await.unwrap()` |
| **取消任务** | `task.cancel()` | `handle.abort()` |

**Python:**
```python
import asyncio

async def background_task():
    await asyncio.sleep(2)
    return "Done"

async def main():
    # 创建任务
    task = asyncio.create_task(background_task())
    
    # 等待任务
    result = await task
    
    # 取消任务
    task.cancel()
    try:
        await task
    except asyncio.CancelledError:
        print("Task was cancelled")
```

**Rust:**
```rust
use tokio;

async fn background_task() -> &'static str {
    tokio::time::sleep(Duration::from_secs(2)).await;
    "Done"
}

#[tokio::main]
async fn main() {
    // 创建任务
    let handle = tokio::spawn(background_task());
    
    // 等待任务
    let result = handle.await.unwrap();
    
    // 取消任务
    let handle = tokio::spawn(background_task());
    handle.abort();
    
    match handle.await {
        Err(e) if e.is_cancelled() => println!("Task was cancelled"),
        _ => {}
    }
}
```

### 2.3 并发执行 (Concurrent Execution)

| 功能 | Python | Rust |
|------|--------|------|
| **并发执行** | `await asyncio.gather(*coros)` | `tokio::join!(fut1, fut2)` |
| **选择第一个完成** | `await asyncio.wait([...], return_when=FIRST_COMPLETED)` | `tokio::select! { ... }` |
| **超时** | `await asyncio.wait_for(coro, timeout)` | `tokio::time::timeout(duration, fut).await` |

**Python:**
```python
import asyncio

async def fetch(n):
    await asyncio.sleep(n)
    return n

async def main():
    # 并发执行（全部等待）
    results = await asyncio.gather(
        fetch(1),
        fetch(2),
        fetch(3)
    )
    print(results)  # [1, 2, 3]
    
    # 等待第一个完成
    done, pending = await asyncio.wait(
        [fetch(1), fetch(2)],
        return_when=asyncio.FIRST_COMPLETED
    )
    
    # 超时
    try:
        result = await asyncio.wait_for(fetch(5), timeout=2.0)
    except asyncio.TimeoutError:
        print("Timeout!")
```

**Rust:**
```rust
use tokio::time::{sleep, timeout, Duration};

async fn fetch(n: u64) -> u64 {
    sleep(Duration::from_secs(n)).await;
    n
}

#[tokio::main]
async fn main() {
    // 并发执行（全部等待）
    let (r1, r2, r3) = tokio::join!(
        fetch(1),
        fetch(2),
        fetch(3)
    );
    println!("{}, {}, {}", r1, r2, r3);
    
    // 等待第一个完成
    tokio::select! {
        r1 = fetch(1) => println!("First: {}", r1),
        r2 = fetch(2) => println!("First: {}", r2),
    }
    
    // 超时
    match timeout(Duration::from_secs(2), fetch(5)).await {
        Ok(result) => println!("Result: {}", result),
        Err(_) => println!("Timeout!"),
    }
}
```

---

## 3. 代码示例对比

### 3.1 HTTP 服务器

**Python (aiohttp):**
```python
from aiohttp import web

async def handle(request):
    name = request.match_info.get('name', "Anonymous")
    text = f"Hello, {name}"
    return web.Response(text=text)

app = web.Application()
app.add_routes([
    web.get('/', handle),
    web.get('/{name}', handle)
])

if __name__ == '__main__':
    web.run_app(app, host='127.0.0.1', port=8080)
```

**Rust (Axum):**
```rust
use axum::{
    routing::get,
    Router,
    extract::Path,
};

async fn handle(Path(name): Path<String>) -> String {
    format!("Hello, {}", name)
}

async fn handle_root() -> &'static str {
    "Hello, Anonymous"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_root))
        .route("/:name", get(handle));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
```

### 3.2 并发HTTP请求

**Python:**
```python
import asyncio
import aiohttp

async def fetch(session, url):
    async with session.get(url) as response:
        return await response.text()

async def main():
    urls = [
        'https://api.github.com/users/1',
        'https://api.github.com/users/2',
        'https://api.github.com/users/3',
    ]
    
    async with aiohttp.ClientSession() as session:
        tasks = [fetch(session, url) for url in urls]
        results = await asyncio.gather(*tasks)
        
        for result in results:
            print(len(result), "bytes")

asyncio.run(main())
```

**Rust:**
```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://api.github.com/users/1",
        "https://api.github.com/users/2",
        "https://api.github.com/users/3",
    ];
    
    let client = reqwest::Client::new();
    
    let futures: Vec<_> = urls
        .iter()
        .map(|url| client.get(*url).send())
        .collect();
    
    let results = futures::future::join_all(futures).await;
    
    for result in results {
        match result {
            Ok(response) => {
                let text = response.text().await?;
                println!("{} bytes", text.len());
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

### 3.3 WebSocket 服务器

**Python:**
```python
import asyncio
import websockets

async def echo(websocket):
    async for message in websocket:
        await websocket.send(f"Echo: {message}")

async def main():
    async with websockets.serve(echo, "localhost", 8765):
        await asyncio.Future()  # run forever

asyncio.run(main())
```

**Rust:**
```rust
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8765").await.unwrap();
    
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();
            
            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let echo = format!("Echo: {}", msg.to_text().unwrap());
                    write.send(echo.into()).await.unwrap();
                }
            }
        });
    }
}
```

---

## 4. 性能对比

### 4.1 任务创建开销

**测试代码 Python:**
```python
import asyncio
import time

async def noop():
    pass

async def benchmark_task_creation():
    start = time.time()
    tasks = [asyncio.create_task(noop()) for _ in range(10000)]
    await asyncio.gather(*tasks)
    elapsed = time.time() - start
    print(f"Python: {elapsed:.4f}s for 10000 tasks")

asyncio.run(benchmark_task_creation())
```

**测试代码 Rust:**
```rust
use tokio;
use std::time::Instant;

async fn noop() {}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..10000 {
        handles.push(tokio::spawn(noop()));
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let elapsed = start.elapsed();
    println!("Rust: {:.4?} for 10000 tasks", elapsed);
}
```

**结果对比：**
| 语言 | 10000 个任务 | 内存占用 |
|------|-------------|---------|
| Python | ~0.5s | ~50MB |
| Rust | ~0.05s | ~5MB |

### 4.2 IO 吞吐量

**Echo Server 性能：**
| 语言 | QPS (请求/秒) | 延迟 (ms) | CPU 使用率 |
|------|--------------|-----------|-----------|
| Python (asyncio) | ~10K | ~5ms | ~50% |
| Rust (Tokio) | ~100K | ~0.5ms | ~30% |

---

## 5. 生态系统对比

### 5.1 常用库对比

| 功能 | Python | Rust |
|------|--------|------|
| **HTTP客户端** | aiohttp | reqwest |
| **HTTP服务器** | aiohttp, FastAPI | Axum, Actix-web |
| **WebSocket** | websockets | tokio-tungstenite |
| **数据库** | asyncpg, aiosqlite | sqlx, tokio-postgres |
| **Redis** | aioredis | redis-rs |
| **gRPC** | grpcio | tonic |

### 5.2 学习曲线

**Python:**
- ✅ 简单易学
- ✅ 语法直观
- ❌ 性能较低
- ❌ GIL 限制

**Rust:**
- ❌ 学习曲线陡峭
- ❌ 所有权系统
- ✅ 极高性能
- ✅ 无 GC，内存安全

---

## 6. 最佳实践

### 6.1 何时使用 Python asyncio

✅ **适合场景：**
- 快速原型开发
- IO 密集型应用
- 与现有 Python 生态集成
- 团队熟悉 Python

❌ **不适合场景：**
- CPU 密集型计算
- 要求极高性能
- 需要严格类型安全

### 6.2 何时使用 Rust async

✅ **适合场景：**
- 高性能要求
- 大规模并发
- 系统级编程
- 内存安全要求

❌ **不适合场景：**
- 快速原型
- 团队不熟悉 Rust
- 开发时间紧迫

---

## 7. 迁移指南

### 7.1 Python 到 Rust 迁移

**步骤：**
1. 识别异步操作
2. 替换 asyncio API 为 Tokio API
3. 处理所有权和借用
4. 添加错误处理
5. 优化性能

**示例：**

**Python 原代码：**
```python
import asyncio
import aiohttp

async def fetch_user(user_id):
    async with aiohttp.ClientSession() as session:
        async with session.get(f'https://api.example.com/users/{user_id}') as response:
            return await response.json()

async def main():
    users = await asyncio.gather(
        fetch_user(1),
        fetch_user(2),
        fetch_user(3)
    )
    print(users)

asyncio.run(main())
```

**迁移到 Rust：**
```rust
use reqwest;
use serde_json::Value;

async fn fetch_user(user_id: u32) -> Result<Value, reqwest::Error> {
    let url = format!("https://api.example.com/users/{}", user_id);
    reqwest::get(&url)
        .await?
        .json()
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (user1, user2, user3) = tokio::try_join!(
        fetch_user(1),
        fetch_user(2),
        fetch_user(3)
    )?;
    
    println!("{:?}", vec![user1, user2, user3]);
    Ok(())
}
```

---

## 8. 总结

| 特性 | Python asyncio | Rust async |
|------|---------------|------------|
| **易用性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **性能** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **内存安全** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **生态系统** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **学习曲线** | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| **并发能力** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

**结论：**
- **Python asyncio**: 适合快速开发和 IO 密集型应用
- **Rust async**: 适合高性能和大规模并发应用

**最佳组合：**
- 用 Python 写原型和业务逻辑
- 用 Rust 实现性能关键部分
- 通过 PyO3 集成两者

---

**用正确的工具解决正确的问题！** 🐍🦀
