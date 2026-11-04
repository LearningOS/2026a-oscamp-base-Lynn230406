//! # Channel Communication
//!
//! In this exercise, you will use `std::sync::mpsc` channels to pass messages between threads.
//!
//! ## Concepts
//! - `mpsc::channel()` creates a multiple producer, single consumer channel
//! - `Sender::send()` sends a message
//! - `Receiver::recv()` receives a message
//! - Multiple producers can be created via `Sender::clone()`

use std::sync::mpsc;
use std::thread;

/// Create a producer thread that sends each element from items into the channel.
/// The main thread receives all messages and returns them.
pub fn simple_send_recv(items: Vec<String>) -> Vec<String> {
    // TODO: Create channel
    // TODO: Spawn thread to send each element in items
    // TODO: In main thread, receive all messages and collect into Vec
    // Hint: When all Senders are dropped, recv() returns Err
    let (tx, rx) = mpsc::channel();

    // 2. 开启生产者线程
    thread::spawn(move || {
        for item in items {
            tx.send(item).unwrap();
        }
        // 线程结束，tx 被自动销毁 (drop)
    });

    // 3. 主线程作为消费者，收集所有消息
    let mut result = Vec::new();
    // for 循环会自动调用 rx.recv()，直到所有 tx 都被销毁才会退出
    for received in rx {
        result.push(received);
    }

    result
}

/// Create `n_producers` producer threads, each sending a message in format `"msg from {id}"`.
/// Collect all messages, sort them lexicographically, and return.
///
/// Hint: Use `tx.clone()` to create multiple senders. Note that the original tx must also be dropped.
pub fn multi_producer(n_producers: usize) -> Vec<String> {
    // TODO: Create channel
    // TODO: Clone a sender for each producer
    // TODO: Remember to drop the original sender, otherwise receiver won't finish
    // TODO: Collect all messages and sort
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for id in 0..n_producers {
        // 1. 克隆发送端，每个线程发一个
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let msg = format!("msg from {}", id);
            tx_clone.send(msg).unwrap();
        });
        handles.push(handle);
    }

    // 2. 极其关键：主线程必须把自己的原始 tx 丢掉！
    drop(tx);

    // 3. 收集消息
    let mut result = Vec::new();
    for received in rx {
        result.push(received);
    }

    // 4. 等待所有子线程跑完（可选，但其实 rx 循环结束说明它们都发完并 drop 了）
    for handle in handles {
        handle.join().unwrap();
    }

    // 5. 排序后返回
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_send_recv() {
        let items = vec!["hello".into(), "world".into(), "rust".into()];
        let result = simple_send_recv(items.clone());
        assert_eq!(result, items);
    }

    #[test]
    fn test_simple_empty() {
        let result = simple_send_recv(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(3);
        assert_eq!(
            result,
            vec![
                "msg from 0".to_string(),
                "msg from 1".to_string(),
                "msg from 2".to_string(),
            ]
        );
    }

    #[test]
    fn test_multi_producer_single() {
        let result = multi_producer(1);
        assert_eq!(result, vec!["msg from 0".to_string()]);
    }
}
