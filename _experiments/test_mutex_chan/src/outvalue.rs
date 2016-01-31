use std::sync::{Arc, Mutex};

use transport::TransportOut;

pub struct Outvalue<R> {
    pub list  : Vec<Box<TransportOut<R>>>,
}

impl<R> Outvalue<R> {
    
    pub fn new() -> Arc<Mutex<Outvalue<R>>> {
        
        Arc::new(Mutex::new(Outvalue{
            list  : Vec::new(),
        }))
    }
}
