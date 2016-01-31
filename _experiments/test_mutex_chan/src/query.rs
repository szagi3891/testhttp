use std::sync::{Arc, Mutex};
use transport::TransportIn;


pub struct Query<T> {
    pub values  : T,
    pub senders : Vec<Box<TransportIn<T>>>,
}

impl<T> Query<T> {
    
    pub fn new() -> Arc<Mutex<Query<T>>> {
        Arc::new(Mutex::new(Query {
            values  : Vec::new(),
            senders : Vec::new(),
        }))
    }
    
    
                        //wysyłanie
    
    pub fn sending(&mut self) {
        
        loop {
            
            match (self.senders.pop(), self.values.pop()) {
                
                (Some(sender), Some(value)) => {
                    sender.send(value);
                },
                
                (Some(sender), None) => {
                    self.senders.push(sender);
                }, 
                
                (None, Some(value)) => {
                    self.values.push(value);
                },
                
                (None, None) => {
                    return;
                }
            }
        }
    }
}