use std::thread;

pub struct ThreadPool{
    workers: Vec<Worker>,
}

impl ThreadPool{
   //creates a new threadpool
   ///
   /// The size is the number of threads in the pool
   /// 
   /// #Panics
   /// 
   /// The `new` function will panic if the size is zero
    pub fn new(size:usize)->ThreadPool{
    assert!(size>0);
        let mut workers= Vec::with_capacity(size);
        for id in 0..size{
         workers.push(Worker::new(id));
        }

        ThreadPool{workers}
    }
    //send transfer the closure from one thread to another and 'static
    //because we don't know how long it will take for the thread to execute
    pub fn execute<F>(&self, f:F) where F:FnOnce() + Send + 'static,
    {}
}

pub struct Worker {
    id:usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    //fn new takes an id and returns a worker instance that holds the id and a thread spawned with empty closure
    pub fn new(id:usize)->Worker{
        let thread=thread::spawn(||{});
        Worker{id, thread}
    }
}