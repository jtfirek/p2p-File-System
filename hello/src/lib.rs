use std::{
    io,
    sync::{mpsc::{self, Receiver}, Arc, Mutex},
    thread
};
use log::{warn};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
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

        ThreadPool { workers, sender }
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

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
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
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        let handle = handle?;
        Ok(Worker { id, handle })
    }
}
