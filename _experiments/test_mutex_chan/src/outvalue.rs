use std::sync::{Arc, Mutex, Condvar};
use std::collections::linked_list::LinkedList;

use transport::TransportOut;


pub struct Outvalue<R> {
    pub mutex : Mutex<OutvalueInner<R>>,
    pub cond  : Condvar,
}

impl<R> Outvalue<R> {
    
    pub fn new() -> Arc<Outvalue<R>> {
        
        Arc::new(Outvalue {
            mutex : OutvalueInner::new(),
            cond  : Condvar::new(),
        })
    }
}


pub struct OutvalueInner<R> {
    pub value : Option<R>,
    pub list  : LinkedList<Box<TransportOut<R> + Send>>,
}

impl<R> OutvalueInner<R> {
    
    fn new() -> Mutex<OutvalueInner<R>> {
        
        Mutex::new(OutvalueInner{
            value : None,
            list  : LinkedList::new(),
        })
    }
}
