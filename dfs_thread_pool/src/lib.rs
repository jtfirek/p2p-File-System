//! A library for managing thread pools and executing tasks concurrently.
//!
//! This library provides a `ThreadPool` struct that can be used to create
//! and manage a pool of worker threads. Tasks can be submitted to the pool
//! for execution by the worker threads.
//!
//! # Example
//!
//! ```
//! use my_threadpool::ThreadPool;
//!
//! let pool = ThreadPool::new(4);
//!
//! pool.execute(|| {
//!     println!("Hello from a worker thread!");
//! });
//! ```

mod thread_pool;

pub use thread_pool::ThreadPool;
