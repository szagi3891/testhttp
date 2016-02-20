//use types::ChannelValue;
use std::sync::{Arc, Mutex};
use query::Query;
use outvalue::Outvalue;
use transport::{Transport, TransportOut};
use fnconvert::{Fnconvert, Convert};


pub struct Transformer<T, R> {
    
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Outvalue<R>>,
    pub fnconvert : Box<Convert<T,R> + Send>,
}


impl<T, R> Transformer<T, R>
    where
        T : Send + Sync + 'static ,
        R : Send + Sync + 'static {
    
    pub fn transform<K>(self, outvalue: Arc<Outvalue<K>>, transform: Box<Fn(R) -> K + Send + Sync + 'static>) -> Transport<T,K>
        where K : 'static + Send + Sync {
        
            //TODO - niepotrzebne klonowanie query
        
        let transport = Transport {
            query     : self.query.clone(),
            outvalue  : outvalue,
            fnconvert : Box::new(Fnconvert::Next(self.fnconvert, Arc::new(transform))),
        };
        
        transport
    }
}



