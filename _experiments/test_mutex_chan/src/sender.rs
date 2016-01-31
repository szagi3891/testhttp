use std::sync::{Arc, Mutex};

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
    
    fn send(&self, value: T) {
        
        let mut query_inner = self.query.lock().unwrap();
        
        query_inner.values.push(value);
        
        query_inner.sending();
    }
}