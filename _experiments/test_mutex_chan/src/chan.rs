use std::sync::{Arc, Mutex};

use sender::Sender;
use receiver::Receiver;
use query::Query;
use transport::Transport;
//use transformer::Transformer;
use outvalue::Outvalue;
use fnconvert::Fnconvert;


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
        
        
        let transport = Transport {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            fnconvert : Fnconvert::<T,T,T>::new(create_iden::<T>()),
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


