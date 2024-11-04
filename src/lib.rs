use std::{sync::mpsc, thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct  Job;

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

        //create a new channel 
        let (sender, reciever)= mpsc::channel();
        let mut workers= Vec::with_capacity(size);
        for id in 0..size{
         workers.push(Worker::new(id, reciever));
        }

        ThreadPool{workers, sender}
    }
    //send transfer the closure from one thread to another and 'static
    //because we don't know how long it will take for the thread to execute
    pub fn execute<F>(&self, f:F) where F:FnOnce() + Send + 'static,
    {}
}

struct Worker {
    id:usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    //fn new takes an id and returns a worker instance that holds the id and a thread spawned with empty closure
    fn new(id:usize, reciever:mpsc::Receiver<Job>)->Worker{
        let thread=thread::spawn(||{
            reciever;
        });
        Worker{id, thread}
    }
}