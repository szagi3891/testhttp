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
    
    pub fn send(&self, value: T) {
        
        let mut query_inner = self.query.lock().unwrap();
        
        query_inner.values.push_back(Box::new(value));
        
        query_inner.sending();
    }
}




/*
impl<R> Clone for Receiver<R> {
    
    fn clone(&self) -> Self {
    
    }

    fn clone_from(&mut self, source: &Self) { ... }
}
*/
