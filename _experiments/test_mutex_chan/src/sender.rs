use std::sync::{Arc, Mutex, MutexGuard};
    
use query::Query;


pub struct Sender<T> {
    query : Arc<Mutex<Query<T>>>,
}

impl<T> Sender<T> {
    
    pub fn new(query: Arc<Mutex<Query<T>>>) -> Sender<T> {
        Sender {
            query : query
        }
    }
    
    pub fn send(&self, value: T) {
        
        let mut query_inner = self.query.lock().unwrap();
        
        query_inner.values.push(Box::new(value));
        
        //sending(query_inner);
        query_inner.sending();
    }
}

               

                        //wysyłanie
/*
    pub fn sending(&mut self) {
        
        loop {
            
            match (self.senders.pop(), self.values.pop()) {
                
                (Some(mut sender), Some(value)) => {
                    sender, value);
                    //sender.send_test();
                },
                
                (Some(sender), None) => {
                    self.senders.push(sender);
                    return;
                }, 
                
                (None, Some(value)) => {
                    self.values.push(value);
                    return;
                },
                
                (None, None) => {
                    return;
                }
            }
        }
    }
*/