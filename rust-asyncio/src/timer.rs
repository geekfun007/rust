use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// 异步睡眠
/// 
/// 类似 Python 的 asyncio.sleep()
pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        when: Instant::now() + duration,
    }
}

/// Sleep Future
pub struct Sleep {
    when: Instant,
}

impl Future for Sleep {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // 注册定时器唤醒
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 定时器
pub struct Timer {
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
    
    pub async fn wait(&self) {
        sleep(self.duration).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_sleep() {
        let start = Instant::now();
        sleep(Duration::from_millis(100)).await;
        let elapsed = start.elapsed();
        
        assert!(elapsed >= Duration::from_millis(100));
    }
}
