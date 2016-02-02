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
    
    pub fn get(&mut self) -> R {
        
        let mut guard = self.mutex.lock().unwrap();
        
        loop {
            
            let value = guard.value.take();
            
            match value {
                
                Some(value) => {
                    return value;
                }
                
                None => {
                    
                    println!("dalej pusta wartość w schowku, czekam dalej");
                }
            }
            
            guard = self.cond.wait(guard).unwrap();
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