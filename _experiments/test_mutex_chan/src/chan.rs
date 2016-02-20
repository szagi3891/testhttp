use std::sync::{Arc, Mutex};

use sender::Sender;
use receiver::Receiver;
use query::Query;
use transport::Transport;
use transformer::Transformer;
use outvalue::Outvalue;
use fnconvert::Fnconvert;


//Sender
//Query
//Transport
//Valueout
//Receiver
//Select


pub struct Chan<T>
    where T : Clone + Send + Sync + 'static {
    
    query     : Arc<Mutex<Query<T>>>,
}

impl<T> Chan<T>
    where T : Clone + Send + Sync + 'static {
    
    pub fn new() -> Chan<T> {
        
        Chan {
            query: Query::new(),
        }
    }
    
    pub fn sender(&self) -> Sender<T> {
        Sender::new(self.query.clone())
    }
    
    pub fn receiver(&self) -> Receiver<T,T> {
        
        let outvalue = Outvalue::new();
        
        
        let transport = Transport {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            fnconvert : Fnconvert::<T,T,T>::new(create_iden::<T>()),
        };     
        
        let transformer = Transformer {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            fnconvert : Fnconvert::<T,T,T>::new(create_iden::<T>()),
        };

        
        let mut inner = outvalue.mutex.lock().unwrap();
        inner.list.push_back(Box::new(transport));
        
        
        Receiver::new(outvalue.clone(), transformer)
        //Receiver::new(outvalue.clone())
    }
    
    pub fn couple(&self) -> (Sender<T>, Receiver<T,T>) {
        
        (self.sender(), self.receiver())
    }
}


fn create_iden<A>() -> Box<Fn(A) -> A + Send + Sync + 'static>
    where A : Send + Sync + 'static {
    
    Box::new(|argin: A| -> A {
        argin
    })
}


pub struct Select<Out> {
    outvalue : Arc<Outvalue<Out>>,
}


impl<Out> Select<Out>
    where Out : Send + Sync + 'static {
    
    pub fn new() -> Select<Out> {
        
        Select {
            outvalue : Outvalue::new(),
        }
    }
    
    pub fn add<T,R>(&self, rec: Receiver<T,R>, transform: Box<Fn(R) -> Out + Send + Sync + 'static>)
        where
            T : Send + Sync + 'static ,
            R : Send + Sync + 'static {
        
        let new_transport = rec.transform(self.outvalue.clone(), transform);
        
        println!("dodaje reciviera");
    }
}


