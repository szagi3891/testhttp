use std::sync::{Arc, Mutex};

use transport::TransportOut;

pub struct Outvalue<R> {
    pub value : Option<R>,
    pub list  : Vec<Box<TransportOut<R> + Send>>,
}

impl<R> Outvalue<R> {
    
    pub fn new() -> Arc<Mutex<Outvalue<R>>> {
        
        Arc::new(Mutex::new(Outvalue{
            value : None,
            list  : Vec::new(),
        }))
    }
}
