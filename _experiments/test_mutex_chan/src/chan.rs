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
}

impl<T: 'static + Clone + Send + Sync> Chan<T> {
    
    pub fn new() -> Chan<T> {
        
        Chan {
            query: Query::new(),
        }
    }
    
    pub fn sender(&self) -> Sender<T> {
        Sender::new(self.query.clone())
    }
    
    pub fn receiver(&self) -> Receiver<T> {
        
        let outvalue               = Outvalue::new();
        let receiver : Receiver<T> = Receiver::new(outvalue.clone());
        
        
        let conv: Box<Fnconvert<T,T,T>> = Box::new(Fnconvert::new(Box::new(|argin: T| -> T {
            argin
        })));
        
        
        let transport = Transport {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            fnconvert : conv,
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


fn create_iden<A>() -> Box<Fn(A) -> A + Send + Sync + 'static> {
    Box::new(|argin: A| -> A {
        argin
    })
}


