use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread_handle: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread_handle = thread::spawn(move || { loop {
                let job_result = receiver.lock().unwrap().recv();

                match job_result {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
        
                        job();
                    }
                    Err(_) => {
                        println!("SHUTTING DOWN WORKER {id}");
                        break;
                    }
                }

            } 
        }
    );
    return Worker {
        id,
        thread_handle
    }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread_handle.join().unwrap();
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mutex_receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&mutex_receiver)));
        }

        return ThreadPool {
            workers, sender: Some(sender)
        }
    }

    pub fn execute<F>(&self, function: F)
    where 
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(function);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}