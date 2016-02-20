use std::sync::{Arc, Mutex};

use sender::Sender;
use receiver::Receiver;
use query::Query;
use transport::Transport;
use transformer::Transformer;
use outvalue::Outvalue;
use fnconvert::{Fnconvert, Convert};


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
        
        
        let fnconvert: Box<Convert<T,T> + Send + 'static> = Fnconvert::<T,T,T>::new(Box::new(|argin: T| -> T {
            argin
        }));
        
        
        let transport = Transport {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            fnconvert : fnconvert.clone(),
        };     
        
        let transformer = Transformer {
            query     : self.query.clone(),
            outvalue  : outvalue.clone(),
            fnconvert : fnconvert,
        };

        
        let mut inner = outvalue.mutex.lock().unwrap();
        inner.list.push_back(Box::new(transport));
        
        
        Receiver::new(outvalue.clone(), transformer)
    }
    
    pub fn couple(&self) -> (Sender<T>, Receiver<T,T>) {
        
        (self.sender(), self.receiver())
    }
}


pub struct Select<Out> where Out : Clone + Send + Sync + 'static {
    outvalue : Arc<Outvalue<Out>>,
}


impl<Out> Select<Out>
    where Out : Clone + Send + Sync + 'static {
    
    pub fn new() -> Select<Out> {
        
        Select {
            outvalue : Outvalue::new(),
        }
    }
    
    pub fn add<T,R>(&self, rec: Receiver<T,R>, transform: Box<Fn(R) -> Out + Send + Sync + 'static>)
        where
            T : Clone + Send + Sync + 'static ,
            R : Send + Sync + 'static {
        
        let new_transport = rec.transform(self.outvalue.clone(), transform);
        
                        //dodaj transporter do nowego out
        {
            let mut inner = self.outvalue.mutex.lock().unwrap();
            inner.list.push_back(Box::new(new_transport));
        }
        
        
        
        println!("dodaje reciviera");
    }
    
    pub fn get(&self) -> Out {
        self.outvalue.get()
    }
}


