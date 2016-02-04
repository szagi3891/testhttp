use std::sync::{Arc, Mutex};
use transport::TransportIn;
use std::collections::linked_list::LinkedList;


pub struct Query<T> {
    pub values  : LinkedList<Box<T>>,
    pub senders : LinkedList<Box<TransportIn<T> + Send>>,
}

impl<T> Query<T> {
    
    pub fn new() -> Arc<Mutex<Query<T>>> {
        Arc::new(Mutex::new(Query {
            values  : LinkedList::new(),
            senders : LinkedList::new(),
        }))
    }
        
    
    pub fn sending(&mut self) {

        loop {

            match (self.senders.pop_front(), self.values.pop_front()) {

                (Some(sender), Some(value)) => {

                                //przekazanie sendera do odbiorcy wraz z przekazywaną wartością
                    sender.send(value);
                },

                (Some(sender), None) => {
                    
                    self.senders.push_front(sender);
                    return;
                }, 

                (None, Some(value)) => {
                    
                    self.values.push_front(value);
                    return;
                },

                (None, None) => {
                    
                    return;
                }
            }
        }
    }
}