use std::sync::{Arc, Mutex};

use sender::Sender;
use receiver::Receiver;
use query::Query;
use transport::Transport;
//use transformer::Transformer;
use outvalue::Outvalue;
use fnconvert::{Fnconvert, Convert};


//Sender
//Query
//Transport
//Valueout
//Receiver


pub struct Chan<T: 'static + Clone + Send> {
    query     : Arc<Mutex<Query<T>>>,
    fnconvert : Box<Convert<T,T>>
}

impl<T: 'static + Clone + Send + Sync> Chan<T> {
    
    pub fn new() -> Chan<T> {
        
        Chan {
            query     : Query::new(),
            fnconvert : Fnconvert::new(Box::new(|argin: T| -> T {
                argin
            }))
        }
    }
    
    pub fn sender(&self) -> Sender<T> {
        Sender::new(self.query.clone())
    }
    
    pub fn receiver(&self) -> Receiver<T> {
        
        let outvalue               = Outvalue::new();
        let receiver : Receiver<T> = Receiver::new(outvalue.clone());
        
        let transport = Transport {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            fnconvert : self.fnconvert.clone(),
        };
        
/* 
        let transformer = Transformer {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            transform : create_identity::<T>(),
        };
*/      
        
        let mut inner = outvalue.mutex.lock().unwrap();
        inner.list.push_back(Box::new(transport));
        //inner.transformers.push(Box::new(transformer));
        
        receiver
    }
    
    pub fn couple(&self) -> (Sender<T>, Receiver<T>) {
        
        (self.sender(), self.receiver())
    }
}


