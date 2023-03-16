use std::{
    io,
    sync::{mpsc::{self}, Arc, Mutex},
    thread
};
use log::{warn};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver)); 

        let mut workers = Vec::with_capacity(size);


        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&receiver));

            match worker {
                Ok(worker) => (workers.push(worker)),
                Err(e) => (warn!("Failed to create worker {i}: {e}")),
            };
        }

        ThreadPool { workers, sender: Some(sender) }
    }
    /// Submits a closure to be executed by an idle thread in the thread pool.
    ///
    /// # Parameters
    ///
    /// * `f` - The closure to be executed by a thread in the pool. 
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); // on heap

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// deconstructor closes the channel and makes sure the threads are done running
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // closing the channel
            drop(self.sender.take());

            // if the thread option is already none this means that the thread has finished running
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>
}

impl Worker {
    /// Creates a new Worker with the given `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the new Worker.
    /// 
    ///  * `receiver` - thread save shared pointer to the mutex that holds the receiver
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> io::Result<Worker> {
        let builder = thread::Builder::new();

        let handle = builder.spawn(move || loop { // loop infinitely recieving jobs
            let result = receiver.lock().unwrap().recv();
            
            match result {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        let handle = handle?;
        Ok(Worker { id, handle: Some(handle) })
    }
}
