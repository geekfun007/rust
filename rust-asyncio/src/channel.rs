use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::mpsc::{channel, Sender, Receiver};

/// 异步通道发送端
pub struct AsyncSender<T> {
    sender: Sender<T>,
}

/// 异步通道接收端
pub struct AsyncReceiver<T> {
    receiver: Receiver<T>,
}

/// 创建异步通道
/// 
/// 类似 Python 的 asyncio.Queue
pub fn channel<T>() -> (AsyncSender<T>, AsyncReceiver<T>) {
    let (sender, receiver) = channel();
    (
        AsyncSender { sender },
        AsyncReceiver { receiver },
    )
}

impl<T> AsyncSender<T> {
    pub async fn send(&self, value: T) -> Result<(), ()> {
        self.sender.send(value).map_err(|_| ())
    }
}

impl<T> AsyncReceiver<T> {
    pub fn recv(&self) -> RecvFuture<T> {
        RecvFuture {
            receiver: &self.receiver,
        }
    }
}

pub struct RecvFuture<'a, T> {
    receiver: &'a Receiver<T>,
}

impl<'a, T> Future for RecvFuture<'a, T> {
    type Output = Option<T>;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.receiver.try_recv() {
            Ok(value) => Poll::Ready(Some(value)),
            Err(_) => Poll::Pending,
        }
    }
}
