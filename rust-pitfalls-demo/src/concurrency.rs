//! 并发编程实战示例

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/// 计数器（使用 Mutex）
pub struct Counter {
    count: Arc<Mutex<i32>>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn increment(&self) {
        let mut num = self.count.lock().unwrap();
        *num += 1;
    }
    
    pub fn get(&self) -> i32 {
        *self.count.lock().unwrap()
    }
    
    pub fn clone_counter(&self) -> Self {
        Counter {
            count: Arc::clone(&self.count),
        }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// 共享数据（使用 RwLock）
pub struct SharedData {
    data: Arc<RwLock<Vec<i32>>>,
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub fn read(&self) -> Vec<i32> {
        self.data.read().unwrap().clone()
    }
    
    pub fn write(&self, value: i32) {
        self.data.write().unwrap().push(value);
    }
    
    pub fn len(&self) -> usize {
        self.data.read().unwrap().len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.read().unwrap().is_empty()
    }
    
    pub fn clone_data(&self) -> Self {
        SharedData {
            data: Arc::clone(&self.data),
        }
    }
}

impl Default for SharedData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_counter() {
        let counter = Counter::new();
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter_clone = counter.clone_counter();
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    counter_clone.increment();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.get(), 100);
    }
    
    #[test]
    fn test_shared_data() {
        let data = SharedData::new();
        let mut handles = vec![];
        
        // 写线程
        for i in 0..5 {
            let data_clone = data.clone_data();
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(10));
                data_clone.write(i);
            });
            handles.push(handle);
        }
        
        // 读线程
        for _ in 0..5 {
            let data_clone = data.clone_data();
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(10));
                let _d = data_clone.read();
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(data.len(), 5);
    }
}
