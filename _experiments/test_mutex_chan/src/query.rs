use std::sync::{Arc, Mutex};
use transport::TransportIn;


pub struct Query<T> {
    pub values  : Vec<Box<T>>,
    pub senders : Vec<Box<TransportIn<T>>>,
}

impl<T> Query<T> {
    
    pub fn new() -> Arc<Mutex<Query<T>>> {
        Arc::new(Mutex::new(Query {
            values  : Vec::new(),
            senders : Vec::new(),
        }))
    }
        
    
    pub fn sending(&mut self) {

        loop {

            match (self.senders.pop(), self.values.pop()) {

                (Some(mut sender), Some(value)) => {

                    //sender, value);
                    sender.send(value);
                    //zniszczenie referencji do sendera
                    
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
}