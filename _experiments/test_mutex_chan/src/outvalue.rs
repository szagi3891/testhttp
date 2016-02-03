use std::sync::{Arc, Mutex};
use std::collections::linked_list::LinkedList;

use transport::TransportOut;

pub struct Outvalue<R> {
    pub value : Option<R>,
    pub list  : LinkedList<Box<TransportOut<R> + Send>>,
}

impl<R> Outvalue<R> {
    
    pub fn new() -> Arc<Mutex<Outvalue<R>>> {
        
        Arc::new(Mutex::new(Outvalue{
            value : None,
            list  : LinkedList::new(),
        }))
    }
}
