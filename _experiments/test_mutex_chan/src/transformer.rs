use std::sync::{Arc, Mutex};
use query::Query;
use outvalue::Outvalue;
use transport::{Transport, TransportOut};

/*
pub trait TransformerInterface<R> {
    fn transform<K>(self : Box<Self>, Box<Fn(R) -> K>) -> (TransportOut<K>, TransformerInterface<K>);
}
*/


pub struct Transformer<T, R> {
    
    pub query     : Arc<Mutex<Query<T>>>,
    pub outvalue  : Arc<Outvalue<R>>,
    pub transform : Box<Fn(T) -> R + 'static + Send>,
}


impl<T: 'static + Send + Sync, R: 'static + Send + Sync> Transformer<T, R> {
    
    fn transform<K>(self: Box<Self>, new_outvalue: &Arc<Outvalue<K>>, new_transform: &Fn() -> Box<Fn(R) -> K>) -> (Transport<T,K>, Transformer<T,K>) {
        
        let transport = Transport {
            query    : self.query.clone(),
            outvalue : new_outvalue.clone(),
            transform : glue::<T,R,K>(self.transform, new_transform()),
        };
        
        let transformer = Transformer {
            query     : self.query.clone(),
            outvalue  : new_outvalue.clone(),
            transform : glue::<T,R,K>(self.transform, new_transform()),
        };
        
        
        (transport, transformer)
    }
}


fn glue<T,R,K>(fn1: Box<Fn(T) -> R>, fn2: Box<Fn(R) -> K>) -> Box<Fn(T) -> K + 'static + Send> {
    
    Box::new(|arg: T| -> K {
        
        fn2(fn1(arg))
    })
}


