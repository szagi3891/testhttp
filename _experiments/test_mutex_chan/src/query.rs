use std::sync::{Arc, Mutex};
use transport::TransportIn;


pub struct Query<T> {
    pub values  : Vec<T>,
    senders : Vec<Box<TransportIn<T>>>,
}

impl<T> Query<T> {
    
    pub fn new() -> Arc<Mutex<Query<T>>> {
        Arc::new(Mutex::new(Query {
            values  : Vec::new(),
            senders : Vec::new(),
        }))
    }
}