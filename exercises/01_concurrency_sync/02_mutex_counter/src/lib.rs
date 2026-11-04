//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: Create Arc<Mutex<usize>> with initial value 0
    // TODO: Spawn n_threads threads
    // TODO: In each thread, lock() and increment count_per_thread times
    // TODO: Join all threads, return final value
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
        for _ in 0..n_threads {
        // 2. 每个线程进门前，先克隆一个 Arc 指针
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..count_per_thread {
                    // 3. 加锁！拿到钥匙才能修改数据
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                    // 离开大括号后，锁会自动释放
                }
            });
            handles.push(handle);
        }

    // 4. 等待所有线程干活完
        for handle in handles {
            handle.join().unwrap();
        }

    // 5. 拿回最终结果
        let result = counter.lock().unwrap();
        *result // 解引用，把 usize 拿出来
}


/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: Create Arc<Mutex<Vec<usize>>>
    // TODO: Each thread pushes its own id
    // TODO: After joining all threads, sort the result and return
    let vec = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..n_threads {
        let vec_clone = Arc::clone(&vec);
        let handle = thread::spawn(move || {
            // 加锁，把当前线程的序号推入数组
            let mut v = vec_clone.lock().unwrap();
            v.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 拿到锁，取出里面的 Vec 并排序
    let mut result = vec.lock().unwrap();
    result.sort();
    result.clone() // 因为返回的不能是 MutexGuard，必须克隆一份数据还回去
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
