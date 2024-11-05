use std::{
    sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}


type Job = Box<dyn FnOnce() + Send + 'static>;


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
        let reciever =Arc::new(Mutex::new(reciever));
        let mut workers= Vec::with_capacity(size);
        for id in 0..size{
         workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool{workers, sender}
    }
    //send transfer the closure from one thread to another and 'static
    //because we don't know how long it will take for the thread to execute
    pub fn execute<F>(&self, f:F) where F:FnOnce() + Send + 'static,
    {
        let job=Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id:usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    //fn new takes an id and returns a worker instance that holds the id and a thread spawned with empty closure
    fn new(id:usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>> )->Worker{
        let thread=thread::spawn(||{
            let job=reciever.lock().recv.unwrap();
            println!("Worker {id} got a job: process ongiong");
            job()
        });
        Worker{id, thread}
    }
}