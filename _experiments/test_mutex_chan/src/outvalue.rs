use std::sync::{Arc, Mutex, Condvar};
use std::collections::linked_list::LinkedList;

use transport::TransportOut;


pub struct Outvalue<R> {
    pub mutex : Arc<Mutex<OutvalueInner<R>>>,
    pub cond  : Condvar,
}

impl<R> Outvalue<R> {
    
    pub fn new(outvalue_inner: Arc<Mutex<OutvalueInner<R>>>) -> Outvalue<R> {
        
        Outvalue {
            mutex : outvalue_inner,
            cond  : Condvar::new(),
        }
    }
}


pub struct OutvalueInner<R> {
    pub value : Option<R>,
    pub list  : LinkedList<Box<TransportOut<R> + Send>>,
}

impl<R> OutvalueInner<R> {
    
    pub fn new() -> Arc<Mutex<OutvalueInner<R>>> {
        
        Arc::new(Mutex::new(OutvalueInner{
            value : None,
            list  : LinkedList::new(),
        }))
    }
}
