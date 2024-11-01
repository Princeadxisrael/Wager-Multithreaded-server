pub struct ThreadPool;

impl ThreadPool{
   
    pub fn new(size:usize)->ThreadPool{
         //fn will panic if size is zero
    assert!(size>0);
        ThreadPool
    }
    //send transfer the closure from one thread to another and 'static
    //because we don't know how long it will take for the thread to execute
    pub fn execute<F>(&self, f:F) where F:FnOnce() + Send + 'static,
    {}
}