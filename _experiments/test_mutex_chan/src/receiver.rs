use std::sync::{Arc, Mutex, Condvar};
use transport::TransportOut;
use outvalue::Outvalue;



pub struct Receiver<R> {
    pub mutex : Arc<Mutex<Outvalue<R>>>,
    cond  : Condvar,
}


impl<R> Receiver<R> {
    
    pub fn new(outvalue: Arc<Mutex<Outvalue<R>>>) -> Receiver<R> {
        Receiver{
            mutex : outvalue,
            cond  : Condvar::new(),
        }
    }
}

/*
impl<R> Clone for Receiver<R> {
    
    fn clone(&self) -> Self {
        
        
    }

    fn clone_from(&mut self, source: &Self) { ... }
}
*/